# Botrun

Run AI coding agents (Claude, Gemini) in isolated Docker containers with a reproducible dev environment.

## Requirements

- Docker
- [yq](https://github.com/mikefarah/yq)

## Quick start

```sh
botrun init        # creates .botrun.yaml in current directory
botrun build       # builds the Docker image
botrun run         # runs the agent in a container
```

## Config

The `.botrun.yaml` file controls the environment:

```yaml
# Required — supported: claude, gemini
agent: claude

# Optional — apt packages to install
packages:
  - git
  - curl
  - build-essential

# Optional — runtimes/tools installed via mise (https://mise.jdx.dev)
mise:
  - node@22
  - python@3.12

# Optional — host:container volume mounts (~ expands to home dir)
volumes:
  - ~/.claude:~/.claude
  - ~/.config/claude:~/.config/claude

# Optional — mount the host Docker socket into the container
docker: true
```

Use `-f <path>` to specify a config file other than `.botrun.yaml`:

```sh
botrun -f custom.yaml build
botrun -f custom.yaml run
```

## What it does

1. **init** — Generates a starter `.botrun.yaml` with commented examples.
2. **build** — Generates a Dockerfile (Ubuntu 24.04 base, mise for runtimes, agent CLI) and builds a `botrun:<agent>` image.
3. **run** — Starts an interactive container that mounts your current directory, configured volumes, and optionally the Docker socket. Cleans up the container on exit.
