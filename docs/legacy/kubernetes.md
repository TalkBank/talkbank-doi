# Kubernetes

[Kubernetes](https://kubernetes.io/) is "an open-source system for
automating deployment, scaling, and management of containerized
applications".

TODO We may want to evaluate using it in conjunction with
[Docker](docker.md).


## Install

https://kubernetes.io/docs/tasks/tools/install-kubectl-linux/#install-using-native-package-management

``` shellsession
sudo snap install kubectl --classic
```

## minikube

https://minikube.sigs.k8s.io/docs/

``` shellsession
curl -LO https://storage.googleapis.com/minikube/releases/latest/minikube_latest_amd64.deb
sudo dpkg -i minikube_latest_amd64.deb
```

## Helm

https://helm.sh/

``` shellsession
$ sudo snap install helm --classic
```

## TODO GitLab

https://docs.gitlab.com/charts/quickstart/index.html

``` shellsession
minikube start
```

``` shellsession
helm repo add gitlab https://charts.gitlab.io/
```

TODO the following is wrong? DNS stuff?

``` shellsession
helm install gitlab gitlab/gitlab \
  --set global.hosts.domain=gitlab.talkbank.org \
  --set certmanager-issuer.email=franklinchen@franklinchen.com
```
