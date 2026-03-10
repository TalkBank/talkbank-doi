# BitBucket

[BitBucket](https://bitbucket.org/) is a version control (e.g. Git)
host.

## Why we use BitBucket

We use BitBucket for one reason and one reason only: years ago, it
offered free *private repositories* at a time when nobody else did,
and so we used it in order to host `childes` and `talkbank`
repositories.

We eventually moved all of our data/site repositories to our own
self-hosted [Git server](git-server.md), for one reason and one reason
only: size limitations on BitBucket.

## Why we may no longer need BitBucket

[GitHub](github.md) as of April 14, 2020 allows unlimited private
repositories for free for teams.

TODO migrate to GitHub?

## Our team

We have a team called `talkbank`: https://bitbucket.org/talkbank

TODO
And why did Brian add so many external users who have access to
anything at all on BitBucket??

## Our repositories

TODO describe all

git@bitbucket.org:talkbank/brian-cheat-sheet
git@bitbucket.org:talkbank/cdcs-to-csv
git@bitbucket.org:talkbank/certificates
git@bitbucket.org:talkbank/contribute
git@bitbucket.org:talkbank/contribute-setup
git@bitbucket.org:talkbank/etc-apache2-childes
git@bitbucket.org:talkbank/etc-apache2-homebank
git@bitbucket.org:talkbank/etc-apache2-talkbank
git@bitbucket.org:talkbank/git-rename-case
git@bitbucket.org:talkbank/git.talkbank.org-web-setup
git@bitbucket.org:talkbank/gra-cgi
git@bitbucket.org:talkbank/homebank.talkbank.org-web-setup
git@bitbucket.org:talkbank/oai-setup
git@bitbucket.org:talkbank/quicktime-hinter-stuff
git@bitbucket.org:talkbank/refmovieperl
git@bitbucket.org:talkbank/rename-by-age
git@bitbucket.org:talkbank/rsyncd-setup
git@bitbucket.org:talkbank/shibboleth-childes
git@bitbucket.org:talkbank/shibboleth-talkbank

`for i in *; do git -C $i branch -m master main; done`

https://docs.github.com/en/get-started/importing-your-projects-to-github/importing-source-code-to-github/importing-a-git-repository-using-the-command-line

Fix staging.

``` shellsession
for i in cdcs-to-csv certificates gra-cgi; do git -C ~/staging/repos/"$i" remote set-url origin "git@github.com:TalkBank/$i.git"; done
for i in cdcs-to-csv certificates gra-cgi; do git -C ~/staging/repos/"$i" branch -m master main; done
for i in cdcs-to-csv certificates gra-cgi; do gres master main ~/staging/repos/"$i"/.git/config; done
```
