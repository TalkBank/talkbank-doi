# Web site checkers

## Link checkers

Currently, Brian manually runs a (proprietary?) Web link checker on
selected already-deployed Web sites. TODO How does this handle
password-protected areas?

TODO Check `https` links instead of `http`

## Web security checkers

https://observatory.mozilla.org/

### Example run on `childes.talkbank.org`

https://observatory.mozilla.org/analyze/childes.talkbank.org HTTP Observatory scored F.

https://observatory.mozilla.org/analyze/childes.talkbank.org#tls is non-compliant

TODO TLS confgure

https://wiki.mozilla.org/Security/Server_Side_TLS

https://observatory.mozilla.org/analyze/childes.talkbank.org#third-party Third-party tests

SSL Labs https://www.ssllabs.com/ssltest/analyze?d=childes.talkbank.org
scored B

https://www.immuniweb.com/ssl/?id=1JAU0Z6l
TODO

https://securityheaders.com/?followRedirects=on&hide=on&q=childes.talkbank.org scored F



### `talkbank.org` domain

https://hstspreload.org/?domain=talkbank.org

TODO fix

### `media.talkbank.org`

### TODO all the other hosts we have

## PageSpeed

https://developers.google.com/speed/pagespeed/insights/

### `childes.talkbank.org`

https://developers.google.com/speed/pagespeed/insights/?url=childes.talkbank.org

#### Mobile

scored 95

why are we still using `secure.statcounter.com`?

TODO other stuff

#### Desktop

scored 100

## TODO Web page test

https://www.webpagetest.org/

try out?

## TODO Mobile-friendly

https://search.google.com/test/mobile-friendly

try it out?
