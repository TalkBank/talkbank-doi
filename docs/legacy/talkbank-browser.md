# TalkBank browser

A subset of our Web sites use their own copy of the TalkBank browser
app that is implemented in [PHP](php.md).

Leonid is currently the maintainer of the TalkBank browser: "Anyone
who want can be involved. The reason I am taking case of browser is
because no one else wanted to do it."

The app also uses [TalkBank CLAN](talkbank-clan.md).

## Sites that include the TalkBank browser

TODO List all the sites that provide it. Extract this info from the
deployment config file.

A copy of the browser is deployed to the following Web sites (as
indicated in `config.py` with `has_browser` flag):

- aphasia
- asd
- biling
- ca
- childes
- class
- dementia
- fluency
- homebank
- phonbank
- rhd
- samtalebank
- slabank
- tbi

## TODO MySQL usage?

What is this in `includes/config.php`?

``` php
define("DB_PASS", "");
```

## TODO Limitations

When there is an error (a missing video file, or a wrong permission on
the media server for a folder, or the media server is down, etc.), the
TalkBank browser just silently fails, leaving no error message to
indicate what the problem is. TODO Someone should fix, and pop up an
error message when something is wrong on the media server.
