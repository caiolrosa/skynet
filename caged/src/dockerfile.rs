use crate::config::{Agent, Config};
use anyhow::{Result, anyhow};
use std::os::unix::fs::MetadataExt;
use std::path::Path;

pub const USER_NAME: &str = "agent";
pub const BASE_IMAGE: &str = "ubuntu:24.04";
pub const DOCKER_SOCKET: &str = "/var/run/docker.sock";

pub fn get_container_home() -> String {
    format!("/home/{}", USER_NAME)
}

pub fn generate_dockerfile(config: &Config, project_dir: &Path) -> Result<String> {
    let user_id = nix::unistd::getuid().as_raw();
    let group_id = nix::unistd::getgid().as_raw();

    let user_home = get_container_home();
    let user_setup = get_user_setup(user_id, group_id, &user_home);

    let docker_group_setup = if config.docker {
        let gid = get_docker_socket_gid()?;
        format!(
            "RUN groupadd -g {gid} docker_host || true && usermod -aG {gid} {user_name} || true",
            gid = gid,
            user_name = USER_NAME
        )
    } else {
        String::default()
    };

    let docker_apt_setup = if config.docker {
        get_docker_configuration()
    } else {
        "true".to_string()
    };

    let packages = build_package_list(config).join(" ");
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

    let project_dir_str = project_dir.to_string_lossy();
    let dockerfile = format!(
        include_str!("dockerfile.template"),
        base_image = BASE_IMAGE,
        user_name = USER_NAME,
        user_home = user_home,
        user_setup = user_setup,
        project_dir = project_dir_str,
        packages = packages,
        docker_apt_setup = docker_apt_setup,
        mise_commands = mise_commands,
        agent_install_cmd = agent_install_cmd,
        docker_group_setup = docker_group_setup,
    );

    Ok(dockerfile)
}

pub fn get_docker_socket_gid() -> Result<u32> {
    if !Path::new(DOCKER_SOCKET).exists() {
        return Err(anyhow!(
            "Docker socket not found at {}. Ensure Docker is running.",
            DOCKER_SOCKET
        ));
    }

    if cfg!(target_os = "macos") {
        return Ok(0);
    }

    Ok(Path::new(DOCKER_SOCKET).metadata()?.gid())
}

fn get_user_setup(user_id: u32, group_id: u32, user_home: &str) -> String {
    if user_id == 1000 && group_id == 1000 {
        format!(
            "RUN groupmod -n {user_name} ubuntu && usermod -l {user_name} -m -d {user_home} ubuntu",
            user_name = USER_NAME,
            user_home = user_home
        )
    } else {
        format!(
            "RUN groupadd -g {group_id} {user_name} || true && \\
    useradd -u {user_id} -g {group_id} -m -d {user_home} {user_name}",
            group_id = group_id,
            user_id = user_id,
            user_name = USER_NAME,
            user_home = user_home
        )
    }
}

fn get_docker_configuration() -> String {
    let docker_setup_cmds = [
        "install -m 0755 -d /etc/apt/keyrings",
        "curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc",
        "chmod a+r /etc/apt/keyrings/docker.asc",
        "echo \"deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu $(. /etc/os-release && echo \"$VERSION_CODENAME\") stable\" | tee /etc/apt/sources.list.d/docker.list > /dev/null",
        "apt-get update -y && apt-get install -y docker-ce-cli docker-compose-plugin",
    ];

    docker_setup_cmds.join(" && \\\n  ")
}

fn get_default_packages() -> Vec<String> {
    vec![
        "curl".to_string(),
        "ca-certificates".to_string(),
        "git".to_string(),
        "zstd".to_string(),
    ]
}

fn build_package_list(config: &Config) -> Vec<String> {
    let mut packages = get_default_packages();
    packages.extend(config.packages.clone());
    packages
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Agent;

    #[test]
    fn test_generate_dockerfile_gemini() {
        let config = Config {
            agent: Agent::Gemini,
            packages: vec!["htop".to_string()],
            mise: vec!["python@3.11".to_string()],
            volumes: vec![],
            docker: false,
        };
        let project_dir = Path::new("/my/project");

        let dockerfile = generate_dockerfile(&config, project_dir).unwrap();

        assert!(dockerfile.contains("FROM ubuntu:24.04"));
        assert!(dockerfile.contains("npm install -g @google/gemini-cli"));
        assert!(dockerfile.contains("apt-get install -y curl ca-certificates git zstd htop"));
        assert!(dockerfile.contains("mise use -g python@3.11"));
        assert!(dockerfile.contains("WORKDIR /my/project"));
    }

    #[test]
    fn test_generate_dockerfile_claude() {
        let config = Config {
            agent: Agent::Claude,
            packages: vec![],
            mise: vec![],
            volumes: vec![],
            docker: false,
        };
        let project_dir = Path::new("/another/project");

        let dockerfile = generate_dockerfile(&config, project_dir).unwrap();

        assert!(dockerfile.contains("curl -fsSL https://claude.ai/install.sh | bash"));
        assert!(dockerfile.contains("WORKDIR /another/project"));
    }
}
