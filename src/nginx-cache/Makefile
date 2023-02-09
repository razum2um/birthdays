.PHONY: build build-cloud

build:
	docker build -t razum2um/openresty-localhost:v1 .
build-cloud:
	docker build -f Dockerfile-cloud -t registry.lunatic.cat/razum2um/openresty-birthdays:v1 .
