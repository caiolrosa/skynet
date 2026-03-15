use anyhow::{anyhow, Context, Result};
use std::io::Write;
use std::process::{Command, Stdio};

pub trait DockerCommandRunner {
    fn build(&self, tag: &str, dockerfile_content: &str) -> Result<()>;
    fn run(&self, args: &[String]) -> Result<()>;
    fn ps_ancestor(&self, tag: &str) -> Result<String>;
    fn stop(&self, id: &str) -> Result<()>;
    fn rm(&self, id: &str) -> Result<()>;
    fn rmi(&self, tag: &str) -> Result<bool>;
    fn images_exists(&self, tag: &str) -> Result<bool>;
    fn check_version(&self) -> Result<()>;
}

pub struct DockerRunner;

impl DockerRunner {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DockerRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl DockerCommandRunner for DockerRunner {
    fn build(&self, tag: &str, dockerfile_content: &str) -> Result<()> {
        let mut child = Command::new("docker")
            .arg("build")
            .arg("-t")
            .arg(tag)
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

    fn run(&self, args: &[String]) -> Result<()> {
        let status = Command::new("docker")
            .args(args)
            .stdin(Stdio::inherit())
            .status()
            .context("Failed to execute docker command")?;

        if !status.success() {
            return Err(anyhow!("Docker command failed with status: {}", status));
        }

        Ok(())
    }

    fn ps_ancestor(&self, tag: &str) -> Result<String> {
        let output = Command::new("docker")
            .args(["ps", "-a", "-q", "--filter", &format!("ancestor={}", tag)])
            .output()
            .context("Failed to list containers for cleanup")?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    fn stop(&self, id: &str) -> Result<()> {
        Command::new("docker")
            .args(["stop", id])
            .status()
            .context(format!("Failed to stop container {}", id))?;
        Ok(())
    }

    fn rm(&self, id: &str) -> Result<()> {
        Command::new("docker")
            .args(["rm", id])
            .status()
            .context(format!("Failed to remove container {}", id))?;
        Ok(())
    }

    fn rmi(&self, tag: &str) -> Result<bool> {
        let status = Command::new("docker")
            .args(["rmi", tag])
            .status()
            .context("Failed to remove image")?;

        Ok(status.success())
    }

    fn images_exists(&self, tag: &str) -> Result<bool> {
        let output = Command::new("docker")
            .args(["images", "-q", tag])
            .output()
            .context("Failed to check if docker image exists")?;

        Ok(!output.stdout.is_empty())
    }

    fn check_version(&self) -> Result<()> {
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
