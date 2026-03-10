# GitLab without Docker, using CE and Apache

## Installation without Docker (had problems)

I found instructions at https://about.gitlab.com/install/#ubuntu and
followed them with modifications (using `gitlab-ce` instead of
`gitlab-ee`).

``` shellsession
$ sudo apt install ca-certificates curl openssh-server postfix
$ curl https://packages.gitlab.com/install/repositories/gitlab/gitlab-ce/script.deb.sh | sudo bash
$ sudo EXTERNAL_URL="https://gitlab.talkbank.org:8929" apt-get install gitlab-ce
```

We didn't currently have any firewall stuff set up. I installed `ufw`
but chose not to configure it.

``` shellsession
$ sudo apt install ufw
$ sudo ufw status
Status: inactive
```

## TODO Configuration file

HTTPS setup: https://docs.gitlab.com/omnibus/settings/nginx.html#manually-configuring-https

Edit configuration file `/etc/gitlab/gitlab.rb` to have our https
server name:

```
external_url 'https://git.talkbank.org'
```

Disable `Let's Encrypt` since we already have our own certificates.

```
letsencrypt['enable'] = false
```

SSL in `/etc/gitlab/gitlab.rb`:

```
nginx['ssl_certificate'] = "/etc/ssl/certs/__talkbank_org_cert.cer"
nginx['ssl_certificate_key'] = "/etc/ssl/private/__talkbank_org.key"
```

Check:

``` shellsession
echo | /opt/gitlab/embedded/bin/openssl s_client -connect gitlab.talkbank.org:8929
```

Use Apache instead of Nginx:

https://docs.gitlab.com/omnibus/settings/nginx.html#using-a-non-bundled-web-server

```
nginx['enable'] = false
web_server['external_users'] = ['www-data']
```

Trusted proxies? Same machine.

```
gitlab_workhorse['listen_network'] = "tcp"
gitlab_workhorse['listen_addr'] = "127.0.0.1:8181"
```

```
gitlab_rails['allowed_hosts'] = ['gitlab.example.com']
```


Finally

``` shellsession
$ sudo gitlab-ctl reconfigure

Default admin account has been configured with following details:
Username: root
Password: You didn't opt-in to print initial root password to STDOUT.
Password stored to /etc/gitlab/initial_root_password. This file will be cleaned up in first reconfigure run after 24 hours.

NOTE: Because these credentials might be present in your log files in plain text, it is highly recommended to reset the password following https://docs.gitlab.com/ee/security/reset_user_password.html#reset-your-root-password.
```

``` shellsession
$ sudo gitlab-rake "gitlab:password:reset[root]"
```

Changed "Administrator" to "Franklin Chen" and "root" to "chen".

TODO
Add macw.

## Apache config setup

https://gitlab.com/gitlab-org/gitlab-recipes/-/tree/master/web-server/apache

https://gitlab.com/gitlab-org/gitlab-recipes/-/blob/master/web-server/apache/gitlab-omnibus-ssl-apache24.conf

TODO

Edit `/etc/apache2/sites-available/default-ssl.conf`.

## Initial configuration

https://gitlab.com/gitlab-org/omnibus-gitlab/blob/master/README.md

Administrator (`root`) password: the [usual](credentials.md) but
doubled to make more than 6 characters.

Set time zone.
Set my email.

https://docs.gitlab.com/ee/install/next_steps.html

## New sign-ups

Disable "Sign-up enabled"!! I made a mistake and did not and there are
thousands of accounts created.

Require admin approval.

## Create a group TalkBank

https://git.talkbank.org/groups/new

Private by default.

Default branch protection: change to "Not protected".

## Group members

me
Brian MacWhinney <macw@andrew.cmu.edu>
John Kowalski <jkau@andrew.cmu.edu>,
Leonid Spektor <spektor@andrew.cmu.edu>,
Andrew Yankes <ayankes@andrew.cmu.edu>,
Davida Fromm <fromm@andrew.cmu.edu>,

## SSH key

/var/opt/gitlab/.ssh/

Go to individual profile Preferences at top right corner.

Paste SSH key.

Test it:

``` shellsession
$ ssh -T git@git.talkbank.org
```

## Deploy keys

https://docs.gitlab.com/ee/user/project/deploy_keys/

read-write

public deploy keys
add macw

each project config must make use
Grant write permissions

## Create repos from command line

TODO

git@git.talkbank.org:talkbank/foo.git

## TODO `main` instead of `master`
