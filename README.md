# sinkserver

## Synopsis

HTTP Server which acts as a sink

for HTTP GET this program duplicates the functionality of the `example` service
for HTTP POST program will read the request body into a hash function to create a digest
the digest is returned as the body of the response

## Usage

to create docker container `dougfort/sinkserver:latest` run make server

to run:  `docker run -p 3000:3000 --name=sinkserver --rm dougfort/sinkserver:latest`
to stop: `docker stop sinkserver`

to test GET: `curl http://localhost:3000/hello`
to test POST: `curl -X POST -H 'Content-Type: application/octet-stream' -d xxxxxxxxxx localhost:3000/upload | hexdump -C`

### Environment Variables

SINKSERVER_HOST: hostname to bind to : default `0.0.0.0`
SINKSERVER_PORT: port to listen on   : default `3000`