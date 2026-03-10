# Apache HTTP server

We have been using [Apache HTTP server](https://httpd.apache.org) for
two decades now as our main HTTP server, simply because it is bundled
with the operating systems we have used as servers: macOS and Linux.

## TODO Should we switch to Nginx?

https://www.nginx.com/blog/nginx-vs-apache-our-view/

"Apache’s Heavyweight, Monolithic Model Has Its Limits".

## Documentation

Sources of documentation:

- [Official documentation as of version 2.4](http://httpd.apache.org/docs/2.4/)
- [Wiki](https://cwiki.apache.org/confluence/display/HTTPD/Home)

## Linux

### Ubuntu

There is a generic [Apache for Ubuntu tutorial](https://ubuntu.com/tutorials/install-and-configure-apache).

#### TODO Ubuntu 20.04 LTS

When we [upgrade Ubuntu](ubuntu.md) to 20.04 LTS, there is a useful [tutorial](https://www.digitalocean.com/community/tutorials/how-to-install-the-apache-web-server-on-ubuntu-20-04) for Apache setup.

#### Ubuntu 18.04 LTS

(This is what we are currently using on `git.talkbank.org` and `homebank.talkbank.org`).

Apache 2.4 [Ubuntu
layout](https://cwiki.apache.org/confluence/display/HTTPD/DistrosDefaultLayout#DistrosDefaultLayout-Debian,Ubuntu(Apachehttpd2.x):)
differs from the "default" layout. Details are on the server at `/usr/share/doc/apache2/README.Debian.gz`.

##### Installation and initial start

``` shellsession
$ sudo apt install apache2
$ sudo service apache2 start
```

The default is one host configured using
`/etc/apache2/sites-enabled/000-default.conf` and content root
`/var/www/html` but we change things up later.

##### Upgrading

TODO are there issues?

##### Enabling and disabling

TODO Enabling and disabling with
a2enmod/a2dismod, a2ensite/a2dissite and a2enconf/a2disconf.

Currently these have been done by hand.

TODO The automatically generated site stuff should have enabling and disabling automated.


TODO

##### Virtual hosts

We use `VirtualHost`, manually creating some configurations while automatically generating others from a template. Information about [matching](https://httpd.apache.org/docs/current/vhosts/details.html).

TODO Look into [dynamically configured mass virtual hosting](https://httpd.apache.org/docs/current/vhosts/mass.html).

###### `ServerName`

We use [name-based virtual
hosts](https://httpd.apache.org/docs/current/vhosts/name-based.html).

For each virtual host, we require `ServerName` and `DocumentRoot`.

TODO
Our [default virtual host](https://httpd.apache.org/docs/current/vhosts/name-based.html#defaultvhost)

git default: `000-default.conf`; TODO SSL
homebank default: oops, no `000-default.conf`, should be talkbank?

###### `ServerAlias`

We use `ServerAlias` only once, on `homebank.talkbank.org` for `000-default.conf`:

``` apacheconf
ServerName talkbank.org
ServerAlias www.talkbank.org
```

TODO which name should be the alias?

###### Setting up `DocumentRoot` for each virtual host

``` shellsession
$ sudo mkdir /var/www/NAME
$ sudo chown -R $USER:$USER /var/www/NAME
$ sudo chmod -R 755 /var/www/NAME
```

###### Config files

General information about [default layout](https://cwiki.apache.org/confluence/display/HTTPD/DistrosDefaultLayout).

We have `/etc/apache2/sites-available/NAME.conf` (which redirects to `NAME-ssl.conf`).

On `git.talkbank.org` without SSL:

- 000-default.conf (reverse proxy for http://git.talkbank.org that does not yet have SSL)

On `git.talkbank.org` with SSL:

- git-ssl.conf (TODO currently not working)
- psyling-ssl.conf
- semtalk-ssl.conf
- step-ssl.conf

On `homebank.talkbank.org` without SSL:

- oai.conf (reverse proxy for http://oai.talkbank.org that does not yet have SSL)

On `homebank.talkbank.org` with SSL:

- 000-ssl.conf
- aphasia-ssl.conf
- asd-ssl.conf
- biling-ssl.conf
- ca-ssl.conf
- childes-ssl.conf
- class-ssl.conf
- dementia-ssl.conf
- fluency-ssl.conf
- homebank-ssl.conf
- iascl-ssl.conf
- phon-ssl.conf
- rhd-ssl.conf
- samtale-ssl.conf
- slabank-ssl.conf
- tbi-ssl.conf
- voice-ssl.conf

##### Configuration

###### Validation

``` shellsession
$ sudo apache2ctl configtest
```

###### Firewall

TODO We don't have the firewall active currently.

``` shellsession
$ sudo ufw status
```

If we did have active, we would run

``` shellsession
$ sudo ufw allow 'Apache Full'
$ sudo ufw delete allow 'Apache'
```




TODO misc stuff

TODO I am confused by `_default_`.


`ServerAdmin`: I set to `FranklinChen@cmu.edu`.

`Directory`
Our template:

``` apacheconf
                <Directory "/path/to/root">
                  Options FollowSymLinks Multiviews Includes Indexes
                  MultiviewsMatch Any
                  IndexOptions FancyIndexing ShowForbidden
                  AllowOverride None
                  Require all granted
                </Directory>
```


Use `Options +Indexes` so that people can see directory listings.


TODO `Files`, `FilesMatch`, `Location`, `LocationMatch` for zip files etc?

avoid multiple configs of same thing

`Context`


`Listen`

##### Authentication

We want to protect some directories. (TODO individual files also?)

There are many ways to do [authentication](https://httpd.apache.org/docs/current/howto/auth.html), but we are currently using passwords.

We generate for each virtual host a configuration file for password
protection, as well as the password file for the host. The Apache
config file is named e.g. `sites-available/homebank-passwords.conf`
and for each protected directory has a section:

``` apacheconf
<Directory "/path/to/protected">
  Options Includes Indexes FollowSymLinks MultiViews
  AllowOverride None
  AuthType Basic
  AuthName "Password protected data"
  AuthUserFile /etc/apache2/passwords/homebank.talkbank.org
  Require user username
</Directory>
```

TODO audit these random settings that I don't understand. Why do we have `Includes` set, for example? Leonid?

`Deny from all`?
`Allow from .talkbank.org`?

TODO update the config?

As recommended, we do not use `.htaccess`, although that would
actually make some things easier:
https://cwiki.apache.org/confluence/display/HTTPD/FAQ says "If you
have access to edit the httpd.conf, you should not use .htaccess
files, ever."

LDAP password protection?? https://cwiki.apache.org/confluence/display/HTTPD/UseLDAPToPasswordProtectAFolder


##### Case-insensitive spelling

We installed [`mod_speling`](https://httpd.apache.org/docs/current/mod/mod_speling.html) in order to support case-insensitive URLs.

`modes-available/speling.load`

##### Content Security Policy

TODO We should follow https://infosec.mozilla.org/guidelines/web_security#content-security-policy and disable inline JavaScript, but we have to audit all our JavaScript code first.

##### X-Frame-Options

https://infosec.mozilla.org/guidelines/web_security#x-frame-options

##### X-Content-Type-Options

Follow https://infosec.mozilla.org/guidelines/web_security#x-content-type-options instructions.

##### X-XSS-Protection

https://infosec.mozilla.org/guidelines/web_security#x-xss-protection

##### SSL

Enable `mod_ssl`:

``` shellsession
$ sudo a2enmod ssl
```

See [our SSL setup](ssl.md) for details about our certificates.

Use [HTTP Strict Transport Security](https://infosec.mozilla.org/guidelines/web_security#http-strict-transport-security).

"Intermediate" configuration recommended by Mozilla: https://ssl-config.mozilla.org/#server=apache&version=2.4.41&config=intermediate&openssl=1.1.1d&guideline=5.5

Note: we tried the "modern" configuration but it was too strict.

``` apacheconf
Header always set Strict-Transport-Security "max-age=63072000; includeSubDomains"
Header set X-Content-Type-Options nosniff
Header set Content-Security-Policy "frame-ancestors 'none';"
Header set X-Frame-Options: "DENY"
Header set X-XSS-Protection: "1; mode=block"

SSLEngine On

SSLCertificateKeyFile /etc/ssl/private/__talkbank_org.key
SSLCertificateFile /etc/ssl/certs/__talkbank_org_cert.cer
SSLCertificateChainFile /etc/ssl/certs/__talkbank_org_interm.cer

Protocols h2 http/1.1

# intermediate configuration
SSLProtocol             all -SSLv3 -TLSv1 -TLSv1.1
SSLCipherSuite          ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-CHACHA20-POLY1305:DHE-RSA-AES128-GCM-SHA256:DHE-RSA-AES256-GCM-SHA384
SSLHonorCipherOrder     off
SSLSessionTickets       off

SSLUseStapling On
```

TODO best practices?

TODO

`mods-available/ssl.conf` has

``` apacheconf
        SSLSessionCache         shmcb:${APACHE_RUN_DIR}/ssl_scache(512000)
```

so add

``` apacheconf
	SSLStaplingCache		shmcb:${APACHE_RUN_DIR}/ssl_stapling(32768)
```

https://httpd.apache.org/docs/current/mod/mod_socache_shmcb.html

###### Redirecting to SSL

https://cwiki.apache.org/confluence/display/HTTPD/RedirectSSL
illustrates how to redirect to SSL. Example:

``` apacheconf
<VirtualHost *:80>
ServerName childes.talkbank.org
Redirect / https://childes.talkbank.org/
</VirtualHost>
```

Actually, make the redirect permanent as follows:

``` apacheconf
Redirect permanent / https://childes.talkbank.org/
```

##### Caching

Is there anything we might want to [cache](https://httpd.apache.org/docs/current/caching.html)?

TODO landing pages, TalkBank browser?

##### Automatic zipping

TODO Should we look into automatic zipping of material, instead of creating
ZIP archives statically?

##### Logging

[Log files](https://httpd.apache.org/docs/current/logs.html) are important for monitoring usage and errors.

We currently combine all the virtual servers' logs.

``` apacheconf
ErrorLog ${APACHE_LOG_DIR}/error.log
CustomLog ${APACHE_LOG_DIR}/access.log combined
```

Hence `/var/log/apache2/access.log` and `/var/log/apache2/error.log`.

TODO should separate out different virtual hosts' logs in order to be able to look at errors better?

##### CGI

We use Perl for one [CGI](https://httpd.apache.org/docs/current/howto/cgi.html) script, stored on `homebank.talkbank.org` at
`/usr/lib/cgi-bin/morgra2jpg.cgi`.

TODO

what's the best way to config? currently have a block

`conf-available/serve-cgi-bin.conf`


``` apacheconf
                <FilesMatch "\.(cgi|shtml|phtml|php)$">
                                SSLOptions +StdEnvVars
                </FilesMatch>
                <Directory /usr/lib/cgi-bin>
                                SSLOptions +StdEnvVars
                </Directory>
```

Example: https://www.talkbank.org/cgi-bin/morgra2jpg.cgi?morText=pro%7cyou%20v%7clike%20inf%7cto%20v%7copen%20det%7cthe%20n%7cdoor-PL%20prep%7cof%20det%7cthe%20n%7chouse%20%20%3f%20&graText=1%7c2%7cSUBJ%202%7c0%7cROOT%203%7c4%7cINF%204%7c2%7cXCOMP%205%7c6%7cDET%206%7c4%7cOBJ%207%7c6%7cMOD%208%7c9%7cDET%209%7c7%7cPOBJ%2010%7c2%7cPUNCT&mainText=you%20like%20to%20open%20the%20doors%20of%20the%20house%20%3f

TODO Should we consider retiring the CGI script in favor of a different solution for CLAN?

##### PHP

https://www.digitalocean.com/community/tutorials/how-to-install-linux-apache-mysql-php-lamp-stack-on-ubuntu-20-04

TODO clean up?

https://cwiki.apache.org/confluence/display/HTTPD/PHP

installation and versioning

``` shellsession
$ sudo apt install mysql-server
```

TODO document MySQL usage.

Do the following?

``` shellsession
$ sudo mysql_secure_installation
```



``` shellsession
$ sudo apt install php libapache2-mod-php php-mysql
```

To remove (for testing):

``` shellsession
$ sudo apt purge php libapache2-mod-php7.4 php-mysql mysql-server -y
```

#### TODO Leonid's Ubuntu

TODO fill out how Leonid configures.

##### Proxy

###### `http://git.talkbank.org`

TODO I haven't figured out how to do HTTPS.

We are currently using [GitBucket](gitbucket.md) and set up a reverse proxy for it by making GitBucket run on port `8089` and then pointing to it from Apache:

``` apacheconf
<VirtualHost *:80>
        ServerAdmin FranklinChen@cmu.edu

        ProxyPreserveHost On

        ProxyPass / http://127.0.0.1:8089/
        ProxyPassReverse / http://127.0.0.1:8089/
</VirtualHost>
```

##### SSL proxy

TODO

The relevant sections:

``` apacheconf
                SSLProxyEngine on
                ProxyPreserveHost On

                ProxyPass / https://127.0.0.1:8000/
                ProxyPassReverse / https://127.0.0.1:8000/
```

###### `http://oai.talkbank.org`

TODO I haven't figured out how to do HTTPS.

Tomcat reverse proxy in `oai.conf` for port `8080`:

``` apacheconf
<VirtualHost *:80>
        ServerName oai.talkbank.org
        ServerAdmin FranklinChen@cmu.edu

        ProxyPreserveHost On

        ProxyPass / http://127.0.0.1:8080/
        ProxyPassReverse / http://127.0.0.1:8080/
</VirtualHost>
```

``` shellsession
$ sudo a2enmod proxy_http
```

TODO
https://cwiki.apache.org/confluence/display/HTTPD/TomcatReverseProxy

### Red Hat



## macOS

TODO list the personal Macs that are currently serving using Apache.

The [macOS
layout](https://cwiki.apache.org/confluence/display/HTTPD/DistrosDefaultLayout#DistrosDefaultLayout-MacOSX(Leopard,Apachehttpd2.2):)
uses standard macOS locations.
