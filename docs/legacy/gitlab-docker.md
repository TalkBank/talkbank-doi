# GitLab

[GitLab](https://gitlab.com/) has positioned itself as a competitor to
[GitHub](github.md).

## Self-hosted community edition

https://about.gitlab.com/install/ce-or-ee/

For our purposes, GitLab is interesting because, unlike GitHub, they
provide an open source community edition for [self-hosted
installation](https://about.gitlab.com/install/).

I had intended to use GitLab for self-hosting [our own Git
server](git-server.md) several years ago, but at the time, our
[servers](servers.md) were not running Linux, so we went with the
[GitBucket](gitbucket.md) instead.

## Installation with Docker

https://hub.docker.com/r/gitlab/gitlab-ee/

``` shellsession
sudo docker pull gitlab/gitlab-ee
```

``` shellsession
export GITLAB_HOME=/srv/gitlab
```

Use our `gitlab` virtual host name.

Map ports not to conflict with our monolithic Apache installation: `hostPort:containerPort`.

Use Docker Compose with `docker-compose.yml`: https://docs.gitlab.com/ee/install/docker.html#install-gitlab-using-docker-compose

``` shellsession
sudo docker-compose up -d
```

`setup_web_1` created. TODO better name.

``` shellsession
sudo docker logs -f setup_web_1
```

TODO Error

``` shellsession
        Error executing action `run` on resource 'ruby_block[create certificate for gitlab.talkbank.org]'

        Invalid response from http://gitlab.talkbank.org/.well-known/acme-challenge/qkRNabaJmh4QhCHFOfl8c26VtThpUqOO-ZSI7rQH3iI
```

## TODO Test on Mac


map away from 22

``` shellsession
docker-compose logs
```

Near beginning:
``` shellsession
setup-web-1  | /opt/gitlab/embedded/bin/runsvdir-start: line 24: ulimit: pending signals: cannot modify limit: Operation not permitted
setup-web-1  | /opt/gitlab/embedded/bin/runsvdir-start: line 37: /proc/sys/fs/file-max: Read-only file system
```


"502 Whoops" error message, why?


End of logs:

``` shellsession
setup-web-1  | gitlab Reconfigured!
setup-web-1  | Checking for unmigrated data on legacy storage
```

``` shellsession
docker-compose exec web gitlab-ctl status
```

hangs!!

sudo cp /etc/ssl/private/__talkbank_org.key /srv/gitlab/config/ssl/gitlab.talkbank.org.key
sudo cp /etc/ssl/certs/__talkbank_org_cert.cer /srv/gitlab/config/ssl/gitlab.talkbank.org.crt

https://gitlab.com/gitlab-org/omnibus-gitlab/blob/master/doc/settings/nginx.md#manually-configuring-https
