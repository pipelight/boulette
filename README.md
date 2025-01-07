# Boulette

Prevent from accidental host shutdown.

Use `boulette` before a critical command and put this in your alias.

```sh
alias off='boulette "shutdown -h now"'
```

![boulette prompt](https://github.com/pipelight/boulette/blob/main/public/images/boulette_hostname_challenge.png)

Here are the ones I use the most frequently

```sh
alias off='boulette "shutdown -h now" --ssh-only --challenge hostname'
alias sus='boulette "systemctl suspend" --ssh-only --challenge hostname'
alias reboot='boulette reboot --ssh-only --challenge hostname'
```

You can display a usefull help message with minimal examples.

```sh
boulette --help
```

If you are in an ssh session,
the command `shutdown -h now` will display a warning and confirmation prompt.

Greatly inspired by [Molly-guard](https://salsa.debian.org/debian/molly-guard).
