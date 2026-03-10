# Apache Tomcat

See what there is on Ubuntu.

``` shellsession
$ apt-cache search tomcat
```

## Default port

Move away from the default HTTP port `8080`.

## `https`

`conf/server.xml` has `Connector` stuff.

## Tomcat

Documentation is at https://tomcat.apache.org/tomcat-9.0-doc/index.html

SSL:

https://tomcat.apache.org/tomcat-9.0-doc/ssl-howto.html

Prepare the certificate keystore.

Port 8443 default.

https://tomcat.apache.org/tomcat-9.0-doc/proxy-howto.html


TODO Error
https://cwiki.apache.org/confluence/display/TOMCAT/Class+Not+Found+Issues

Jasper?
https://tomcat.apache.org/tomcat-9.0-doc/jasper-howto.html

### Installation on Ubuntu

https://linuxhint.com/install_apache_tomcat_server_ubuntu/

``` shellsession
$ sudo apt install tomcat10 tomcat10-admin tomcat10-docs tomcat10-examples tomcat10-user
```

This installs stuff into `/etc/tomcat10/`.

Note the service config path:

```
Created symlink /etc/systemd/system/multi-user.target.wants/tomcat10.service → /usr/lib/systemd/system/tomcat10.service.

```

Note what the default running process is:

```
/usr/lib/jvm/default-java/bin/java -Djava.util.logging.config.file=/var/lib/tomcat10/conf/logging.properties -Djava.util.logging.manager=org.apache.juli.ClassLoaderLogManager -Djava.awt.headless=true -Djdk.tls.ephemeralDHKeySize=2048 -Djava.protocol.handler.pkgs=org.apache.catalina.webresources -Dorg.apache.catalina.security.SecurityListener.UMASK=0027 --add-opens=java.base/java.lang=ALL-UNNAMED --add-opens=java.base/java.io=ALL-UNNAMED --add-opens=java.base/java.util=ALL-UNNAMED --add-opens=java.base/java.util.concurrent=ALL-UNNAMED --add-opens=java.rmi/sun.rmi.transport=ALL-UNNAMED -classpath /usr/share/tomcat10/bin/bootstrap.jar:/usr/share/tomcat10/bin/tomcat-juli.jar -Dcatalina.base=/var/lib/tomcat10 -Dcatalina.home=/usr/share/tomcat10 -Djava.io.tmpdir=/tmp org.apache.catalina.startup.Bootstrap start

```

#### Port

Change default port from `8080`.

`/etc/tomcat10/server.xml` has port assignments.

Shutdown port.
Connector port, redirect port.
AJP port.


``` shellsession
sudo systemctl enable tomcat10
```

#### Host manager app

TODO do we care?

`conf/tomcat-users.xml` roles

``` xml
<role rolename="admin-gui"/>
<role rolename="manager-gui"/>
<user username="tomcat" password="pass"roles="admin-gui,manager-gui"/>
```

#### Realms

TODO security

https://tomcat.apache.org/tomcat-9.0-doc/realm-howto.html

### Connecting to Apache HTTP server

TODO
https://cwiki.apache.org/confluence/display/TOMCAT/Connectors
https://tomcat.apache.org/connectors-doc/

https://tomcat.apache.org/tomcat-9.0-doc/config/ajp.html

Proxy
https://tomcat.apache.org/tomcat-9.0-doc/proxy-howto.html

### SSL

Enable SSL connector at port `8443`.

https://tomcat.apache.org/tomcat-9.0-doc/config/http.html#SSL_Support

https://tomcat.apache.org/tomcat-9.0-doc/ssl-howto.html

Change `server.xml` in `/var/lib/tomcat9/conf/`:

Comment out the `8080` directive:

``` xml
    <Connector port="8080" protocol="HTTP/1.1"
               connectionTimeout="20000"
               redirectPort="8443"
               maxParameterCount="1000"
               />
```


Uncomment one of the SSL connector directives, this one:

``` xml
    <Connector port="8443" protocol="org.apache.coyote.http11.Http11NioProtocol"
               maxThreads="150" SSLEnabled="true"
               maxParameterCount="1000"
               >
        <UpgradeProtocol className="org.apache.coyote.http2.Http2Protocol" />
        <SSLHostConfig>
            <Certificate certificateKeystoreFile="conf/localhost-rsa.jks"
                         type="RSA" />
        </SSLHostConfig>
    </Connector>
```

Change it to have the correct certificate paths:

``` xml
            <Certificate certificateKeyFile="conf/localhost-rsa-key.pem"
                         certificateFile="conf/localhost-rsa-cert.pem"
                         certificateChainFile="conf/localhost-rsa-chain.pem"
                         type="RSA" />
```

Copy the files appropriately:

``` shellsession
cd /var/lib/tomcat10/conf
sudo cp /etc/ssl/private/__talkbank_org.key localhost-rsa-key.pem
sudo cp /etc/ssl/certs/__talkbank_org_cert.cer localhost-rsa-cert.pem
sudo cp /etc/ssl/certs/__talkbank_org_interm.cer localhost-rsa-chain.pem
```

Restart and check that it works:

``` shellsession
sudo systemctl restart tomcat10
```

`https://oai.talkbank.org:8443/` should work.

### Apache Portable Runtime

https://tomcat.apache.org/tomcat-9.0-doc/apr.html
https://apr.apache.org/

``` shellsession
sudo apt-get install apache2-dev libapr1-dev libaprutil1-dev
```

### Logs

Check `catalina.out` etc.

`/var/log/tomcat9/localhost.*.log`:

```
        javax.servlet.ServletException: java.lang.NoClassDefFoundError: Could not initialize class org.apache.jasper.compiler.EncodingDetector
```

### Web apps

In `/var/lib/tomcat9/webapps/`.

### TODO Jasper

https://tomcat.apache.org/tomcat-9.0-doc/jasper-howto.html

```
sudo apt-get install tomcat10-common
```

`/usr/share/tomcat9/lib/`

### Use Java 8? (no)

Add to `/lib/systemd/system/tomcat9.service`:

```
Environment="JAVA_HOME=/usr/lib/jvm/adoptopenjdk-8-hotspot-amd64"
```

But this actually broke SSL, so I reverted back to default Java.

### Memory usage

TODO
