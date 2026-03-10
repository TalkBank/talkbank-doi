# New machine setup

If there is a "rename" of an old machine to a new one, ssh has to be
fixed.

``` shellsession
$ ssh-keygen -f '/home/macw/.ssh/known_hosts' -R 'gandalf.lan.local.cmu.edu'
```
