# HomeBank server setup

Install from scratch, starting with Ubuntu Server 20.04 LTS
using vSphere Client.

TODO partition problem

## Install from CD-ROM

Need C-g then ESC.

Brian MacWhinney
homebank
jfk

openssh
import SSH identity?
GitHub or Launchpad (macw GitHub for now)
allow password

snaps

## First login

with password

$ sudo timedatectl set-timezone America/New_York
$ sudo apt upgrade

Copy over .ssh
$ scp -pr .bash_profile .ssh macw@homebank.talkbank.org:~

no keychain required?

$ sudo apt install apache2

$ scp -pr data-cmdi data-cmdi-oai rm-empty-dirs.sh bin .gitconfig .local macw@homebank.talkbank.org:~

for setup later:
$ scp -pr apache2 oai.tgz shibboleth tomcat9.* macw@homebank.talkbank.org:~

Apache
shib.conf (later)

proxy.conf
speling

passwords

sites-available
000-default-ssl.conf

ours:
$ cd apache2
$ sudo cp mods-available/ssl.conf /etc/apache2/mods-available/
$ sudo cp -p -i sites-available/* /etc/apache2/sites-available/
$ sudo a2dissite 000-default
$ sudo cp -p -i sites-enabled/* /etc/apache2/sites-enabled/
$ sudo a2enmod proxy speling ssl headers
$ sudo a2enmod cgid
$ sudo apt install libcgi-session-perl graphviz
$ sudo apt install libapache2-mod-php

remove iascl!

manually install chatter
from home: ./update-xml 174
from swan: ./deploy-xsddoc.sh


$ sudo apt install tomcat9


TODO
Only in tomcat9/Catalina/localhost: docs.xml
Only in tomcat9/Catalina/localhost: examples.xml
Only in tomcat9/Catalina/localhost: host-manager.xml
Only in tomcat9/Catalina/localhost: manager.xml
Only in tomcat9: localhost-rsa-cert.pem
Only in tomcat9: localhost-rsa-chain.pem
Only in tomcat9: localhost-rsa-key.pem
Only in tomcat9/policy.d: 10examples.policy
diff -ur /etc/tomcat9/server.xml tomcat9/server.xml


$ git clone --depth 1 git@bitbucket.org:talkbank/gra-cgi.git
$ sudo cp -p gra-cgi/morgra2jpg.cgi /usr/lib/cgi-bin/

$ git clone ssh://git@git.talkbank.org:29418/TalkBank/samtale-users.git

## git.talkbank.org force deploy

Need correct order because sub-sites depend on the main site
directories to exist.

$ mkdir -p /var/www/talkbank/software

$ for i in *-site; do ~/staging/scripts/deploy-1.py --repo $i --force; done

$ for i in browser browser-clan CapVid education education-rhd education-tbi mor screencasts styles users; do ~/staging/scripts/deploy-1.py --repo $i --force; done

TODO Missing anything?

## Home
bin/

data-cmdi/
data-cmdi-oai/

rm-empty-dirs.sh

.bashrc
.bash_profile
.gitconfig
.local/
.ssh/

samtale-users/ (relies on PHP?)

### Tools to clone and build

git clone --depth 1 git@bitbucket.org:talkbank/certificates.git
cd /home/macw/certificates/star_talkbank_org
sudo cp __talkbank_org.key /etc/ssl/private
sudo cp __talkbank_org.cer __talkbank_org_cert.cer __talkbank_org_interm.cer /etc/ssl/certs

oai-rename-sync

joai_v3.2.zip

oai-setup

## System

apt list --installed

apache2-bin/focal-updates,focal-security,now 2.4.41-4ubuntu3.1 amd64 [installed]
apache2-data/focal-updates,focal-updates,focal-security,focal-security,now 2.4.41-4ubuntu3.1 all [installed]
apache2-dev/focal-updates,focal-security,now 2.4.41-4ubuntu3.1 amd64 [installed]
apache2-utils/focal-updates,focal-security,now 2.4.41-4ubuntu3.1 amd64 [installed]
apache2/focal-updates,focal-security,now 2.4.41-4ubuntu3.1 amd64 [installed]
libapache2-mod-shib/focal,now 3.0.4+dfsg1-1build1 amd64 [installed]

ca-certificates-java/focal,focal,now 20190405ubuntu1 all [installed,automatic]
java-common/focal,focal,now 0.72 all [installed,automatic]
libatk-wrapper-java-jni/focal,now 0.37.1-1 amd64 [installed,automatic]
libatk-wrapper-java/focal,focal,now 0.37.1-1 all [installed,automatic]
libeclipse-jdt-core-java/focal,focal,now 3.18.0+eclipse4.12-1 all [installed,automatic]
libel-api-java/focal,focal,now 3.0.0-2 all [installed,automatic]
libjsp-api-java/focal,focal,now 2.3.4-2 all [installed,automatic]
libservlet-api-java/focal,focal,now 4.0.1-2 all [installed,automatic]
libservlet3.1-java/focal,focal,now 1:4.0.1-2 all [installed]
libtaglibs-standard-impl-java/focal,focal,now 1.2.5-2 all [installed,automatic]
libtaglibs-standard-spec-java/focal,focal,now 1.2.5-2 all [installed,automatic]
libtomcat9-embed-java/focal-updates,focal-updates,focal-security,focal-security,now 9.0.31-1ubuntu0.1 all [installed]
libtomcat9-java/focal-updates,focal-updates,focal-security,focal-security,now 9.0.31-1ubuntu0.1 all [installed,automatic]
libwebsocket-api-java/focal,focal,now 1.1-1 all [installed,automatic]
oracle-java15-installer/focal,now 15.0.1-1~linuxuprising0 amd64 [installed]
oracle-java15-set-default/focal,focal,now 15.0.1-1~linuxuprising0 all [installed,automatic]

adoptopenjdk-8-hotspot/focal,now 8u272-b10-3 amd64 [installed]
default-jdk-headless/focal,now 2:1.11-72 amd64 [installed,automatic]
default-jdk/focal,now 2:1.11-72 amd64 [installed]
openjdk-11-jdk-headless/focal-updates,focal-security,now 11.0.8+10-0ubuntu1~20.04 amd64 [installed,automatic]
openjdk-11-jdk/focal-updates,focal-security,now 11.0.8+10-0ubuntu1~20.04 amd64 [installed]
openjdk-11-jre-headless/focal-updates,focal-security,now 11.0.8+10-0ubuntu1~20.04 amd64 [installed,automatic]
openjdk-11-jre/focal-updates,focal-security,now 11.0.8+10-0ubuntu1~20.04 amd64 [installed,automatic]
openjdk-14-jdk-headless/focal-updates,now 14.0.2+12-1~20.04 amd64 [installed,automatic]
openjdk-14-jdk/focal-updates,now 14.0.2+12-1~20.04 amd64 [installed]
openjdk-14-jre-headless/focal-updates,now 14.0.2+12-1~20.04 amd64 [installed,automatic]
openjdk-14-jre/focal-updates,now 14.0.2+12-1~20.04 amd64 [installed,automatic]
openjdk-8-jre-headless/focal-updates,focal-security,now 8u265-b01-0ubuntu2~20.04 amd64 [installed,automatic]

keychain

sudoers-macw:
macw ALL=NOPASSWD:/usr/bin/rsync
macw ALL=NOPASSWD:/usr/sbin/apachectl

$ sudo chown 0440 sudoers-macw
$ sudo cp sudoers-macw /etc/sudoers.d

root protected: /etc/tomcat9
system: /lib/systemd/system/tomcat9.service
old tomcat.service??

OAI: sudo
/var/lib/tomcat9/webapps/oai

/etc/shibboleth
/etc/apache2/



certificates: sudo
/etc/ssl/private/__talkbank_org.key

/etc/ssl/certs/__talkbank_org.cer
/etc/ssl/certs/__talkbank_org_cert.cer
/etc/ssl/certs/__talkbank_org_interm.cer


Java 8 needed at all??

php stuff, including Apache

## TODO samtale rid PHP

https://samtalebank.talkbank.org/register/index.php
