# Automated deployment

It is important for deployment to be as automated and fast as
possible, because we often change information that triggers a change
in what the public should see, whether it is a CHAT file, a Web page,
a changed protection, or metadata that we serve as a provider.

We currently do automatic deployment as the final part of a monolithic
process that includes continuous integration. TODO The deployment part
should be separated out.

TODO asynchronous

## Deployment server

Our automated deployment server is `git.talkbank.org` because that is
where things are built and ready to copy over to the appropriate
servers.

## Configuration

Our Git repo `staging` contains deployment as part of its
functionality. The configuration aspect has mostly been extracted into
a Python module `config.py` that contains dictionaries for looking up
important information.

### Limitations

A major limitation of using a Python module is that everything depends
on a static configuration file. It would be ideal to have a dynamic
configuration file that could be loaded in real time. This would
enable creating a full testing environment as well as
general [containerization](docker.md).

I only briefly considered JSON/YAML as a configuration language but it
became clear that this was far too limited because of lacks of types
and programmability.

### TODO Future improvement

The [Dhall configuration language](https://dhall-lang.org/) looks like
a perfect fit for what we really want.

## TODO Testing

As mentioned earlier, the static configuration system makes it
impossible to test the system in isolation.


TODO details

TODO problem with permissions when syncing user and access information
to media.
