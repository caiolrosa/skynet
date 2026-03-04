# Caged

`caged` is a CLI tool designed for isolated agent execution. It simplifies the process of building and running Docker containers for AI agents (Claude or Gemini) with project-specific configurations.

## Features

- **Isolated Execution**: Runs agents in a secure, containerized environment.
- **Declarative Configuration**: Define your agent, packages, and environment in a `caged.yaml` file.
- **Tool Management**: Built-in support for `mise` to manage language runtimes and tools.
- **Docker-in-Docker**: Optional support for accessing the Docker socket from within the agent container.
- **Volume Mounting**: Easily mount host directories into the container. The current project directory is automatically mounted at its original path for seamless integration.

## Commands

- `caged run`: Build the image and execute the agent.
- `caged shell`: Build the image and open an interactive bash shell in the container.
- `caged build`: Force a rebuild of the project-specific agent image.
- `caged cleanup`: Stop and remove containers and images associated with the current project.

## Configuration (`caged.yaml`)

Create a `caged.yaml` file in your project root:

```yaml
agent: gemini # or claude
packages: # Optional: install extra os packages
  - build-essential
  - curl
mise: # Optional: install extra mise supported packages
  - node@20
  - rust@latest
volumes: # Optional: any additional volumes you want
  - /home/myuser/.gemini:/home/user/.gemini
  - /home/myuser/.claude:/home/user/.claude
docker: true # Optional: enable docker access
```

## Usage

```bash
# Run the agent using the local caged.yaml
caged run

# Start an interactive shell using the local caged.yaml
caged shell

# Specify a custom config file
caged -f custom-config.yaml run
```

## Requirements

- Docker
- Rust (if you want to build from source)
