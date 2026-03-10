# Certbot setup

## (I did on Ubuntu) Install certbot with snap

Instructions:

https://certbot.eff.org/instructions?ws=apache&os=ubuntufocal

## Run certbot after Ted gave me info

Ted Pham's doc:

https://docs.google.com/document/d/1lCncZXy4T0_JfystN07FoEeIo5LOFmfbd0uk_1-C0fU/edit?tab=t.0

See the [`certbot.sh`](shell script) I wrote, factoring out variables.

Since we had Apache configs on the `homebank.talkbank.org` machine, we used `--apache` to automatically detect virtual hosts. Very convenient.

With `certonly`, stuff goes into `/etc/letsencrypt/live/yourdomain.com/`.

```
macw@homebank:~$ ~/certbot.sh
Saving debug log to /var/log/letsencrypt/letsencrypt.log

Which names would you like to activate HTTPS for?
We recommend selecting either all domains, or all domains in a VirtualHost/server block.
- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
1: talkbank.org
2: aphasia.talkbank.org
3: asd.talkbank.org
4: biling.talkbank.org
5: ca.talkbank.org
6: childes.talkbank.org
7: class.talkbank.org
8: dementia.talkbank.org
9: fluency.talkbank.org
10: homebank.talkbank.org
11: phon.talkbank.org
12: psychosis.talkbank.org
13: psyling.talkbank.org
14: rhd.talkbank.org
15: samtale.talkbank.org
16: semtalk.talkbank.org
17: slabank.talkbank.org
18: tbi.talkbank.org
19: voice.talkbank.org
20: www.talkbank.org
- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
Select the appropriate numbers separated by commas and/or spaces, or leave input
blank to select all options shown (Enter 'c' to cancel):
Requesting a certificate for talkbank.org and 19 more domains

Successfully received certificate.
Certificate is saved at: /etc/letsencrypt/live/talkbank.org/fullchain.pem
Key is saved at:         /etc/letsencrypt/live/talkbank.org/privkey.pem
This certificate expires on 2026-02-19.
These files will be updated when the certificate renews.
Certbot has set up a scheduled task to automatically renew this certificate in the background.

- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
If you like Certbot, please consider supporting our work by:
 * Donating to ISRG / Let's Encrypt:   https://letsencrypt.org/donate
 * Donating to EFF:                    https://eff.org/donate-le
- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
```

## Automatic renewal

Here is where `certbot` installed automatic renewal:

```
macw@homebank:~$ systemctl list-timers | grep certbot
Wed 2025-02-19 22:08:00 EST           10h -                                      - snap.certbot.renew.timer       snap.certbot.renew.service
```

### Manually try renewal (TODO failed)

```
$ sudo certbot renew --dry-run
Saving debug log to /var/log/letsencrypt/letsencrypt.log

- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
Processing /etc/letsencrypt/renewal/talkbank.org.conf
- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
Account registered.
Simulating renewal of an existing certificate for talkbank.org and 19 more domains
Failed to renew certificate talkbank.org with error: Unable to find a virtual host listening on port 80 which is currently needed for Certbot to prove to the CA that you control your domain. Please add a virtual host for port 80.

- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
All simulated renewals failed. The following certificates could not be renewed:
  /etc/letsencrypt/live/talkbank.org/fullchain.pem (failure)
- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
1 renew failure(s), 0 parse failure(s)
Ask for help or search for solutions at https://community.letsencrypt.org. See the logfile /var/log/letsencrypt/letsencrypt.log or re-run Certbot with -v for more details.

```

We could not figure out why this failed. Ted will continue looking into this.

## Certificate location

I have the following locations on the `homebank` machine:

```
                SSLCertificateKeyFile /etc/ssl/private/__talkbank_org.key
                SSLCertificateFile /etc/ssl/certs/__talkbank_org_cert.cer
                SSLCertificateChainFile /etc/ssl/certs/__talkbank_org_interm.cer
```

Ted told me to move away those old certificates and use symlinks from the new `letsencrypt` area:

```
sudo ln -s /etc/letsencrypt/live/talkbank.org/privkey.pem /etc/ssl/private/__talkbank_org.key
sudo ln -s /etc/letsencrypt/live/talkbank.org/cert.pem /etc/ssl/certs/__talkbank_org_cert.cer
sudo ln -s /etc/letsencrypt/live/talkbank.org/chain.pem /etc/ssl/certs/__talkbank_org_interm.cer
```

After

```
sudo apachectl restart
```

everything worked!

## To undo and put back the wildcard certificates

```
sudo -i

rm /etc/ssl/private/__talkbank_org.key
cp ~macw/certificates/star_talkbank_org/__talkbank_org.key /etc/ssl/private/__talkbank_org.key
rm /etc/ssl/certs/__talkbank_org_cert.cer
cp ~macw/certificates/star_talkbank_org/__talkbank_org_cert.cer /etc/ssl/certs/__talkbank_org_cert.cer
rm /etc/ssl/certs/__talkbank_org_interm.cer
cp ~macw/certificates/star_talkbank_org/__talkbank_org_interm.cer /etc/ssl/certs/__talkbank_org_interm.cer
```

## Nginx

```

sudo certbot certonly --webroot -w /var/www/letsencrypt \
  --cert-name talkbank.org-le \
  -d talkbank.org \
  -d www.talkbank.org \
  -d aphasia.talkbank.org \
  -d asd.talkbank.org \
  -d biling.talkbank.org \
  -d ca.talkbank.org \
  -d childes.talkbank.org \
  -d class.talkbank.org \
  -d dementia.talkbank.org \
  -d fluency.talkbank.org \
  -d homebank.talkbank.org \
  -d motor.talkbank.org \
  -d phon.talkbank.org \
  -d psychosis.talkbank.org \
  -d rhd.talkbank.org \
  -d samtale.talkbank.org \
  -d slabank.talkbank.org \
  -d tbi.talkbank.org
```

```
sudo vi /etc/letsencrypt/renewal/talkbank.org-le.conf
```

```
deploy_hook = systemctl reload nginx
```
