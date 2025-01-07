# Boulette

Prevent from accidental host shutdown.

Use `boulette` before a critical command and put this in your alias.

```sh
alias shutdown= "boulette shutdown"
```

If you are in an ssh session,
the command `shutdown -h now` will display a warning and confirmation prompt.

Greatly inspired by [Molly-guard](https://salsa.debian.org/debian/molly-guard).
