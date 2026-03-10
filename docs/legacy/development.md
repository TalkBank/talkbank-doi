# Data and documentation development

## Corpora

### Corpus metadata

Each "corpus" has a `0metadata.cdc` file at the top level of its
directory structure indicating that everything in that structures is
one corpus.

[DOI information for DataCite](datacite.md) is generated afresh or
kept updated as appropriate.

TODO Generation is done at a strange time.
TODO Updating is done purely manually at the moment.

### CHAT data

The CHAT data are maintained in a directory in some Git repo
`NAME-data`, which is processed by the continuous integration and
continuous deployment systems. TODO (Link to appropriate section.)

The CHAT data are checked for validity by Chatter. However, currently
invalid CHAT data are still allowed to be pushed into the Git repo and
deployed on the Web.

### CHAT metadata that is generated

#### `@PID` header

See [`@PID` documentation](pid.md).

#### `@Types` from `0types.txt`

If the immediate directory containing a CHAT file has a `0types.txt`
file containing a `@Types` header, that header is automatically
inserted into the CHAT file.

#### TODO other header information

### Other data

Image files and other auxiliary files can be part of a corpus.

TODO List what is possible and what is not.

### Documentation

The documentation for each corpus is located in the corresponding
`NAME-site` Git repository, which is processed by the continuous
integration and continuous deployment systems. TODO (Link to
appropriate section.)

There is a checker to ensure that documentation exists for each
corpus.

TODO It is error-prone and confusing having information for a corpus
spread out over different Git repositories. Ideally each corpus should
be its own self-contained Git repository and should include data as
well as documentation.

### Media
    The media files (audio or video) are currently *not* tracked in
    Git, and are managed by one person, Brian, by copying them to the
    appropriate places on the Mac `gandalf.talkbank.org`, and then
    from there synced to `media.talkbank.org` by means of his own
    authentication by VPN through the required CMU VPN server.

    There are checkers to make sure the directory structure and names
    of the media files match with those of the associated CHAT files.

    TODO (Link to discussion of the checkers.)
