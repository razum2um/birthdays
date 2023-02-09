# Localhost environment

This has everything you need to run it locally.
Note, local environment doesn't come with logs/metrics/tracing
The app just follows twelve-factor principles

## Run

To build containers from source on localhost:

```
docker-compose up
```

and open `http://localhost:8000`

Note, the database state also lives in a container

## Tests

To run tests in containers (not to install dependencies and Rust environment)

```
docker-compose -f docker-compose.test.yml run app
```
## Rust development setup

See `src/birthday/README.md`

## Benchmark

Using `e5-2678` (12c / 8Gb) and [hey](https://github.com/rakyll/hey) load tool:

### GET endpoint

This is essentially measuring speed of nginx `proxy_cache`.
Note, this assumes the username exists, all responses are 200

``` 
hey -z 10s -q 500 -n 500 -c 500 -t 1 http://127.0.0.1:8000/hello/username

Summary:
  Total:        10.0045 secs
  Slowest:      0.1158 secs
  Fastest:      0.0001 secs
  Average:      0.0058 secs
  Requests/sec: 84091.2141

  Total data:   51318690 bytes
  Size/request: 61 bytes

Response time histogram:
  0.000 [1]       |
  0.012 [797958]  |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.023 [42407]   |■■
  0.035 [379]     |
  0.046 [46]      |
  0.058 [167]     |
  0.070 [115]     |
  0.081 [0]       |
  0.093 [34]      |
  0.104 [165]     |
  0.116 [18]      |


Latency distribution:
  10% in 0.0019 secs
  25% in 0.0034 secs
  50% in 0.0053 secs
  75% in 0.0075 secs
  90% in 0.0099 secs
  95% in 0.0118 secs
  99% in 0.0161 secs

Details (average, fastest, slowest):
  DNS+dialup: 0.0000 secs, 0.0001 secs, 0.1158 secs
  DNS-lookup: 0.0000 secs, 0.0000 secs, 0.0000 secs
  req write: 0.0000 secs, 0.0000 secs, 0.0864 secs
  resp wait: 0.0053 secs, 0.0001 secs, 0.0571 secs
  resp read: 0.0003 secs, 0.0000 secs, 0.0626 secs

Status code distribution:
  [200] 841290 responses
```
### PUT endpoint

This is measuring performance including database/app/nginx cache invalidation

```
hey -z 10s -q 500 -n 500 -c 500 -t 1 -m PUT -d '{ "dateOfBirth": "2000-01-01" }' -T 'application/json' http://127.0.0.1:${PORT}/hello/username

Summary:
  Total:        10.0332 secs
  Slowest:      0.1768 secs
  Fastest:      0.0091 secs
  Average:      0.0411 secs
  Requests/sec: 12102.3682


Response time histogram:
  0.009 [1]     |
  0.026 [105]   |
  0.043 [90252] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.059 [30013] |■■■■■■■■■■■■■
  0.076 [763]   |
  0.093 [29]    |
  0.110 [15]    |
  0.127 [202]   |
  0.143 [38]    |
  0.160 [1]     |
  0.177 [6]     |


Latency distribution:
  10% in 0.0366 secs
  25% in 0.0384 secs
  50% in 0.0404 secs
  75% in 0.0427 secs
  90% in 0.0455 secs
  95% in 0.0485 secs
  99% in 0.0583 secs

Details (average, fastest, slowest):
  DNS+dialup: 0.0001 secs, 0.0091 secs, 0.1768 secs
  DNS-lookup: 0.0000 secs, 0.0000 secs, 0.0000 secs
  req write: 0.0001 secs, 0.0000 secs, 0.0703 secs
  resp wait: 0.0409 secs, 0.0060 secs, 0.0808 secs
  resp read: 0.0000 secs, 0.0000 secs, 0.0451 secs

Status code distribution:
  [204] 121425 responses
```
