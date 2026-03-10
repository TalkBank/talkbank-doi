# DataCite DOI management

We have an arrangement (TODO Brian details?) to use
[DataCite](https://datacite.org/) as our provider for DOIs for our
work.

## DataCite Fabrica

We use https://doi.datacite.org/ to manually manage some aspects
of our DOIs, such as marking as inactive.

## Python package to update DOIs, `cdcs-to-csv`

The repo `cdcs-to-csv` has a main driver script `cdcs_to_csv.py` that
updates `ourcsv.py` and sends requests to the DataCite server using an
API.

### Create new DOI

If a DOI is not present yet in a `0metadata.cdc` file yet, then one is
minted by the script.

### Update DOI

Most updated DOIs are a result of changing the URL of a corpus, but
other metadata changes are tracked as well.

Franklin runs `cdcs_to_csv.py` manually, which modifies
`0metadata.cdc` files in place inside all relevant Git repos in the
staging area, and then he commits and pushes the changes to
`ourcsv.py`.

TODO Move to integration time after validating everything else?

## Update documentation

TODO Move to development time? Difficult because of need to coordinate
changes in both `NAME-data` and `NAME-site` Git repositories
simultaneously.

## Implementation details

Python 3 was used. The program is split up in to modules.

### `cdcs_to_csv`

The main driver is intended to be used at the command line, i.e.

``` sh
$ cd /path/to/cdcs-to-csv
$ ./cdcs_to_csv
```

### `cdcfile`

Handles parsing of `0metadata.cdc` files as well as modifying them
with any newly minted DOIs.

### `credentials`

Login credentials for submitting of requests to the DataCite server
https://mds.datacite.org as required.

TODO It is bad practice to store passwords in source code!! See
[security](security.md).

### `datacite`

A handcrafted rudimentary Python API for accessing DataCite which
constructs and executes `curl` commands.

### `ourcsv.py`

Reading and writing to our internal `output.csv` file that keeps track
of all our DOIs.

This "database" is not in fact necessarily consistent with what is on
the DataCite server, for several reasons:

- An unexpected error during update could leave something improperly
  updated.
- TODO I believe Brian has manually created DOIs through their Web GUI that
  are not reflected in any of our metadata files?
- TODO Corpora that disappear simply no longer show up in `output.csv` but
  still exist on the DataCite server, and Brian periodically gets
  warning emails about them, I believe, and has to manually go in and
  mark the DOIs as "inactive" (since public DOIs can never be deleted
  once minted).

TODO Automate full consistency so that Brian is not in the loop?

### `check-isbns.py`

(No longer used.)

## Checking for valid `0metadata.cdc`

TODO There is no checking for validity of DataCite-relevant
information in 0metadata.cdc. This can result in crashing the DOI
updater script.

## TODO Derived corpus

Own DOI.

CHAT files.

should create new metadata files

CMDI?
check validity
names are not consistent

write script to clean up stranded DOIs

Missing DOI:

``` shellsession
grep --include 0metadata.cdc -L -r '^DOI' ~/staging/repos/*-data
```
