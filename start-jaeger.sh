#!/bin/sh

docker run -e COLLECTOR_OTLP_ENABLED=true -p16686:16686 -p14268:14268 jaegertracing/all-in-one:latest

