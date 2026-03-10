# GitBucket

[GitBucket](https://gitbucket.github.io/) is "an open source Git
platform on JVM".

## Why we use GitBucket

We decided we needed our own [Git server](git-server.md) and at the
time, we were very constrained since we were using only Macs, no Linux
machines, to host all of our servers, and so Franklin chose the fairly
obscure GitBucket, solely because it relies only on [Java](java.md)
and is therefore independent of operating system and was installable
on Mac.

Eventually we did obtain a Linux server to host Git, and we migrated
our Mac installation of GitBucket over to [Ubuntu Linux](ubuntu.md).

The current release of GitBucket, version 4.33.0, is from December 31,
2019.

## TODO Migration to GitLab?

If we continue self-hosting Git, we will eventually want to get off
GitBucket because it has severe limitations. [GitLab](gitlab.md) would
be the obvious choice, since it is well-supported on Linux.

## Installation

We have a manual installation of GitBucket on `git.talkbank.org`,
roughly following instructions that happened to be found at
https://www.rosehosting.com/blog/install-gitbucket-on-ubuntu-16-04/
and did the job.

We created a user `gitbucket` with home directory
`/home/gitbucket`. All the data and metadata are stored in
`/home/gitbucket/.gitbucket`.

The `gitbucket` account has the [usual password](credentials.md).

Installation steps (TODO some are out of date now!):

``` shellsession
$ sudo apt-get install python-software-properties -y
$ sudo add-apt-repository ppa:webupd8team/java
$ sudo apt-get update -y
$ sudo apt-get install oracle-java8-installer -y
$ sudo adduser --gecos 'Gitbucket User' gitbucket
$ sudo wget -O /home/gitbucket/gitbucket.war https://github.com/gitbucket/gitbucket/releases/download/4.13/gitbucket.war
$ sudo chown -R gitbucket: /home/gitbucket
```

Creation of `/etc/systemd/system/gitbucket.service` setting up port
8089 (to try to avoid conflicts) and using the default system Java
version:

```
[Unit]
Description=GitBucket service
After=syslog.target
After=network.target

[Service]
User=gitbucket
ExecStart=/usr/bin/java -jar /home/gitbucket/gitbucket.war --port=8089 --host=127.0.0.1

[Install]
WantedBy=multi-user.target
```


### Reverse proxy for Apache

To hide the details of GitBucket, we provide a [reverse
proxy](https://en.wikipedia.org/wiki/Reverse_proxy) in which the
GitBucket server is served through Apache.


``` shellsession
$ sudo a2enmod proxy
$ sudo a2enmod proxy_http
$ sudo a2enmod proxy_balancer
$ sudo a2enmod lbmethod_byrequests
```

Edit `/etc/apache2/sites-available/000-default.conf` for port 8089.


``` apacheconf
        ProxyPreserveHost On

        ProxyPass / http://127.0.0.1:8089/
        ProxyPassReverse / http://127.0.0.1:8089/
```

TODO We are not using https yet!

### Upgrades

Franklin manually upgrades GitBucket now and then by shutting down the
service and downloading a new version of the GitBucket WAR file from
https://github.com/gitbucket/gitbucket/releases, copying it to
`/home/gitbucket/gitbucket.war`, and then restarting the service, e.g.

``` shellsession
$ sudo systemctl stop gitbucket.service
$ sudo wget -O /home/gitbucket/gitbucket.war https://github.com/gitbucket/gitbucket/releases/download/4.33.0/gitbucket.war
$ sudo systemctl start gitbucket.service
```

## Our repositories

### CapVid

TODO Missing a README.

Some sort of JavaScript/PHP Web app. This is deployed to `slabank`.

### aphasia repos

aphasia-data
aphasia-site

### asd repos

asd-data
asd-site

### biling repos

biling-data
biling-site

### browser

Source code in JavaScript and PHP for the [TalkBank
browser](talkbank-browser.md).

### [browser-clan](browser-clan.md)

Companion to the TalkBank browser.

### ca repos

ca-data
ca-site

### cheatsheet

Brian's personal "cheat sheet" containing information for himself to use.

TODO Much of this information may be out of date or incorrect.

### childes repos

childes-data
childes-site

### [clan-info](clan-info.md)

Maintained by Brian.

### class repos

class-data
class-site

### dementia repos

dementia-data
dementia-site

### documentation

In progress: complete documentation about everything we have and how
we do it!

### education

Special material deployed to aphasia.

### education-rhd

Special material deployed to rhd.

### education-tbi

Special material deployed to tbi.

### fluency repos

fluency-data
fluency-site

### homebank repos

homebank-data
homebank-site

### homebank-secure

TODO Last modified 2017. Unused as far as I know.

### iascl

For a site without data, using its own styles.

### mor

MOR material. ZIP files are built and deployed to talkbank-site.

### phon repos

phon-data
phon-site

### psyling

A standalone site.

### rhd repos
rhd-data
rhd-site

### samtale repos

samtale-data
samtale-site

### samtale-users

Registration from samtale Web site uses PHP to modify `users.txt` stored on
homebank at `/home/macw/samtale-users`. Franklin once in a while
manually commits and pushes changes.

TODO Stop doing this and instead write to a database in the cloud
somewhere.

### screencasts

Material deployed to talkbank.

### semtalk

A standalone site.

### slabank repos
slabank-data
slabank-site

### staging

Our scripts for build/check/deploy.

### step

A standalone site.

### styles

CSS style sheets that are deployed to multiple Web sites.

TODO List which ones.

### talkbank

The rudimentary [TalkBank API](talkbank-api.md) in
[TypeScript](typescript.md), for accessing
[schema-validated](xml-schema.md) CHAT XML.

Currently used by John for [TalkBank DB](talkbank-db.md).

### talkbank-data

TODO Obsolete as of 2018. Should be deleted.

### talkbank-site

Note that there is no longer a `talkbank-data` used in conjunction
with this portal site.

TODO That makes the naming convention very confusing.

### tbi repos

tbi-data
tbi-site

### users (owned by Administrator, not TalkBank)

Brian keeps track of Apache protection users and passwords here.

TODO These are in plain text!

Apache password files are built and deployed as appropriate.

## Repos that are large (roughly over 1 GB)

- homebank-data
- talkbank-site
- phon-data
- screencasts
- childes-data
- childes-site
- psyling
- aphasia-site
- step
- education
- education-rhd

TODO what to do about big repos? Or big files in repos?

Investigate Git LFS?
https://github.com/git-lfs/git-lfs/wiki/Implementations

## List of repos to migrate

Stuff that should be pruned because huge:
4.1G	education.git
3.3G	screencasts.git
1.1G	psyling.git

*data
*site

aphasia-data.git
aphasia-site.git
asd-data.git
asd-site.git
biling-data.git
biling-site.git
browser-clan.git
browser.git
ca-data.git
CapVid.git
ca-site.git
cheatsheet.git
childes-data.git
childes-site.git
clan-info.git
class-data.git
class-site.git
dementia-data.git
dementia-site.git
documentation.git
education.git
education-rhd.git
education-tbi.git
fluency-data.git
fluency-site.git
homebank-data.git
homebank-secure.git
homebank-site.git
iascl.git
mor.git
phon-data.git
phon-site.git
psychosis-data.git
psychosis-site.git
psyling.git
rhd-data.git
rhd-site.git
samtale-data.git
samtale-site.git
samtale-users.git
screencasts.git
semtalk.git
slabank-data.git
slabank-site.git
staging.git
step.git
styles.git
talkbank-data.git
talkbank.git
talkbank-site.git
tbi-data.git
tbi-site.git
voice-site.git
