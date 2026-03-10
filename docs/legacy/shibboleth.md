# Shibboleth

## Installation of Shibboleth 3

On Ubuntu 20.04 LTS.

TODO
https://www.switch.ch/aai/guides/sp/installation/
https://documentation.its.umich.edu/node/343

TODO

https://gluu.org/docs/gluu-server/integration/sswebapps/saml-sp/

``` shellsession
$ apt-cache search shibboleth
libapache2-mod-shib - Federated web single sign-on system (Apache module)
liblog4shib-dev - log4j-style configurable logging library for C++ (development)
liblog4shib-doc - log4j-style configurable logging library for C++ (API docs)
liblog4shib2 - log4j-style configurable logging library for C++ (runtime)
libshibresolver-dev - Shibboleth SP Attribute Resolver library (development)
libshibresolver2 - Shibboleth SP Attribute Resolver library
libshibsp-dev - Federated web single sign-on system (development)
libshibsp-doc - Federated web single sign-on system (API docs)
libshibsp-plugins - Federated web single sign-on system (plugins)
libshibsp8 - Federated web single sign-on system (runtime)
ruby-omniauth-shibboleth - OmniAuth Shibboleth strategies for OmniAuth
shibboleth-sp-common - Federated web single sign-on system (common files)
shibboleth-sp-utils - Federated web single sign-on system (daemon and utilities)
shibboleth-sp2-common - transitional package
shibboleth-sp2-utils - transitional package
simplesamlphp - Authentication and federation application supporting several protocols
sympa - Modern mailing list manager
wordpress-shibboleth - Shibboleth plugin for WordPress
```


``` shellsession
$ sudo apt-get install shibboleth-sp-utils libapache2-mod-shib
```

First step to check for loading:

``` shellsession
$ sudo shibd -t
[...]
overall configuration is loadable, check console or log for non-fatal problems
```

``` shellsession
$ sudo a2enconf shib
$ sudo a2enmod shib
```

## Updating from Shibboleth 2 to 3

https://wiki.shibboleth.net/confluence/display/SP3/ReleaseNotes

TODO

virtual host?

TODO InCommon migration

https://spaces.at.internet2.edu/display/MDQ/migrate-to-mdq

says:

``` text
InCommon will retire the legacy InCommon metadata service at md.incommon.org on Monday, March 1, 2021.
```

Get the new certificate:

http://md.incommon.org/certs/inc-md-cert-mdq.pem

https://spaces.at.internet2.edu/display/MDQ/configure-shib-sp

## Configuration

https://www.switch.ch/aai/guides/sp/configuration/

Outdated Clarin guide: https://clarin-eric.github.io/SPF-tutorial/Shib_SP_tutorial.html


TODO Clarin stuff: https://github.com/clarin-eric/SPF-SPs-metadata/blob/master/metadata/childes.talkbank.org%252Fshibboleth.xml

https://github.com/clarin-eric/SAML-metadata-checker


Config files are in `/etc/shibboleth/`.

Copy over our `incommon.pem`.

Copy over our `sp-cert.pem`, `sp-key.pem`, already generated earlier
with

``` shellsession
$ sudo shib-keygen -f -u _shibd -h homebank.talkbank.org -y 3 -e https://homebank.talkbank.org/shibboleth -o /etc/shibboleth/
```

Sample config file:

``` shellsession
$ curl --output ~/sample-shibboleth2.xml 'https://www.switch.ch/aai/docs/shibboleth/SWITCH/3.1/sp/deployment/download/customize.php/shibboleth2.xml?osType=nonwindows&hostname=homebank.talkbank.org&targetURL=https%3A%2F%2Fhomebank.talkbank.org%2FShibboleth.sso%2FSession&keyPath=%2Fetc%2Fshibboleth%2Fsp-key.pem&certPath=%2Fetc%2Fshibboleth%2Fsp-cert.pem&federation=SWITCHaai&supportEmail=FranklinChen%40cmu.edu&wayfURL=https%3A%2F%2Fwayf.switch.ch%2FSWITCHaai%2FWAYF&metadataURL=http%3A%2F%2Fmetadata.aai.switch.ch%2Fmetadata.switchaai%2Bidp.xml&metadataFile=metadata.switchaai%2Bidp.xml&eduIDEntityID=https%3A%2F%2Feduid.ch%2Fidp%2Fshibboleth&hide=windows-only,eduid-only,'
```

What is

``` xml
<MetadataFilter type="Signature" certificate="fedsigner.pem" verifyBackup="false"/>
```

TODO I don't know what `fedsigner.pem` is, so commented it out!

TODO What's the `Blacklist` thing? Ignoring for now.

Hmm, use same `sp-key.pem` and `sp-cert.pem` pair for both signing and
encryption.

Sample attribute file:

``` shellsession
$ curl --output ~/sample-attribute-policy.xml 'https://www.switch.ch/aai/docs/shibboleth/SWITCH/3.1/sp/deployment/download/customize.php/attribute-policy.xml?osType=nonwindows&hide='
```

## Logs

Log files are in `/var/log/shibboleth/`, especially
`/var/log/shibboleth/shibd.log`.

TODO watch out for cache files in `/var/cache/shibboleth/`.

## TODO Bug

Some kind of infinite loop? CPU usage. Can't kill

``` shellsession
$ sudo systemctl stop shibd
$ sudo systemctl disable shibd
```

## Virtual hosts using Shibboleth

All the virtual hosts involving CHAT data currently support a
Shibboleth login for a specific protected directory.

For example, https://homebank.talkbank.org/secure is set up.

TODO List them all?

### Usage

TODO I believe that the protected directory is not currently being
used at all.
