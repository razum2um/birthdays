# Birthday kubernetes definitions

## Deploy

This assumes you have a working cluster setup by `terraform`
Let's apply application manifests

Database connectivity follows the best security practices and goes via `cloudsql-docker/gce-proxy` sidecar proxy

`Openresty` caching layer has to live as a sidecontainer to talk to upstream as `127.0.0.1` and not to resolve internal DNS.
TODO: extract it to a separate deployment by using a lua module which updates DNS entries (like `ingress` does)

```
make tunnel
# in different shell:
kubectl create secret generic db-secret --from-literal=PG.USER=... --from-literal=PG.PASSWORD=... --from-literal=PG.DBNAME=...
make apply
```

Prepare rest of services:

```
helm repo add ingress-nginx https://kubernetes.github.io/ingress-nginx
helm repo add jetstack https://charts.jetstack.io
helm repo add prometheus-community https://prometheus-community.github.io/helm-charts
helm repo update

helm install nginx-ingress nginx-stable/nginx-ingress
# get public ip, assign to the domain
kubectl get service nginx-ingress-nginx-ingress -ojson | jq -r '.status.loadBalancer.ingress[].ip'

helm install cert-manager jetstack/cert-manager --namespace cert-manager --create-namespace --version v1.11.0 --set installCRDs=true

helm -n monitoriing install prometheus prometheus-community/prometheus
```

## Test

This scripts emulates the write-then-read scenario
and catches cache inconsistency between different caching hosts deployed in cloud cluster.
It reports progress and runs indefinetely until any error

```
docker-compose run sanity-check
```