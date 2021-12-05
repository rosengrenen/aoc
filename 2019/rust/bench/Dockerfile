FROM ghcr.io/rust-lang/rust:nightly

WORKDIR /app

RUN cargo install --git https://github.com/rosengrenen/aoc-bencher adapter-rust

COPY . .

RUN cargo bench --no-run

ENTRYPOINT ["adapter-rust"]
CMD ["adapter-rust"]
