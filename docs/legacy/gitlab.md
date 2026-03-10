# GitLab without Docker, EE with built-in Nginx

## Installation without Docker (had problems)

I found instructions at https://about.gitlab.com/install/#ubuntu and
followed them with modifications (using `gitlab-ce` instead of
`gitlab-ee`).

``` shellsession
$ sudo apt install ca-certificates curl openssh-server postfix
$ curl https://packages.gitlab.com/install/repositories/gitlab/gitlab-ee/script.deb.sh | sudo bash
$ sudo EXTERNAL_URL="https://gitlab.talkbank.org:8929" apt-get install gitlab-ee
```

## Configuration file

HTTPS setup: https://docs.gitlab.com/omnibus/settings/nginx.html#manually-configuring-https

https://docs.gitlab.com/omnibus/settings/ssl.html

does not work with non-standard ports

Edit configuration file `/etc/gitlab/gitlab.rb` to have our https
server name:

```
external_url 'https://gitlab.talkbank.org:8929'
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

Set time zone.
Set my email.

## Personal access token

``` shellsession
chen
_uWRiyy5KhxsnzwoxMLz
```

TODO
Add macw.

## Initial configuration

https://gitlab.com/gitlab-org/omnibus-gitlab/blob/master/README.md

### New sign-ups

Disable "Sign-up enabled"!! I made a mistake and did not and there are
thousands of accounts created.

Require admin approval.

https://docs.gitlab.com/ee/install/next_steps.html

## TODO Create a group TalkBank

https://gitlab.talkbank.org:8929/groups/new

### Group members

me
Brian MacWhinney <macw@andrew.cmu.edu>
John Kowalski <jkau@andrew.cmu.edu>
Leonid Spektor <spektor@andrew.cmu.edu>
Andrew Yankes <ayankes@andrew.cmu.edu>
Davida Fromm <fromm@andrew.cmu.edu>

## SSH key

https://gitlab.talkbank.org:8929/-/profile/keys

Note: stuff goes into `/var/opt/gitlab/.ssh/`

Go to individual profile Preferences at top right corner.
https://gitlab.talkbank.org:8929/-/profile/keys

Paste SSH keys for Franklin and Brian into `chen` admin.

Test it:

``` shellsession
$ ssh -T git@gitlab.talkbank.org
```

## Group Access token

``` shellsession
talkbank
iHBNzx75oyi-izLspbMA
```

## Deploy keys

https://docs.gitlab.com/ee/user/project/deploy_keys/

read-write

Public deploy key? Was not useful enough, so deleted.

``` shellsession
curl --header "PRIVATE-TOKEN: _uWRiyy5KhxsnzwoxMLz" "https://gitlab.talkbank.org:8929/api/v4/deploy_keys"
# Result: id 2
```

each project config must make use
Grant write permissions for each project.

TODO How automate?

## Deploy token

For all talkbank projects. Not sure whether to use.

``` shellsession
gitlab+deploy-token-1
JbD6K4Xiosy7SeD5HmBK
```

## Create repos from command line

https://docs.gitlab.com/ee/api/projects.html#create-project

### Note `main` instead of `master`

Empty project with talkbank owner. Need chen access though.

TODO not working

First, create with chen namespace.

``` shellsession
curl --header "PRIVATE-TOKEN: _uWRiyy5KhxsnzwoxMLz" \
--data-urlencode 'name=junk' \
-XPOST "https://gitlab.talkbank.org:8929/api/v4/projects"
```

Then transfer namespace using project ID.

``` shellsession
curl --request PUT --header "PRIVATE-TOKEN: _uWRiyy5KhxsnzwoxMLz" "https://gitlab.talkbank.org:8929/api/v4/projects/8/transfer?namespace=talkbank"
```

## Initial import

``` shellsession
git remote add origin git@gitlab.talkbank.org:talkbank/junk
git push --set-upstream origin main
```

Shows up at `https://gitlab.talkbank.org:8929/talkbank/junk`

## TODO Enable deploy key

https://docs.gitlab.com/ee/api/deploy_keys.html#enable-a-deploy-key

Need project ID or path, deploy ID.

``` shellsession
curl --request POST --header "PRIVATE-TOKEN: <your_access_token>" "https://gitlab.example.com/api/v4/projects/5/deploy_keys/13/enable"
```


## Clone

``` shellsession
git clone ssh://git@gitlab.talkbank.org/talkbank/junk.git
```

or

``` shellsession
git clone git@gitlab.talkbank.org:talkbank/junk.git
```

## TODO Prune history

Prune the history. Then create new repo.

``` shellsession
git rev-parse HEAD > .git/shallow
git gc --prune=now
FILTER_BRANCH_SQUELCH_WARNING=1 git filter-branch -- --all
```

``` shellsession
git branch -m master main
git remote set-url origin git@gitlab.talkbank.org:talkbank/junk-new
git push --set-upstream origin main
```

## Upgrades

https://gitlab-com.gitlab.io/support/toolbox/upgrade-path/

``` shellsession
sudo apt-get install -y gitlab-ee=15.11.13-ee.0
```

``` shellsession
2024-02-27 11:53:06 -0500 -- Warning: Your gitlab.rb and gitlab-secrets.json files contain sensitive data
and are not included in this backup. You will need these files to restore a backup.
Please back them up manually.

```

``` shellsession
There was an error running gitlab-ctl reconfigure:

redis_service[redis] (redis::enable line 19) had an error: RuntimeError: ruby_block[warn pending redis restart] (redis::enable line 77) had an error: RuntimeError: Execution of the command `/opt/gitlab/embedded/bin/redis-cli -s /var/opt/gitlab/redis/redis.socket INFO` failed with a non-zero exit code (1)
stdout:
stderr: Could not connect to Redis at /var/opt/gitlab/redis/redis.socket: No such file or directory



Running handlers complete
[2024-02-27T11:57:27-05:00] ERROR: Exception handlers complete
Infra Phase failed. 26 resources updated in 42 seconds
[2024-02-27T11:57:27-05:00] FATAL: Stacktrace dumped to /opt/gitlab/embedded/cookbooks/cache/cinc-stacktrace.out
[2024-02-27T11:57:27-05:00] FATAL: ---------------------------------------------------------------------------------------
[2024-02-27T11:57:27-05:00] FATAL: PLEASE PROVIDE THE CONTENTS OF THE stacktrace.out FILE (above) IF YOU FILE A BUG REPORT
[2024-02-27T11:57:27-05:00] FATAL: ---------------------------------------------------------------------------------------
[2024-02-27T11:57:27-05:00] FATAL: RuntimeError: redis_service[redis] (redis::enable line 19) had an error: RuntimeError: ruby_block[warn pending redis restart] (redis::enable line 77) had an error: RuntimeError: Execution of the command `/opt/gitlab/embedded/bin/redis-cli -s /var/opt/gitlab/redis/redis.socket INFO` failed with a non-zero exit code (1)
stdout:
stderr: Could not connect to Redis at /var/opt/gitlab/redis/redis.socket: No such file or directory


Running reconfigure: NOT OK
== Fatal error ==
Something went wrong during final reconfiguration, please check the output
== Reverting ==
ok: down: postgresql: 0s, normally up
Symlink correct version of binaries: OK
ok: run: postgresql: (pid 263905) 0s
== Reverted ==
== Reverted to 12.14. Please check output for what went wrong ==
Toggling deploy page:rm -f /opt/gitlab/embedded/service/gitlab-rails/public/index.html
Toggling deploy page: OK
Toggling services:ok: run: alertmanager: (pid 263919) 1s
ok: run: gitaly: (pid 2179) 99069s, want down, got TERM
ok: run: gitlab-exporter: (pid 263930) 0s
ok: run: gitlab-kas: (pid 263932) 1s
ok: run: grafana: (pid 263943) 0s
ok: run: logrotate: (pid 263945) 1s
ok: run: node-exporter: (pid 263959) 0s
ok: run: postgres-exporter: (pid 263966) 0s
ok: run: prometheus: (pid 263974) 1s
ok: run: redis-exporter: (pid 263976) 0s
ok: run: sidekiq: (pid 263992) 1s
Toggling services: OK
Checking if a newer PostgreSQL version is available and attempting automatic upgrade to it: NOT OK
Error ensuring PostgreSQL is updated. Please check the logs
dpkg: error processing package gitlab-ee (--configure):
 installed gitlab-ee package post-installation script subprocess returned error exit status 1
Errors were encountered while processing:
 gitlab-ee
E: Sub-process /usr/bin/dpkg returned an error code (1)
```

Oops, now installing redis:

``` shellsession
sudo apt-get install -y redis
```

Had to do the following in sequence:

``` shellsession
sudo apt-get install -y gitlab-ee=16.3.7-ee.0
sudo apt-get install -y gitlab-ee=16.7.6-ee.0
sudo apt-get install -y gitlab-ee=16.9.1-ee.0
```

TODO
``` shellsession
sudo apt-get install -y gitlab-ee=16.10.0-ee.0
```
