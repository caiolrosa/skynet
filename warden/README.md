# Warden

`warden` wraps `limactl` behind five short commands. Bringing a sandbox VM up is
one line instead of a long `limactl create` followed by a separate
`limactl start`. Past `build`'s two required flags, warden validates nothing and
passes arguments it doesn't own to `limactl` untouched, so every error you see
is `limactl`'s own.

## Commands

- `warden build`: Create an instance from a template and start it.
- `warden run`: Shell into an instance, starting it first if it's down.
- `warden stop`: Stop an instance.
- `warden ls`: List instances.
- `warden rm`: Delete an instance.

## Usage

```bash
# Create a sandbox from a template, mounting a directory writable
warden build -n sandbox -c ./warden/agent.yaml -m ~/Documents:w

# Get an interactive shell (starts the instance if it's stopped)
warden run sandbox

# Run a command in the guest — warden passes everything after the name through
warden run sandbox claude --resume

# Stop it when you're done with it
warden stop sandbox

# List every instance, or ask Lima for JSON
warden ls
warden ls --json

# Delete a stopped instance
warden rm sandbox
```

`build` requires `-n` and `-c`. `-m` is optional and takes one mount per flag,
but it replaces Lima's default mounts rather than adding to them, so list every
path you need. The template this repo ships lives at `warden/agent.yaml`.

## `run` leaves the VM running

Exiting the shell doesn't stop the instance. That's intended — a session that
ends shouldn't tear down a machine another session is using. Stop it with
`warden stop <name>`. Until you do, the instance holds its 4 CPUs and 4 GiB.

## Requirements

- [Lima](https://lima-vm.io) 2.0 or newer, with `limactl` on your `PATH`.

## Install

```bash
ln -s "$PWD/warden/warden" ~/.local/bin/warden
```
