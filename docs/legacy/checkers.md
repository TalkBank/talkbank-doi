# Checkers

## Test Chatter

## Allowed file names

I believe that we are currently informally requiring that certain file
names not contain spaces or Unicode characters. Some scripts may break
if this is violated. TODO Enforce this?

## Checking CHAT files for validity

Chatter is used to check CHAT files and also generate XML.

### Avoid recomputation

Chatter uses time stamps to determine whether to rerun on a CHAT file
to generate a new XML file. This is not ideal:

- It is possible to touch a CHAT file without modifying its contents.
  TODO A hash-based check might be better.
- When a new version of Chatter is released, semantics may change that
  should force rerunning Chatter on files that were processed by an
  older version of Chatter. TODO Chatter should check Chatter version
  information and/or XML Schema version information.

### When is checking done?

Currently, users are expected to check their work on their own.

When Chatter is run on the staging server, checking is done but that
is a very late time to be doing checking.

TODO Ideally, the checking should be done before the user does a `git
push`, possibly as part of a Git hook.

Install `pre-commit` with `brew install pre-commit`.

Put in `.pre-commit-config.yaml` for repos.


## CHECK

CHECK, part of CLAN, is a checker. However, our process does not
currently require use of CHECK **at all**, because we have settled on
Chatter as the final arbiter of validity.

TODO Should CHECK be used as part of our process?

## Password-protection checking

We currently have no checking that password-protected areas actually
are correctly password-protected. We only trust that the deployment
mechanism correctly generates the desired protections (by means of
updating the password files and the Apache config files for each
site).

TODO Do we need to add a checker for post-deployment, or do we
consider the deployment mechanism to be foolproof?

### Multiple levels of folder protection are possible

We support having multiple levels of folder protection when accessing
our data or media, but this results in people having to type more than
one set of user/password pairs to get to what they want.

TODO Make this illegal? Doesn't really make sense, discussed on
Thursday, 2020-05-07.

## [Web site checkers](web-site-checkers.md)

## Corpus HTML correspondence with data

Currently we check that each data corpus has HTML documentation in the
appropriate corresponding site repository.

TODO

`check_chat_html` from GitHub.

Brian runs this manually on a CHAT directory and a site directory.

## XML Schema links to CHAT manual

TODO

## [Update CHAT @Types in place](update-chat-types.md)

## Interdependencies between data and site

Hence having to handle both.
