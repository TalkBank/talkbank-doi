# Homebrew for macOS

[Homebrew](https://brew.sh/) "installs the stuff you need that Apple
didn’t". It is indispensable and required for any serious work on a
Mac.

## Installation

Official instructions: run

``` shellsession
$ /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install.sh)"
```

## List of some useful packages

TODO details

rsync
openssh
git

## Upgrade packages

``` shellsession
$ brew upgrade
```

Packages should be upgraded periodically, to get the latest versions
of important tools such as [Python](python.md) and [Git](git.md).

## Homebrew Cask

I recommend using [Homebrew
Cask](https://github.com/Homebrew/homebrew-cask) for installing macOS
applications because then it is easy to keep them up to date.

### Installing applications

Example:

``` shellsession
$ brew cask install google-chrome
```

### List of some useful applications


TODO important also, for installing Java, etc.
List important packages

adoptopenjdk8
docker
emacs
etrecheckpro
firefox
flash-npapi
flash-ppapi
github
google-chrome
java
phon
r
r-app
rstudio
vagrant
virtualbox
visual-studio-code
xquartz

### Upgrading applications

I recommend using https://github.com/buo/homebrew-cask-upgrade to
handle Homebrew Cask upgrades. Add the tap just once:

``` shellsession
$ brew tap buo/cask-upgrade
```

Then to upgrade all casks at once, I recommend

``` shellsession
$ brew cu -a -y
```

## Our standardized setup

TODO We don't currently have a standardized setup to provision for
everyone's Mac. We should.

TODO list important packages
