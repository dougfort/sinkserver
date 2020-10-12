SHELL := /bin/bash

export PROJECT = sinkserver

# Building containers

all: server client
server:
	docker build \
		-f dockerfile.server \
		-t dougfort/sinkserver:latest \
		.

client:
	docker build \
		-f dockerfile.client \
		-t dougfort/sinkclient:latest \
		.
