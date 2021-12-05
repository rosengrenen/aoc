#!/bin/sh

docker build --file bench/Dockerfile . -t ghcr.io/rosengrenen/aoc-bencher:2020-rust
docker push ghcr.io/rosengrenen/aoc-bencher:2020-rust
