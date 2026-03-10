# XML Schema for TalkBank

## History and purpose

The XML Schema for TalkBank is located at
https://talkbank.org/software/talkbank.xsd and has been developed
since 2002 starting with Romeo's work and extended by Franklin and
Greg (for Phon purposes).

## Versioning

We don't really have a meaningful "semantic versioning" system for
versioning the schema.

TODO Ideally there would be a way to signal compatibility between
different versions of the schema.

## Documentation

### Automatically generated overview

The schema has rudimentary documentation at
https://talkbank.org/software/xsddoc/index.html using a
[script](automated-build.md#documentation-of-xml-schema). What this
documentation does is provide

- some graphical representations of the
schema elements, attributes, and types; and
- clickable HTML links to appropriate places in the HTML versions of

  - the [CHAT manual](https://talkbank.org/manuals/CHAT.html) and
  - the [MOR manual](https://talkbank.org/manuals/MOR.html).

### Tutorial and reference manual

TODO Write up a human-friendly tutorial and reference manual for the
schema. John was asking for this but we never got around to it.

## Irregularities

There are irregularities in the way some elements and types are
defined or factored out, because of legacy reasons. It does not
necessarily seem practical to "fix" them, however, because of
compatibility issues for consumers of existing XML. But they should at
least be documented.
