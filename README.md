# Boulette - A terminal confirmation prompt.

Prevents you from accidentally shutting down remote hosts.

If you've ever finished a late night codding session 🥱
by typing `shutdown -h now` in the wrong terminal 😳.

## Usage

Prefix a critical command with `boulette` and a confirmation prompt will show up.

```sh
boulette "shutdown"
```

Create an alias to replace the command with the **bouletteproof** one.

```sh
# .bashrc/.zshrc or config.fish
alias shutdown = 'boulette shutdown'
```

and then safely use `shutdown`.

### Challenge types

In order to execute the provided command you can choose between some challenges to be resolved:

- **ask**, which is the default (`--challenge ask`). You have to type 'y' or 'n' to resume commande execution.

  ![boulette prompt](https://github.com/pipelight/boulette/blob/main/public/images/ask_challenge.png)

- **hostname**, enable with `--challenge hostname`. You must type the host name to resume command execution.

  ![boulette prompt](https://github.com/pipelight/boulette/blob/main/public/images/hostname_challenge.png)

- **numbers**, with `--challenge numbers` You must type a random 6 number sequence to resume command execution.

  ![boulette prompt](https://github.com/pipelight/boulette/blob/main/public/images/numbers_challenge.png)

### Over ssh only

Boulette confirmation prompt can be triggerd inside **ssh session only** thanks to the `--ssh-only` option.

When aliasing a command `<cmd>` with `boulette <cmd>`, typing `<cmd>` will execute transparently in a local terminal,
and will only raise a prompt when executed from inside an ssh session.

```sh
alias off='boulette "shutdown -h now"' --ssh-only
```

![boulette prompt](https://github.com/pipelight/boulette/blob/main/public/images/example_ssh.png)

### Write aliases

The idea is to enforce a prompt on your most dangerous commands.
We can do so by creating aliases of those commands and prefix them with boulette.

For example, setting the following alias, will prompt you whenever you type `shutdown -h now`.

`shutdown` becomes `boulette shutdown`.

Here are the ones I use the most frequently.

```sh
alias off='boulette "shutdown -h now" --ssh-only --challenge hostname'
alias sus='boulette "systemctl suspend" --ssh-only --challenge hostname'
alias reboot='boulette reboot --ssh-only --challenge hostname'
```

You can also prefix every shutdown commands wit boulette.
This way `shutdown` and `shutdown -h now` will both require confirmation.

- bash/zsh

```sh
shutdown () {
  boulette "shutdown $argv" --ssh-only --challenge hostname
}
```

- fish

```fish
function shutdown;
  boulette "shutdown $argv" --ssh-only --challenge hostname
end
```

## Install

with cargo:

```sh
cargo install --git https://github.com/pipelight/boulette

```

with nix(flakes):

```sh
nix-shell -p https://github.com/pipelight/boulette

```

## Help

You can display a usefull help message with minimal examples.

```sh
boulette --help
```

Greatly inspired by [Molly-guard](https://salsa.debian.org/debian/molly-guard).
