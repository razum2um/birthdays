# Birthdays

[![CI](https://github.com/razum2um/birthdays-draft/actions/workflows/master.yml/badge.svg)](https://github.com/razum2um/birthdays-draft/actions/workflows/master.yml)

A simple application counting how many days until the birthday, built to demonstrate the best SDE/SRE practices
## Try

Publicly accessible version: use [SwaggerUI](https://birthdays.razum2um.me/swagger-ui/) or `curl` to test the app

```
curl -X PUT 'https://birthdays.razum2um.me/hello/username' -H 'Content-Type: application/json' -d '{ "dateOfBirth": "2000-01-01" }'
curl 'https://birthdays.razum2um.me/hello/username'
```
## Quick local try

Run a `docker-compose` in the top-level directory (using prebuilt containers) to get it locally:

```
docker-compose up
```

and open http://localhost:8000 or use `curl` to test the app

```
curl -X PUT 'http://localhost:8000/hello/username' -H 'Content-Type: application/json' -d '{ "dateOfBirth": "2000-01-01" }'
curl 'http://localhost:8000/hello/username'
```
## Features

The app deployment adresses the following SRE aspects:

- scalability: `Kubernetes` autoscaling based on runtime metrics (`Prometheus`, todo: `Keda`)
- performance: see benchmarks in `src` (GET: 84k RPS; PUT: 12k RPS), todo: `Grafana` to draw for the deployment
- redundancy: `Kubernetes` runs in multiple zones, `CloudSQL` HA setup
- resilience/no-downtime production deployment: `RollingUpdate` strategy to rotate the pods
- fault tolerance: app is able to retry database failures (e.g. during failover), it has a separate caching layer, which serves a valid cache (even is the app upstream is down)
- security: `ProxySQL` to connect to database, the `Kubernetes` cluster is private, both nodes and master, using bastion host to apply
- monitoring and alerting: `Prometheus` + `Alertmanager` tuned to notify developer about performance degradation, todo: `Jaeger` tracing
- deployment process: CI/CD process using `Github Actions`, todo: a test runner / deployer inside the cluster, `ArgoCD`
- rollback plan: rolling update on the previous version
- documentation: using `SwaggerUI`, also see this repo structure and other READMEs

## System diagram

![diagram](https://raw.githubusercontent.com/razum2um/birthdays/master/diagram.png)

## Compoments

- app: a web service in Rust (actix-web)
- openresty: a caching layer which relies on HTTP semantics and serves from cache
- memcached: distributes cache, sharded between multi-node deployment
- database: HA PostgreSQL
- infrastructure in GCP:
  - Kubernetes (private network and master, bastion host, cert-manager, ingress-nginx)
  - Cloud SQL (securely connecting via `proxy sql` sidecar)
  - Prometeus & Alertmanager

## Structure

Each layer has it's own `README.md`

```
├── src              # app code, how to run it locally
│   ├── birthdays    # web service, how to develop it locally
│   ├── nginx-cache  # caching proxy layer
├── terraform        # infrastructure, how to deploy infra in GCP
└── k8s              # cluster definitions, how to deploy app into cluster
```

## Local development & tests

See `src/README.md`
