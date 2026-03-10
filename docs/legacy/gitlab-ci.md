# GitLab CI

https://docs.gitlab.com/ee/ci/

Intro from March 31, 2022
https://www.youtube.com/watch?v=havrTtBlEPo

`.gitlab-ci.yml`

default stages:

build
test
review
deploy

recommended to explicitly list "stage: ..."

each stage has multiple jobs in parallel

when does a job fire?

"image:" for Docker

variables
environment

cache for jobs and stages
tags

## GitLab Runner

https://docs.gitlab.com/runner/install/

https://docs.gitlab.com/runner/install/linux-repository.html

``` shellsession
curl -L "https://packages.gitlab.com/install/repositories/runner/gitlab-runner/script.deb.sh" | sudo bash

sudo apt-get install gitlab-runner
```

Register runner: https://docs.gitlab.com/runner/register/index.html

Go to Overview/Runners at https://gitlab.talkbank.org:8929/admin/runners to see runners.

``` shellsession
sudo gitlab-runner register --url https://gitlab.talkbank.org:8929/ --registration-token ixSWZaBnHFsWLcPFYM52
```

Enter an executor: I used `shell`.


Shared runner: https://docs.gitlab.com/ee/ci/runners/runners_scope.html#shared-runners

Enable shared runners for new projects.

## TODO Access credentials




Example command that requires ssh keychain:

``` shellsession
ssh macw@git.talkbank.org hostname
```

ssh keys: https://docs.gitlab.com/ee/ci/ssh_keys/
example: https://gitlab.com/gitlab-examples/ssh-private-key/

``` shellsession
gitlab-runner@git:~$ ssh-keygen -t ed25519 -C GitLab
Generating public/private ed25519 key pair.
Enter file in which to save the key (/home/gitlab-runner/.ssh/id_ed25519):
Enter passphrase (empty for no passphrase):
Enter same passphrase again:
Your identification has been saved in /home/gitlab-runner/.ssh/id_ed25519
Your public key has been saved in /home/gitlab-runner/.ssh/id_ed25519.pub
The key fingerprint is:
SHA256:TybhaIZ2FnRLFOYruPftIKMWiJEG8MYiICOWTbYBH7k GitLab
The key's randomart image is:
+--[ED25519 256]--+
|Bo==. ..*.       |
|*=oo+. = .       |
|+.+o. . +        |
|++ E o + o       |
|.o .+ B S o      |
|. ...* . =       |
|    ..+ . .      |
|    .o + o       |
|   ..   ..o      |
+----[SHA256]-----+
```



variables: https://docs.gitlab.com/ee/ci/variables/
use group variable

https://docs.gitlab.com/ee/ci/secrets/index.html

Vault
https://docs.gitlab.com/ee/ci/secrets/index.html#configure-your-vault-server

https://www.vaultproject.io/
TODO self-managed
https://www.vaultproject.io/downloads

``` shellsession
sudo snap install vault
```
