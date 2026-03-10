# SCons

[SCons](https://scons.org/) is a Python-based build tool we use for
generating ZIP files for [continuous
deployment](continuous-deployment.md) on [our servers](servers.md).

## Why SCons?

We adopted SCons a long time ago because it is a flxeible,
programmable way to manage task dependencies and also can runs tasks
in parallel.

## Our SCons script

Our script resides in the repo `generate-from-chat`.
