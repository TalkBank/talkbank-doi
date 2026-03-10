# Cloud

## Provider

We are using the CMU Campus Cloud.

TODO Fill in details of what we are signed up for.

## Reasons

We had several reasons to move to the cloud:

- Reliability: not depending on our personal servers crashing and
  losing data or being stolen or hacked into, or used for personal
  purposes.
- The Mac was not a good server platform, because of
  - lack of availability of various tools
  - proprietary nature that blocks ease of virtualization and
    [containerization](docker.md)

## Console

There is a VMware vSphere Client for access to VMs.

The client is listed as “Campus Cloud vSphere Client HTML5” in the
Citrix portal at https://myapps.andrew.cmu.edu and there is
documentation at https://cmu.box.com/v/CampusCloudDocumentation
available.

The login is the bare Andrew ID and password.

## Our cloud machines

Not all of our servers are in the cloud, but some are.

### Data and documentation and services

There are two cloud machines we currently use for serving data and
documentation and various apps and services:

- `git.talkbank.org`
- `homebank.talkbank.org`

For these, we subscribe to a paid plan for [infrastructure as a
service
(IaaS](https://en.wikipedia.org/wiki/Infrastructure_as_a_service),
where we basically fully take over managing the machines, for full
flexibility.

We are responsible for updating the OS, installation software, etc.,
using our `root` access. There are no restrictions on us.

### [Media server](media-server.md)

The media server `media.talkbank.org` is located in a much more
restricted, "Cloud Plus" plan, in which certain services and commands
are configured for us, but we do not have full access and have to
request that specific commands be allowed to us.

TODO details

### Other cloud servers

TODO John has his own cloud machines?
`sla.talkbank.org` `sla2.talkbank.org`.

## [Non-cloud machines](non-cloud-servers.md)
