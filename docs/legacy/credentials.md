# Credentials

## Shared accounts

Most of our shared accounts use a particular password.

## SSH keys

We currently use Brian's old `id_rsa` and `id_rsa.pub`.

TODO update to be more secure.

Also, for GitLab runner via shell, we set up `id_ed25519` with no passphrase.

### Authorized keys

``` shellsession
cat .ssh/id_rsa.pub >> .ssh/authorized_keys
cat .ssh/id_ed25519.pub >> .ssh/authorized_keys
```

Also, we authorized Franklin's key.

### TODO Startup hack

For `git.talkbank.org` and `homebank.talkbank.org`.

In `.bashrc`:

``` shellsession
eval `keychain --quiet --agents ssh --eval id_rsa id_ed25519 rsync1-key rsync2-key apache-key`
```
