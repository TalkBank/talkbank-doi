# Ubuntu Linux

TODO
our upgrade

sudo apt-get update -y; sudo apt-get upgrade -y; sudo apt-get dist-upgrade -y; sudo apt autoremove -y

There is a [Ubuntu tutorial site](https://ubuntu.com/tutorials) that is useful.

## On our servers

Franklin has configured both `git.talkbank.org` and
`homebank.talkbank.org` to use the same LTS version of Ubuntu Linux,
18.04 LTS.

TODO details
bionic

TODO
ntp

Leonid is using a much older version of Ubuntu, version 16.04.5 LTS.
TODO Leonid may need to upgrade at some point.



## Users

We had a single `macw` user created, with password authentication
using our [credentials](credentials.md).

I don't know what commands were used, but maybe

``` shellsession
# adduser macw
# usermod -aG sudo macw
```

in order to have `sudo` access.

## SSH keys

We added [SSH](ssh.md) keys using our [credentials](credentials.md).

TODO describe

TODO describe hack to save and enter manually upon each reboot.

TODO remove password authentication and use only SSH keys for access?

## Firewall

I believe we do not have the firewall set up:

``` shellsession
$ sudo ufw status
Status: inactive
```

Here is the current app list:

``` shellsession
$ sudo ufw app list
Available applications:
  Apache
  Apache Full
  Apache Secure
  OpenSSH
  Postfix
  Postfix SMTPS
  Postfix Submission
```

TODO should we turn on the firewall?

## Kernel patches

TODO Investigate using the [Canonical Livepatch Service](https://ubuntu.com/tutorials/enable-the-livepatch-service) to get automatic kernel patches.

This is not critical for us, as Franklin periodically manually applies updates and reboots the servers, but is potentially useful because it takes Franklin out of the loop.

## Upgrades

TODO Look at the general [Ubuntu upgrade tutorial](https://ubuntu.com/tutorials/upgrading-ubuntu-desktop).

### To Ubuntu 20.04 LTS (released April 23, 2020), "Focal Fossa"

Ubuntu 20.04 LTS was announced at https://ubuntu.com/blog/ubuntu-20-04-lts-arrives and https://wiki.ubuntu.com/FocalFossa/ReleaseNotes is the release notes.

It is supported up to 2025.

New: there is a [Snap Store](https://snapcraft.io/store).

I upgraded `git.talkbank.org` and `homebank.talkbank.org` to 20.04
LTS, using the steps found
[here](https://ubuntu.com/blog/how-to-upgrade-from-ubuntu-18-04-lts-to-20-04-lts-today).

``` shellsession
$ sudo do-release-upgrade -d
```

```
To make recovery in case of failure easier, an additional sshd will
be started on port '1022'. If anything goes wrong with the running
ssh you can still connect to the additional one.
If you run a firewall, you may need to temporarily open this port. As
this is potentially dangerous it's not done automatically. You can
open the port with e.g.:
'iptables -I INPUT -p tcp --dport 1022 -j ACCEPT'

```

We must be careful not to break things. This might also be an opportunity
to use

- more recent Java (OK already)
- more recent Python 3 (yes, Python 3.8.2) and pip
- non-manual [Apache Tomcat installation](tomcat.md)
- non-manual installation of [SCons](scons.md) (yes, used `sudo apt install scons`)

## Shibboleth

Setting up shibboleth-sp-common (3.0.4+dfsg1-1build1) ...

```
Configuration file '/etc/shibboleth/attribute-map.xml'
 ==> Modified (by you or by a script) since installation.
 ==> Package distributor has shipped an updated version.
   What would you like to do about it ?  Your options are:
    Y or I  : install the package maintainer's version
    N or O  : keep your currently-installed version
      D     : show the differences between the versions
      Z     : start a shell to examine the situation
 The default action is to keep your current version.
*** attribute-map.xml (Y/I/N/O/D/Z) [default=N] ?
Installing new version of config file /etc/shibboleth/attribute-policy.xml ...
Installing new version of config file /etc/shibboleth/example-shibboleth2.xml ...
Installing new version of config file /etc/shibboleth/native.logger ...
Installing new version of config file /etc/shibboleth/security-policy.xml ...

Configuration file '/etc/shibboleth/shibboleth2.xml'
 ==> Modified (by you or by a script) since installation.
 ==> Package distributor has shipped an updated version.
   What would you like to do about it ?  Your options are:
    Y or I  : install the package maintainer's version
    N or O  : keep your currently-installed version
      D     : show the differences between the versions
      Z     : start a shell to examine the situation
 The default action is to keep your current version.
*** shibboleth2.xml (Y/I/N/O/D/Z) [default=N] ?
```

``` shellsession
macw@homebank:/etc/apache2/sites-available$ sudo shibd -t
2020-07-29 12:24:57 WARN Shibboleth.Config : DEPRECATED: legacy 2.0 configuration, support will be removed from a future version of the software
2020-07-29 12:24:57 WARN Shibboleth.RequestMapper : DEPRECATED: legacy 2.0 configuration, support will be removed from a future version of the software
2020-07-29 12:24:57 INFO Shibboleth.PropertySet : DEPRECATED: legacy configuration, remapping property/set (urn:mace:shibboleth:2.0:native:sp:config) to (urn:mace:shibboleth:3.0:native:sp:config)
2020-07-29 12:24:57 INFO Shibboleth.PropertySet : DEPRECATED: legacy configuration, remapping property/set (urn:mace:shibboleth:2.0:native:sp:config) to (urn:mace:shibboleth:3.0:native:sp:config)
2020-07-29 12:24:57 WARN OpenSAML.MetadataProvider.XML : DEPRECATED: uri attribute should be replaced with url to specify remote resource
2020-07-29 12:24:57 WARN OpenSAML.MetadataProvider.XML : DEPRECATED: uri attribute should be replaced with url to specify remote resource
2020-07-29 12:24:57 WARN OpenSAML.MetadataProvider.XML : DEPRECATED: uri attribute should be replaced with url to specify remote resource
```

https://documentation.its.umich.edu/node/343

TODO Reconcile with default?

## Java

https://www.digitalocean.com/community/tutorials/how-to-install-java-with-apt-on-ubuntu-20-04

We use OpenJDK, not Oracle.

``` shellsession
$ sudo add-apt-repository ppa:linuxuprising/java
$ sudo apt install default-jdk
$ sudo apt install openjdk-8-jdk
$ sudo apt install openjdk-15-jdk
$ sudo update-alternatives --config javac
```

Have set to Java 15 instead of the default Java 11.

## Postfix

TODO We suspect someone is using Postfix to cause Homebank to crash.

```
$ sudo update-rc.d postfix disable
$ sudo systemctl stop postfix

$ sudo apt-get remove --purge postfix
```

## Log files

An [overview of log files in Ubuntu](https://ubuntu.com/tutorials/viewing-and-monitoring-log-files#2-log-files-locations).

Remember that log files are in `/var/log/`.

TODO Example log files we should look at.

TODO Our configurations for log file paths for our services.

IASCL needs redeploy
IASCL Apache needed
Enabling site iascl-passwords.
Enabling site iascl-ssl.
Enabling site iascl.

## Upgrade

``` shellsession
sudo do-release-upgrade -d
```

TODO problem

``` text
A fatal error occurred

Please report this as a bug and include the files
/var/log/dist-upgrade/main.log and /var/log/dist-upgrade/apt.log in
your report. The upgrade has aborted.
Your original sources.list was saved in
/etc/apt/sources.list.distUpgrade.

Traceback (most recent call last):

File "/tmp/ubuntu-release-upgrader-p7ci_spq/jammy", line 8, in
<module>
sys.exit(main())

File
"/tmp/ubuntu-release-upgrader-p7ci_spq/DistUpgrade/DistUpgradeMain.py",
line 227, in main
from .DistUpgradeController import DistUpgradeController

File
"/tmp/ubuntu-release-upgrader-p7ci_spq/DistUpgrade/DistUpgradeController.py",
line 25, in <module>
import dbus

ModuleNotFoundError: No module named 'dbus'
```

Me:

``` shellsession
sudo apt-get install --reinstall python-dbus
```
