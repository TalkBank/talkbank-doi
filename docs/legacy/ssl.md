# SSL (this is the old way, obsoleted by [certbot](certbot.md))

A useful tutorial on SSL/TLS: https://www.smashingmagazine.com/2017/06/guide-switching-http-https/

## Certificates

### Acquiring

Brian manually manages application for and renewal of SSL certificates
through CMU.

We generate a Certificate Signing Request (CSR).

### Saving and installing

We eventually decided to use [wildcard
certificates](https://cwiki.apache.org/confluence/display/HTTPD/UnderstandingMultiUseSSLCertificates)
for `*.talkbank.org` so that whenever we add or change virtual hosts,
we don't need a new set of certificates.

(The exception is that the certificate for `media.talkbank.org` is
handled by CMU.)

The certificates are downloaded from email links:

the `certificates` repo at
`git@bitbucket.org:talkbank/certificates.git` and manually copies them
to appropriate Web hosts `homebank` and `git` along with custom Apache
configurations for them: `/etc/ssl/certs/` for certificates and
`/etc/ssl/private/` for private key and CSR.

#### Verifying

TODO

https://httpd.apache.org/docs/current/ssl/ssl_faq.html

Make sure key, CSR, certs match. They don't.

#### Installing

TODO wrong

``` shellsession
$ sudo cp star_talkbank_org/__talkbank_org_cert.cer star_talkbank_org/__talkbank_org_interm.cer /etc/ssl/certs/
$ sudo cp star_talkbank_org/__talkbank_org.key /etc/ssl/private/
```

### Using with Apache

See our [Apache setup documentation](apache-httpd.md).

## Leonid's servers

Leonid has installed the wildcard SSL certificates on
`dali.talkbank.org`.

## John's servers?

TODO John provide info on his setup.

Does he use [Certbot](https://certbot.eff.org/)?

## Name-based virtual hosts with SSL

We found out that there are problems regarding [name-based SSL virtual
hosts](https://cwiki.apache.org/confluence/display/HTTPD/NameBasedSSLVHostsWithSNI).

Luckily, we have a situation in which we are using the same SSL
certificate for all our `*.talkbank.org` virtual hosts.

We now have `www.talkbank.org` as an alias for `talkbank.org`.

TODO Fix manual editing of git virtual hosts, talkbank, iascl, voice

## Omissions

Some of our servers do not currently use SSL:

- our [Git server](git-server.md) does not
- our [OAI server](oai.md) does not
- TODO Brian's Gandalf server?

From Ted Pham:

As for installing your existing *.talkbank.org certificate on the two
new machines, Secitgo (formerly Comodo) has moved their SSL installation
docs to:

https://support.comodo.com/index.php?/Knowledgebase/List/Index/37/certificate-installation

If you need further assistance with installing them, please let us know
where you are running into trouble and we'll try to advise.

## Phase out `http`?

For compatibility, we currently internally handle redirects when
possible from `http` to `https`.

It seems advisable to permanently redirect, rather than remove `http`.

## TODO Security testing

https://www.ssllabs.com/ssltest/analyze.html?d=talkbank.org&latest
rating: B

https://www.ssllabs.com/ssltest/analyze.html?d=childes.talkbank.org&latest

https://www.ssllabs.com/ssltest/

## Updating certificates on git.talkbank.org and homebank.talkbank.org

``` shellsession
$ cd Certificates
$ git pull

$ sudo cp star_talkbank_org/__talkbank_org_cert.cer star_talkbank_org/__talkbank_org_interm.cer /etc/ssl/certs/

$ sudo cp /etc/ssl/certs/__talkbank_org_cert.cer /etc/tomcat9/localhost-rsa-cert.pem
$ sudo cp /etc/ssl/certs/__talkbank_org_interm.cer /etc/tomcat9/localhost-rsa-chain.pem
$ sudo cp /etc/ssl/private/__talkbank_org.key /etc/tomcat9/localhost-rsa-key.pem
```
