# Python

Python 3 is used for many parts of our system.

## Versions

### macOS

On macOS (multiple versions of it), we prefer to use the Homebrew
version of Python 3, in order not to lag behind the default shipped
with the macOS installation.

As of 2020-04-10, Homebrew `python3` is at version 3.7.7.

Note: this is *not* the most current version of Python 3. I do not
know why Homebrew lags behind the official Python version which has
been at 3.8.x for months now, but that is OK. There is a discussion of
Homebrew Python packaging issues at
https://github.com/Homebrew/homebrew-core/issues/47274 which I am not
following.

Note that it is important to run Python 3 as `python3` (with
associated tools such as `pip3`), because the `python` command (with
associated tools such as `pip`) still points to the end-of-life Python
2, which should *never* be used.

### Ubuntu Linux

``` shellsession
$ sudo apt install python3 python3-venv python-is-python3
```

For our scripts:

``` shellsession
$ sudo apt install python3-click
$ sudo apt install python3-jinja2
```

## Development

Franklin has attempted to use [Mypy](http://mypy-lang.org/), an
optional type system, for all recent Python code, for better
ease of development and reliability of the software. Some of the older
Python code may not have `mypy` annotations retrofitted.

TODO
Use [Pyright](https://github.com/microsoft/pyright) to catch more problems.

TODO Set up project, testing.

TODO Assess SCons, deployment performance?

https://github.com/benfred/py-spy
