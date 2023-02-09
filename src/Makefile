# load tool: https://github.com/rakyll/hey

.PHONY: ulimit load-get load-put reset preload
PORT ?= 8000
USERNAME ?= username

load-get: ulimit preload
	hey -z 10s -q 500 -n 500 -c 500 -t 10 http://127.0.0.1:${PORT}/hello/${USERNAME}
load-put: ulimit
	hey -z 10s -q 500 -n 500 -c 500 -t 10 -m PUT -d '{ "dateOfBirth": "2000-01-01" }' -T 'application/json' http://127.0.0.1:${PORT}/hello/${USERNAME}
preload:
	curl -X PUT 'http://localhost:${PORT}/hello/${USERNAME}' -H 'Content-Type: application/json' -d '{ "dateOfBirth": "2000-12-31" }'
ulimit:
	ulimit -n 4096
reset:
	docker-compose rm -s -f -v && docker-compose build && docker-compose up
