FROM ghcr.io/rust-lang/rust:nightly as builder

WORKDIR /app

RUN cargo install --git https://github.com/rosengrenen/aoc-bencher adapter-rust

COPY . .

RUN cargo bench --no-run

RUN cp $(find target/release/deps -type f | grep aoc2020 | grep -v '.d$') aoc2020-bench

FROM ubuntu:20.04

WORKDIR /app

RUN apt-get update \
    && apt install -y libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/aoc2020-bench /app/aoc2020-bench
COPY --from=builder /usr/local/cargo/bin/adapter-rust /usr/local/bin/adapter-rust

ENTRYPOINT ["adapter-rust", "./aoc2020-bench"]
CMD ["adapter-rust", "./aoc2020-bench"]
