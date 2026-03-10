# Media server

We have a media server `media.talkbank.org` which is part of a
restricted CMU cloud.

## OS

It runs on Red Hat Linux.
TODO version

## Access

Ordinary access requires CMU VPN with 2fa.

TODO Who is allowed access? Franklin, Brian, John only?

## Configuration

TODO details

## Special permissions for access from [automated deployment server](automated-deployment.md)

TODO details: rsync, Apache

## Media file updates

We have no automated system for managing media file updates. Brian
uses Gandalf as a staging area to deploy updates to the `/data` on the
media server.

### Syncing media

`sync-media.sh` is a script used to sync Gandalf's own media file
structure with that of `media.talkbank.org`.

### Permission problems

In theory permissions should be set correctly by Brian on Gandalf
before syncing to `media.talkbank.org`, but he often forgets to set
the permissions properly, and the result is that the wrong permissions
get transferred to `media.talkbank.org` and people cannot access the
media, especially through [TalkBank browser](talkbank-browser.md).

`fix-media-folders.sh` is a workaround, to enable Brian to fix the
permissions after the fact, on `media.talkbank.org`.

## Permissions changed by Campus Cloud Plus admin

We have to undo changes:

Basically, I have to log into media:

``` shellsession
ssh macw@media.talkbank.org
sudo chmod 777 /etc/httpd/conf.d/
rm -rf ~/passwords/*
```

and then I have to do forced destroys and deploys. It's a horror show. For reference, the following is what I have to do, and it takes forever, and you shouldn't do it, but it's what I have to do:

``` shellsession
ssh macw@git.talkbank.org
rm -rf ~/staging/build/confs ~/staging/build/users/*
cd ~/staging/repos
for i in users *-data *-site; do ~/staging/scripts/deploy-1.py --repo $i --force; done
```
