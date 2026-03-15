use anyhow::Result;
use caged::config::Config;
use caged::docker::DockerOrchestrator;
use caged::docker_runner::DockerCommandRunner;
use caged::dockerfile;
use mockall::mock;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

mock! {
    pub DockerRunner {}
    impl DockerCommandRunner for DockerRunner {
        fn build(&self, tag: &str, dockerfile_content: &str) -> Result<()>;
        fn run(&self, args: &[String]) -> Result<()>;
        fn ps_ancestor(&self, tag: &str) -> Result<String>;
        fn stop(&self, id: &str) -> Result<()>;
        fn rm(&self, id: &str) -> Result<()>;
        fn rmi(&self, tag: &str) -> Result<bool>;
        fn images_exists(&self, tag: &str) -> Result<bool>;
        fn check_version(&self) -> Result<()>;
    }
}

fn create_temp_config(dir: &TempDir) -> (PathBuf, Config) {
    let config_path = dir.path().join("caged.yaml");
    let yaml = "
agent: gemini
packages:
  - curl
docker: false
";
    fs::write(&config_path, yaml).unwrap();
    let config = Config::load(&config_path).unwrap();
    (config_path, config)
}

#[test]
fn test_build_image() {
    let temp_dir = TempDir::new().unwrap();
    let (config_path, config) = create_temp_config(&temp_dir);

    let mut mock_runner = MockDockerRunner::new();
    mock_runner.expect_check_version().returning(|| Ok(()));

    let project_dir = std::env::current_dir().unwrap();
    let expected_dockerfile = dockerfile::generate_dockerfile(&config, &project_dir).unwrap();

    mock_runner
        .expect_build()
        .withf(move |tag: &str, content: &str| {
            tag.starts_with("caged-agent-") && content == expected_dockerfile
        })
        .times(1)
        .returning(|_, _| Ok(()));

    let orchestrator = DockerOrchestrator::new(config_path, mock_runner).unwrap();

    assert!(orchestrator.build_image(&config).is_ok());
}

#[test]
fn test_run_agent() {
    let temp_dir = TempDir::new().unwrap();
    let (config_path, config) = create_temp_config(&temp_dir);

    let mut mock_runner = MockDockerRunner::new();
    mock_runner.expect_check_version().returning(|| Ok(()));

    mock_runner
        .expect_run()
        .withf(|args: &[String]| {
            args.contains(&"run".to_string()) && args.contains(&"gemini".to_string())
        })
        .times(1)
        .returning(|_| Ok(()));

    let orchestrator = DockerOrchestrator::new(config_path, mock_runner).unwrap();

    assert!(orchestrator.run_agent(&config).is_ok());
}

#[test]
fn test_shell() {
    let temp_dir = TempDir::new().unwrap();
    let (config_path, config) = create_temp_config(&temp_dir);

    let mut mock_runner = MockDockerRunner::new();
    mock_runner.expect_check_version().returning(|| Ok(()));

    mock_runner
        .expect_run()
        .withf(|args: &[String]| {
            args.contains(&"run".to_string()) && args.contains(&"/bin/bash".to_string())
        })
        .times(1)
        .returning(|_| Ok(()));

    let orchestrator = DockerOrchestrator::new(config_path, mock_runner).unwrap();

    assert!(orchestrator.shell(&config, vec![]).is_ok());
}

#[test]
fn test_cleanup() {
    let temp_dir = TempDir::new().unwrap();
    let (config_path, _) = create_temp_config(&temp_dir);

    let mut mock_runner = MockDockerRunner::new();
    mock_runner.expect_check_version().returning(|| Ok(()));

    mock_runner
        .expect_ps_ancestor()
        .times(1)
        .returning(|_| Ok("dummy_id_123\n".to_string()));

    mock_runner
        .expect_stop()
        .withf(|id: &str| id == "dummy_id_123")
        .times(1)
        .returning(|_| Ok(()));

    mock_runner
        .expect_rm()
        .withf(|id: &str| id == "dummy_id_123")
        .times(1)
        .returning(|_| Ok(()));

    mock_runner
        .expect_rmi()
        .withf(|tag: &str| tag.starts_with("caged-agent-"))
        .times(1)
        .returning(|_| Ok(true));

    let orchestrator = DockerOrchestrator::new(config_path, mock_runner).unwrap();

    assert!(orchestrator.cleanup().is_ok());
}
