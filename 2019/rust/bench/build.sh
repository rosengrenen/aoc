#!/bin/sh

docker build --file bench/Dockerfile . -t ghcr.io/rosengrenen/aoc-bencher:2019-rust
docker push ghcr.io/rosengrenen/aoc-bencher:2019-rust
