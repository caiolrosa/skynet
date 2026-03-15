use crate::config::{Agent, Config};
use crate::docker_runner::DockerCommandRunner;
use crate::dockerfile;
use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::env::{self, home_dir};
use std::fs;
use std::path::PathBuf;

pub struct DockerOrchestrator<R: DockerCommandRunner> {
    project_dir: PathBuf,
    config_path: PathBuf,
    runner: R,
}

impl<R: DockerCommandRunner> DockerOrchestrator<R> {
    const IMAGE_TAG_PREFIX: &'static str = "caged-agent";

    pub fn new(config_path: PathBuf, runner: R) -> Result<Self> {
        let project_dir = env::current_dir().context("Failed to get current working directory")?;
        
        runner.check_version()?;
        
        let orchestrator = Self {
            project_dir,
            config_path,
            runner,
        };

        Ok(orchestrator)
    }

    pub fn build_image(&self, config: &Config) -> Result<()> {
        let dockerfile_content = dockerfile::generate_dockerfile(config, &self.project_dir)?;
        let tag = self.get_image_tag();

        self.runner.build(&tag, &dockerfile_content)
    }

    pub fn run_agent(&self, config: &Config) -> Result<()> {
        let agent_cmd = match config.agent {
            Agent::Claude => "claude",
            Agent::Gemini => "gemini",
        };

        self.execute_container(config, vec![agent_cmd.to_string()])
            .context("Failed to run agent container")
    }

    pub fn shell(&self, config: &Config, command: Vec<String>) -> Result<()> {
        let cmd = if command.is_empty() {
            vec!["/bin/bash".to_string()]
        } else {
            command
        };

        self.execute_container(config, cmd)
            .context("Failed to start shell container")
    }

    pub fn cleanup(&self) -> Result<()> {
        let tag = self.get_image_tag();
        println!("Cleaning up containers and image for: {}", tag);

        let container_ids = self.runner.ps_ancestor(&tag)?;

        for id in container_ids.lines() {
            if !id.is_empty() {
                println!("Stopping container: {}", id);
                if let Err(e) = self.runner.stop(id) {
                    println!("Warning: {}", e);
                }

                println!("Removing container: {}", id);
                if let Err(e) = self.runner.rm(id) {
                    println!("Warning: {}", e);
                }
            }
        }

        println!("Removing image: {}", tag);
        let success = self.runner.rmi(&tag)?;

        if !success {
            println!("Warning: Failed to remove image {}", tag);
        }

        Ok(())
    }

    pub fn image_exists(&self) -> Result<bool> {
        let tag = self.get_image_tag();
        self.runner.images_exists(&tag)
    }

    fn execute_container(
        &self,
        config: &Config,
        container_args: Vec<String>,
    ) -> Result<()> {
        let tag = self.get_image_tag();
        let mut args = vec!["run".to_string(), "-it".to_string()];
        args.extend(self.get_common_run_args(config)?);
        args.push(tag);
        args.extend(container_args);

        self.runner.run(&args)
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

                    path.replace("~", &dockerfile::get_container_home())
                })
                .collect::<Vec<String>>()
                .join(":");

            args.push(expanded_volume);
        }

        Ok(args)
    }

    fn get_security_args(&self, config: &Config) -> Result<Vec<String>> {
        // On mac docker socket is gid 0, which requires us to skip hardening flags
        let skip_hardening = config.docker && dockerfile::get_docker_socket_gid()? == 0;

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

        let gid = dockerfile::get_docker_socket_gid()?;
        println!("Warning: Mounting Docker socket. This has security implications.");

        Ok(vec![
            "-v".to_string(),
            format!("{}:{}", dockerfile::DOCKER_SOCKET, dockerfile::DOCKER_SOCKET),
            "--group-add".to_string(),
            gid.to_string(),
        ])
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
}
