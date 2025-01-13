# Boulette - A terminal confirmation prompt.

_It's late._ 🥱

_You finish your night coding session by typing `shutdown -h now` in a terminal._

_But nothing happens._

_Because it's the wrong terminal._

_And suddenly your production server is unreachable._

<img src="https://github.com/pipelight/boulette/blob/main/public/images/oh_la_boulette.jpg" width="200">

Protect you from yourself.
Hop on the boulette train!

**Boulette prevents you from accidentally damaging remote hosts**
by raising a warning prompt on dangerous commands.
The prompt simply asks for user confirmation,
and can also enforce a challenge resolution to decide whether to resume(or abort) the command.

## Usage

Prefix a critical command with `boulette` and a confirmation prompt will show up.

As an example we will use the `shutdown` command
however every command can be **bouletteproofed**.

```sh
boulette "shutdown -h now"
```

Create an alias to replace the command with the **bouletteproof** one.
See the [Write aliases](#write-aliases) section.

And then safely use `shutdown` 😌.

![boulette prompt](https://github.com/pipelight/boulette/blob/main/public/images/example_shutdown.png)

### Challenge types

In order to execute the provided command you can choose between some challenges to be resolved:

- **ask**, which is the default (`--challenge ask`). You have to type 'y' or 'n' to resume commande execution.

  ![boulette prompt](https://github.com/pipelight/boulette/blob/main/public/images/ask_challenge.png)

- **hostname**, enable with `--challenge hostname`. You must type the host name to resume command execution.

  ![boulette prompt](https://github.com/pipelight/boulette/blob/main/public/images/hostname_challenge.png)

- **numbers**, with `--challenge numbers` You must type a random 6 number sequence to resume command execution.

  ![boulette prompt](https://github.com/pipelight/boulette/blob/main/public/images/numbers_challenge.png)

- **characters**, with `--challenge chars` You must type a random 6 character string (Lower case 'a' to 'z' [a-z]) to resume command execution.

  ![boulette prompt](https://github.com/pipelight/boulette/blob/main/public/images/chars_challenge.png)

### Over ssh only

Boulette confirmation prompt can be triggered inside **ssh session only** thanks to the `--ssh-only` option.

When aliasing a command `<cmd>` with `boulette <cmd>`, typing `<cmd>` will execute transparently in a local terminal,
and will only raise a prompt when executed from inside an ssh session.

```sh
alias off='boulette "shutdown -h now"' --ssh-only
```

![boulette prompt](https://github.com/pipelight/boulette/blob/main/public/images/example_ssh.png)

### Write aliases

The idea is to enforce a prompt on your most dangerous commands.
We can do so by creating aliases of those commands
and **prefixing them with boulette**.

#### Single command alias

For example, setting the following alias,

```sh
alias off='boulette "shutdown -h now"' --ssh-only
```

will prompt you whenever you type `shutdown -h now`.

Here are the one-liners I use the most frequently.

```sh
alias off='boulette "shutdown -h now" --ssh-only --challenge hostname'
alias sus='boulette "systemctl suspend" --ssh-only --challenge hostname'
```

#### Mutliple command alias

You can also enable boulette on a command and its every subcommands.

Let's say you want to protect yourself from `shutdown` command ant its
every options.
This way `shutdown -r`, `shutdown -h now` and others will also raise a warning prompt.

Create a shell function to wrap the command call.

- for bash and zsh shells

```sh
# bash
shutdown () {
  boulette "shutdown $@" --ssh-only --challenge hostname
}
```

- for fish shell

```fish
# fish
function shutdown;
  boulette "shutdown $argv" --ssh-only --challenge hostname
end
```

#### Safeguard sudo

If you really are reckless and scroll, eye shuts,
through your shell history.
You are more likely to pase a command prefixed with `sudo`.

The following alias is a safeguar for the `sudo <cmd>`
version of your dangerous command.

```sh
# bash
sudo () {
  args="$*"
  if [[ $args =~ ^(shutdown|reboot).* ]]; then
    cmd='boulette "sudo $args" --ssh-only --challenge hostname'
    eval $cmd
  else
    cmd='$SHELL -c "sudo $args"'
    eval $cmd
  fi
}
```

```fish
# fish
function sudo
  set args "$argv"
  set -l res $(string match -r "^(shutdown|reboot).*" $args)
  # If there is a match
  if set -q res[1]
    command boulette "sudo $args" --ssh-only --challenge hostname
  else
    command sudo $argv
  end
end
```

## Install

### Cargo:

```sh
cargo install --git https://github.com/pipelight/boulette

```

### Try in a nix shell:

```sh
nix-shell -p https://github.com/pipelight/boulette

```

<details close>
<summary><h3> Nixos Module (Flakes) </h3></summary>

Add the flake url to your inputs.

```nix
inputs.boulette.url = "github:pipelight/boulette";
```

```nix
imports = [
  inputs.boulette.nixosModules.default
  # or
  inputs.boulette.hmModules.default
];
```

Tweak the following options to your needs.

```nix
# default.nix AND/OR home.nix

services.boulette = {
  enable = true; # Will enable and install `boulette` to your path.
  enableZsh = true; # Optional: Will add guards for `shutdown` and `reboot` commands to your `zsh` interactive shell sessions.
  enableBash = true; # Optional: Will add guards for `shutdown` and `reboot` commands to your `bash` interactive shell sessions.
  enableFish = true; # Optional: Will add guards for `shutdown` and `reboot` commands to your `fish` interactive shell sessions.
  enableSudoWrapper = true; # Optional
  commands = ["shutdown" "reboot"]; # Optional
  challengeType = "hostname"; # Optional: Defaults to hostname. One of "ask" "hostname", or "numbers".
  sshOnly = true # Boolean, default is`true`. Optional: Boulette confirmation prompts will be triggerd inside ssh session only. Only effects the enable{zsh,bash,fish} options.
};
```

</details>

## Help

You can display a usefull help message with minimal examples.

```sh
boulette --help
```

Greatly inspired by [Molly-guard](https://salsa.debian.org/debian/molly-guard).
