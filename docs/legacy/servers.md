# Our servers and their services

We provide services to the world by means of a number of servers.

## [SSL certificates](ssl.md)

## Data and documentation servers

### Our cloud servers

On our [cloud machines](cloud.md) we currently have for data and
documentation, there are multiple virtual hosts for each of these
machines.

### [OAI server](oai.md)

### [Non-cloud servers](non-cloud-servers.md)

## Password protection updates

### Users and passwords

Our private `users` Git repo contains a single file `users.txt`

### Directory protection using `0access.txt`

Any directory can be protected by providing `0access.txt`.

Note that directory protection is independent of corpus organization
using `0metadata.cdc`. Also, because of the way a corpus folder gets zipped
into a ZIP file that is at the same corresponding level, it is
impossible to "protect" a corpus. That is, one cannot have
`0metadata.cdc` and `0access.txt` in the same directory. To "protect"
a corpus, it is necessary to protect a parent directory above the
corpus directory.

## Stateful information

### `samtale-users`

`users.txt` in `homebank.talkbank.org:~macw/samtale-users` is modified
by a registration form at
https://samtalebank.talkbank.org/register/index.php that is
implemented using [PHP](php.md).

Currently Franklin periodically commits changes to this repo and
pushes.

TODO Save and push this information automatically?

### Web counters

TODO Are we still using Web counters to keep track of visitors to our
Web pages? If so, we should document those.

## [Shibboleth](shibboleth.md)

## TODO Auditing our servers

https://observatory.mozilla.org/
