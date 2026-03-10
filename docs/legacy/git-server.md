# Our own Git server

We currently host a private Git server on `git.talkbank.org`, running
our own [manual installation of GitBucket](gitbucket.md).


## Why do we have our own Git server?

space and cost limitations
performance

TODO

## TalkBank team

Most of our Git repos are owned by the TalkBank team.

TODO List every repo we have, and make sure to keep up to date.

## External users

Brian has created some number of external users so that they can view,
clone, and/or push to certain Git repos.

TODO Brian list all the external users and why they have access?

## Space usage

We may need to periodically prune certain repos because they can grow
very large as a result of very frequent commits that are transient in
nature.

TODO Would it be sensible to tag snapshots of certain repos in order
to retain those snapshots while squashing other commits?

## TODO Future migration to GitLab?

I believe that eventually, we should migrate from GitBucket to our own
hosting of the open source version of [GitLab](gitlab.md), because
GitLab is much more popular and supported and has many features that
are sorely missing in GitBucket.

This would require changing some URLs, of course, as well as
rebuilding users and passwords and access permissions.
