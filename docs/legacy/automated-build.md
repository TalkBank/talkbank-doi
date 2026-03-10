# Automated build

Automated building of artifacts is a vital part of our system, given
the amount of code, data, documentation, and configuration information
we have.

TODO Make sure to have a complete list of what is built, when, and
where.

## Code

Franklin strives to organize all his code to be self-contained so that
it can be built automatically.

TODO Give examples.

## Documentation of XML Schema

Our [XML Schema](xml-schema.md) is documented through manual
generation of https://talkbank.org/software/xsddoc/index.html by
Franklin using a script `generate-xsddoc.sh` that runs our paid-for
license for Oxygen XML Editor using its `schemaDocumentation.sh`
script, which we are *not* allowed to run in an automated fashion.

## XML version of CHAT corpora

[Chatter](chatter.md) is used to generate XML versions of CHAT
corpora.

## ZIP files of CHAT corpora

An `SCons` script is used to automatically generate

- ZIP files of the original CHAT versions of corpora
- ZIP files of the generated XML versions of corpora

## ZIP files of `mor` information

TODO Shell script.

## Apache configuration files

Some, but not all Apache configuration files are generated
automatically.

TODO List what is still manual, e.g., `talkbank-site`?

TODO Describe the automation.
