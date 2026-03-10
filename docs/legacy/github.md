# [GitHub](https://github.com)

We have a GitHub organization called
[`TalkBank`](https://github.com/TalkBank).

## Why?

GitHub is the de facto standard location for sharing materials with the
**public** and reaching the widest possible audience. For this reason,
Franklin's policy has been:

- For *anything the public may find useful and does not have hardcoded
  secrets we need to protect*, we put it on GitHub.

## Privacy

As of April 14, 2020, [GitHub is free for
teams](https://github.blog/2020-04-14-github-is-now-free-for-teams/).

TODO This means that the single main reason we have used other Git hosting
sites (the lack of unlimited private repositories on a free plan) is
no longer valid. I propose that we move as many of our repositories to GitHub
as possible, in order to consolidate where our repositories live. This
is very easy to do.

## Members

The list of members of `TalkBank` is at
https://github.com/orgs/TalkBank/people and currently there are only
two members:

- [Franklin Chen](https://github.com/orgs/TalkBank/people/FranklinChen)
- [Brian MacWhinney](https://github.com/orgs/TalkBank/people/macw)
- [John Kowalski](https://github.com/orgs/TalkBank/people/jkau1)

TODO Leonid

## Repositories

TODO list what we have, public and private

### update_chat_types (public)

Python script to update CHAT file `@Types` header based on the nearest
`0types.txt` metadata file. Modifies CHAT files in place.

This script is currently copied over manually to be run during
build/check/deploy, where automatic Git commits are also performed for
all CHAT file changes that are made.

TODO Don't copy the script manually.

TODO This step should really be performed at user commit time,
probably, checking whether `0types.txt` files have been changed?

### testchat (public)

A critical part of ensuring that [Chatter](chatter.md) does the right
thing. This repo should be considered authoritative for the public in
illustrating what is valid CHAT and what is invalid and why.

Currently, Franklin manually reruns Chatter periodically to update

TODO The manual running is quite error-prone. Changes to Chatter have
been forgotten or not updated with new tests and removal of old
ones. We should have continuous integration test tied to Chatter
changes. Also automation of regeneration of Chatter golden test output.

### check_chat_html (private)

Python script to check for existence of a corresponding HTML file for
a data corpus.

Brian is currently running this manually on his personal clones of Git
repos.

TODO Incorporate into an automated checker step for all users.

### generate-from-chat (private)

Contains our `SCons` script ([more details on `SCons`](scons.md))
which generates Zip files. TODO fill in more details from the README.

TODO This script is very old and on the slow side, and possibly should
be redone not to use `SCons` at all.

TODO error handling

Uses a manually copied `cdcfile.py`. TODO properly handle the dependency.

### shiny-vagrant

I don't really know what this repo is used for. `franklinr` last committed on
November 12, 2019.

TODO The `README.md` is devoid of useful content and seems
to be based on rough internal notes in 2017. Can Brian or `franklinr`
update this document so that others can understand the status of this
project?

### normalize-ages (moved out)

A Perl script written in 2018 to normalize the age format in CHAT
files.

No longer actively used.

### save-word-html-pdf (private)

macOS AppleScript script to automatically convert a Word document to
HTML and PDF.

TODO Currently unused, because Brian (or whoever else edits the CHAT
manual or other manuals) manually generates the HTML and PDF from
Word. Ideally this should be automated.

### oai-rename-sync (private)

Haskell program to rename [CMDI](cmdi) files for ingestion by our [OAI
server](oai.md).

Leonid currently manually runs a script that includes

``` shell
rsync -avz --delete /web/data-cmdi/ oai.talkbank.org:/home/macw/data-cmdi
ssh macw@oai.talkbank.org /home/macw/.local/bin/oai-rename-sync
```

### check-chat-bookmarks (private)

A Perl script to check that

- all links from [our XML schema](xml-schema.md) to our HTML manuals
  actually exist;
- bookmarked material in the HTML manuals are actually linked to from
  the XML schema.

Currently Brian or Franklin runs this script periodically, and
manually inspects the output also in order to judge whether there was
an error.

TODO integrate into checker work flow

### Windows-CLAN (public)

Brian manages this repo. It seems not to have been updated since June
2017.

TODO Brian, why do we even have this repo if nobody is updating it
constantly?

### OSX-CLAN (public)

Brian manages this repo. It seems not to have been updated since June
2017.

TODO Brian, why do we even have this repo if nobody is updating it
constantly?

### reorganize-lena-data (public)

Shell scripts from 2016 to reorganize LENA data.

TODO Brian, are these of any further use?

### random-scripts (private)

Some Perl scripts used at some point. No changes since 2016.

TODO How many of these are still useful?

### reorder-chat-tiers (moved out)

From 2015. Perl script to reorder CHAT tiers for `%gra` etc.

TODO Do we still need these?

### gra-xquery (moved out)

From 2015. Uses XQuery.

TODO Seems irrelevant.

### cmdi2dc (moved out)

TODO Irrelevant.

### TalkBankDB-R (private)

This is John's, from December 2020.
