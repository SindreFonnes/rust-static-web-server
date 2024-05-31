name := rust-static-web-server
sha := $(shell git rev-parse HEAD)
version := 0

image	:= $(name):$(sha)


build: 
	docker build \
		-t $(image) \
		-t $(image)-$(version) .

run:
	docker run --rm -it -p 8080:8080 $(image)

run-debug:
	docker run --rm -it --entrypoint=sh $(image)