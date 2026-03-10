# OAI provider

## Purpose

The OAI server provides CMDI to the outside world for harvesting.

## Host server is `homebank.talkbank.org`

The Tomcat installation is accessible as https://oai.talkbank.org:8443/
and our OAI provider has access point
https://oai.talkbank.org:8443/oai/ by means of an Apache proxy to
forward requests to Tomcat.

We redirect https://oai.talkbank.org appropriately.

## Installation

It is implemented with an installation of a released
[jOAI](https://github.com/NCAR/joai-project) WAR file into an [Apache
Tomcat](tomcat.md) container.

### Caveats on installation

Note [our Tomcat](tomcat.md) issues.

Get the latest release:

``` shellsession
cd
wget https://github.com/NCAR/joai-project/releases/download/v3.3/joai_v3.3.zip
unzip ~/joai_v3.3.zip
```

### Dockerfile stuff

Important!!

``` shell
cd /var/lib/tomcat10
sudo wget -O lib/woodstox-core-5.0.3.jar 'https://search.maven.org/remotecontent?filepath=com/fasterxml/woodstox/woodstox-core/5.0.3/woodstox-core-5.0.3.jar'
sudo wget -O lib/stax2-api-4.0.0.jar 'https://search.maven.org/remotecontent?filepath=org/codehaus/woodstox/stax2-api/4.0.0/stax2-api-4.0.0.jar'
```

## Configuration

TODO details of configuration

TODO Attempting to add authentication to OAI somehow failed. Authentication
had been working before.

A watch directory is configured so that any changes to it result in
automatic re-indexing.

### Saving configs to latest Tomcat

TODO not right

``` shell
sudo cp -pr /opt/tomcat/latest/webapps/oai/WEB-INF/harvester_settings_and_data /opt/tomcat/latest/webapps/oai/WEB-INF/repository_settings_and_data /var/lib/tomcat9/webapps/oai/WEB-INF
```

TODO save

## [Updating from CMDI data](cmdi.md)

## TODO Bugs?

Look at `/var/lib/tomcat10/logs/` for information. Java exceptions.

Problem after doing https redirect.

## Tomcat 10 migration

Error in `/var/lib/tomcat10/logs/localhost*.log`:

```
        java.lang.NoClassDefFoundError: javax/servlet/Filter

```

https://tomcat.apache.org/migration-10.html#Migrating_from_9.0.x_to_10.0.x

https://tomcat.apache.org/download-migration.cgi

Use `webapps-javaee` instead of `webapps`.

```
cd /var/lib/tomcat10
sudo mkdir webapps-javaee
sudo cp ~/joai_v3.3/oai.war webapps-javaee
sudo chown -R tomcat webapps-javaee
```

TODO Didn't work!

Use https://github.com/apache/tomcat-jakartaee-migration

``` shellsession
sudo apt-get install tomcat-jakartaee-migration
```

Migrate:

``` shellsession
javax2jakarta ~/joai_v3.3/oai.war ~/oai.war
sudo mv ~/oai.war /var/lib/tomcat10/webapps/
```
