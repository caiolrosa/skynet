use crate::config::{Agent, Config};
use anyhow::{Context, Result, anyhow};
use sha2::{Digest, Sha256};
use std::env::{self, home_dir};
use std::fs;
use std::io::Write;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub struct DockerOrchestrator {
    project_dir: PathBuf,
    config_path: PathBuf,
}

impl DockerOrchestrator {
    const DOCKER_SOCKET: &'static str = "/var/run/docker.sock";
    const BASE_IMAGE: &'static str = "ubuntu:26.04";
    const IMAGE_TAG_PREFIX: &'static str = "caged-agent";
    const USER_NAME: &'static str = "agent";

    pub fn new(config_path: PathBuf) -> Result<Self> {
        let project_dir = env::current_dir().context("Failed to get current working directory")?;
        let orchestrator = Self {
            project_dir,
            config_path,
        };
        orchestrator.check_docker()?;

        Ok(orchestrator)
    }

    pub fn build_image(&self, config: &Config) -> Result<()> {
        let dockerfile_content = self.generate_dockerfile(config)?;
        let tag = self.get_image_tag();

        let mut child = Command::new("docker")
            .arg("build")
            .arg("-t")
            .arg(&tag)
            .arg("-")
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .context("Failed to spawn docker build command")?;

        let mut stdin = child
            .stdin
            .take()
            .ok_or_else(|| anyhow!("Failed to open stdin for docker build"))?;
        stdin.write_all(dockerfile_content.as_bytes())?;
        drop(stdin);

        let status = child.wait().context("Failed to wait for docker build")?;
        if !status.success() {
            return Err(anyhow!("Docker build failed with status: {}", status));
        }

        Ok(())
    }

    pub fn run_agent(&self, config: &Config) -> Result<()> {
        let agent_cmd = match config.agent {
            Agent::Claude => "claude",
            Agent::Gemini => "gemini",
        };

        self.execute_container(config, vec![agent_cmd.to_string()], "run agent container")
    }

    pub fn shell(&self, config: &Config, command: Vec<String>) -> Result<()> {
        let cmd = if command.is_empty() {
            vec!["/bin/bash".to_string()]
        } else {
            command
        };

        self.execute_container(config, cmd, "start shell container")
    }

    pub fn cleanup(&self) -> Result<()> {
        let tag = self.get_image_tag();
        println!("Cleaning up containers and image for: {}", tag);

        let output = Command::new("docker")
            .args(["ps", "-a", "-q", "--filter", &format!("ancestor={}", tag)])
            .output()
            .context("Failed to list containers for cleanup")?;

        let container_ids = String::from_utf8_lossy(&output.stdout);
        for id in container_ids.lines() {
            if !id.is_empty() {
                println!("Stopping container: {}", id);
                Command::new("docker")
                    .args(["stop", id])
                    .status()
                    .context(format!("Failed to stop container {}", id))?;

                println!("Removing container: {}", id);
                Command::new("docker")
                    .args(["rm", id])
                    .status()
                    .context(format!("Failed to remove container {}", id))?;
            }
        }

        println!("Removing image: {}", tag);
        let status = Command::new("docker")
            .args(["rmi", &tag])
            .status()
            .context("Failed to remove image")?;

        if !status.success() {
            println!("Warning: Failed to remove image {}", tag);
        }

        Ok(())
    }

    pub fn image_exists(&self) -> Result<bool> {
        let tag = self.get_image_tag();
        let output = Command::new("docker")
            .args(["images", "-q", &tag])
            .output()
            .context("Failed to check if docker image exists")?;

        Ok(!output.stdout.is_empty())
    }

    fn generate_dockerfile(&self, config: &Config) -> Result<String> {
        let user_id = nix::unistd::getuid().as_raw();
        let group_id = nix::unistd::getgid().as_raw();

        let docker_group_setup = if config.docker {
            let gid = self.get_docker_socket_gid()?;
            format!(
                "RUN groupadd -g {gid} docker_host || true && usermod -aG {gid} {user_name} || true",
                gid = gid,
                user_name = Self::USER_NAME
            )
        } else {
            String::default()
        };

        let docker_apt_setup = if config.docker {
            self.get_docker_configuration()
        } else {
            "true".to_string()
        };

        let packages = self.build_package_list(config).join(" ");
        let mise_commands = if !config.mise.is_empty() {
            config
                .mise
                .iter()
                .map(|tool| format!("mise use -g {}", tool))
                .collect::<Vec<String>>()
                .join(" && ")
        } else {
            "true".to_string()
        };

        let agent_install_cmd = match config.agent {
            Agent::Claude => "curl -fsSL https://claude.ai/install.sh | bash",
            Agent::Gemini => "npm install -g @google/gemini-cli",
        };

        let project_dir = self.project_dir.to_string_lossy();
        let dockerfile = format!(
            include_str!("dockerfile.template"),
            base_image = Self::BASE_IMAGE,
            user_name = Self::USER_NAME,
            user_home = self.get_container_home(),
            group_id = group_id,
            user_id = user_id,
            project_dir = project_dir,
            packages = packages,
            docker_apt_setup = docker_apt_setup,
            mise_commands = mise_commands,
            agent_install_cmd = agent_install_cmd,
            docker_group_setup = docker_group_setup,
        );

        Ok(dockerfile)
    }

    fn execute_container(
        &self,
        config: &Config,
        container_args: Vec<String>,
        error_context: &str,
    ) -> Result<()> {
        let tag = self.get_image_tag();
        let mut args = vec!["run".to_string(), "-it".to_string()];
        args.extend(self.get_common_run_args(config)?);
        args.push(tag);
        args.extend(container_args);

        let status = Command::new("docker")
            .args(args)
            .stdin(Stdio::inherit())
            .status()
            .with_context(|| format!("Failed to {}", error_context))?;

        if !status.success() {
            return Err(anyhow!("{} failed with status: {}", error_context, status));
        }

        Ok(())
    }

    fn get_common_run_args(&self, config: &Config) -> Result<Vec<String>> {
        let project_path = self.project_dir.to_string_lossy().to_string();
        let mut args = vec!["--rm".to_string()];

        args.extend(self.get_security_args(config)?);
        args.extend(self.get_docker_socket_args(config)?);

        args.extend([
            "-v".to_string(),
            format!("{}:{}:rw", project_path, project_path),
            "-w".to_string(),
            project_path,
        ]);

        for vol in &config.volumes {
            args.push("-v".to_string());

            let expanded_volume: String = vol
                .split(':')
                .enumerate()
                .map(|(index, path)| {
                    if index == 0 {
                        let user_home = home_dir().expect("Failed to get user home directory");
                        return path.replace("~", &user_home.to_string_lossy());
                    }

                    path.replace("~", &self.get_container_home())
                })
                .collect::<Vec<String>>()
                .join(":");

            args.push(expanded_volume);
        }

        Ok(args)
    }

    fn get_security_args(&self, config: &Config) -> Result<Vec<String>> {
        // On mac docker socket is gid 0, which requires us to skip hardening flags
        let skip_hardening = config.docker && self.get_docker_socket_gid()? == 0;

        if skip_hardening {
            return Ok(Vec::new());
        }

        Ok(vec![
            "--security-opt".to_string(),
            "no-new-privileges:true".to_string(),
            "--cap-drop".to_string(),
            "ALL".to_string(),
        ])
    }

    fn get_docker_socket_args(&self, config: &Config) -> Result<Vec<String>> {
        if !config.docker {
            return Ok(Vec::new());
        }

        let gid = self.get_docker_socket_gid()?;
        println!("Warning: Mounting Docker socket. This has security implications.");

        Ok(vec![
            "-v".to_string(),
            format!("{}:{}", Self::DOCKER_SOCKET, Self::DOCKER_SOCKET),
            "--group-add".to_string(),
            gid.to_string(),
        ])
    }

    fn get_docker_configuration(&self) -> String {
        let docker_setup_cmds = [
            "install -m 0755 -d /etc/apt/keyrings",
            "curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc",
            "chmod a+r /etc/apt/keyrings/docker.asc",
            "echo \"deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu $(. /etc/os-release && echo \"$VERSION_CODENAME\") stable\" | tee /etc/apt/sources.list.d/docker.list > /dev/null",
            "apt-get update -y && apt-get install -y docker-ce-cli docker-compose-plugin",
        ];

        docker_setup_cmds.join(" && \\\n  ")
    }

    fn get_default_packages(&self) -> Vec<String> {
        vec![
            "curl".to_string(),
            "ca-certificates".to_string(),
            "git".to_string(),
            "zstd".to_string(),
        ]
    }

    fn build_package_list(&self, config: &Config) -> Vec<String> {
        let mut packages = self.get_default_packages();

        packages.extend(config.packages.clone());

        packages
    }

    fn get_config_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let canonical_path =
            fs::canonicalize(&self.config_path).unwrap_or(self.config_path.clone());
        hasher.update(canonical_path.to_string_lossy().as_bytes());

        let hash = hasher.finalize();
        hex::encode(hash)[..12].to_string()
    }

    fn get_image_tag(&self) -> String {
        format!("{}-{}", Self::IMAGE_TAG_PREFIX, self.get_config_hash())
    }

    fn get_docker_socket_gid(&self) -> Result<u32> {
        if !Path::new(Self::DOCKER_SOCKET).exists() {
            return Err(anyhow!(
                "Docker socket not found at {}. Ensure Docker is running.",
                Self::DOCKER_SOCKET
            ));
        }

        if cfg!(target_os = "macos") {
            return Ok(0);
        }

        Ok(Path::new(Self::DOCKER_SOCKET).metadata()?.gid())
    }

    fn get_container_home(&self) -> String {
        format!("/home/{}", Self::USER_NAME)
    }

    fn check_docker(&self) -> Result<()> {
        let status = Command::new("docker")
            .arg("version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .context("Docker CLI not found. Ensure Docker is installed and in your PATH.")?;

        if !status.success() {
            return Err(anyhow!(
                "Docker is installed but not functioning correctly."
            ));
        }

        Ok(())
    }
}
