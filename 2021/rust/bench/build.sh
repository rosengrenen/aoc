#!/bin/sh

docker build --file bench/Dockerfile . -t ghcr.io/rosengrenen/aoc-bencher:2021-rust
docker push ghcr.io/rosengrenen/aoc-bencher:2021-rust
