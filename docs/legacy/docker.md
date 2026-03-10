# Docker

[Docker](https://www.docker.com/) enables containerized applications.

## Installation on [macOS](macos.md)

``` shellsession
$ brew install --cask docker
```

## Our use

TODO Eventually we should Dockerize stuff?

## Installation on Ubuntu

https://hub.docker.com/editions/community/docker-ce-server-ubuntu

Docker Engine (not yet Docker Desktop)

https://docs.docker.com/engine/install/ubuntu/

``` shellsession
sudo apt-get remove docker docker-engine docker.io containerd runc
sudo apt-get update
sudo apt-get install \
    ca-certificates \
    curl \
    gnupg \
    lsb-release
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu \
  $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
sudo apt-get update
sudo apt-get install docker-ce docker-ce-cli containerd.io -y
```

Docker Compose

``` shellsession
sudo curl -L "https://github.com/docker/compose/releases/download/1.29.2/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose
```

## TODO Docker Desktop extensions

## TODO Okteto?

## TODO Tanzu?
