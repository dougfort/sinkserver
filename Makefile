SHELL := /bin/bash

export PROJECT = sinkserver

# Building containers

all: server

server:
	docker build \
		-f dockerfile.server \
		-t dougfort/sinkserver:latest \
		.
