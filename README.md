# Molly-breaks

Prevent from accidental host shutdown.

Use `molly-break` before a critical command and put this in your alias.

```sh
alias shutdown= "molly-break shutdown"
```

If you are in an ssh session,
the command `shutdown -h now` will display a warning and confirmation prompt.

Greatly inspired by [Molly-guard](https://salsa.debian.org/debian/molly-guard).
