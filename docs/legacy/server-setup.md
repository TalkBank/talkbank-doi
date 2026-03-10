# Server setup

## Create domain name, e.g. `bank.talkbank.org`

## Create empty Git repos: `bank-data` and `bank-site`

Use GUI to create those repos.

## Push initial contents of the repos

Create in staging area.

Add `.gitignore` as appropriate.

``` shellsession
$ cd ~/staging/repos
$ mkdir psychosis-data psychosis-site

$ cp samtale-data/.gitignore psychosis-data/
$ cp samtale-site/.gitignore psychosis-site/

$ cd ~/staging/repos/psychosis-data
$ git init
$ git add .
$ git commit -a -m 'Initial.'
$ git remote add origin ssh://git@git.talkbank.org:29418/TalkBank/psychosis-data.git
$ git push -u origin master

$ cd ~/staging/repos/psychosis-site
$ git init
$ git add .
$ git commit -a -m 'Initial.'
$ git remote add origin ssh://git@git.talkbank.org:29418/TalkBank/psychosis-site.git
$ git push -u origin master
```

## Add the new repos to `staging/scripts/config.py`

Add to `GIT_REPOS`:

``` text
    "psychosis-data": RepoInfo(
        url=f"{TALKBANK_GITBUCKET_BASE}/psychosis-data.git",
        root_dir="",
        git_clean=True,
    ),
    "psychosis-site": RepoInfo(
        url=f"{TALKBANK_GITBUCKET_BASE}/psychosis-site.git",
        root_dir="",
        git_clean=True,
    ),
```

Add to `HOSTS`:

``` text
    "psychosis": HostInfo(
        host=HOMEBANK_HOST,
        server=f"psychosis.{TALKBANK_DOMAIN}",
        root_dir=f"{WWW_HOME}/psychosis",
        cert_info=STAR_CERT_INFO,
        has_shib=True,
        has_browser=True,
        has_styles=True,
        has_data=True,
        deploy_excludes=[],
        repos=["psychosis-data", "psychosis-site"],
    ),
```

## `cdcs-to-csv/metadatas.py`

Add to `urls`:

``` text
    'psychosis-data': 'psychosis.talkbank.org',
```

## Apache configs on `homebank.talkbank.org`

$ sudo touch /etc/apache2/sites-available/{psychosis,psychosis-ssl,psychosis-passwords}.conf
$ sudo a2ensite psychosis psychosis-ssl psychosis-passwords

## Web folders on `homebank.talkbank.org` at `/var/www/`

``` shellsession
$ sudo mkdir /var/www/psychosis
```

## Build areas

``` shellsession
$ mkdir -p ~/staging/build/psychosis/{data,data-orig-xml,data-xml}
```

## `gandalf.talkbank.org` for sync to `media.talkbank.org`

Add `psychosis` to `bin/sync-media-dry-run.sh` and `bin/sync-media.sh`.

`bin/sync-all.sh` is not Franklin's.

Make symbolic link, e.g.

``` shellsession
$ ln -sf /Volumes/Other/psychosis ~/media/psychosis
```

## Force initial deploys

``` shellsession
$ ~/staging/scripts/deploy-1.py --repo psychosis-data --force
$ ~/staging/scripts/deploy-1.py --repo psychosis-site --force
$ ~/staging/scripts/deploy-1.py --repo styles --force
```

## Populate the repos
