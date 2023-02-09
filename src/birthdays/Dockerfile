FROM rust:slim-bookworm as rust-builder
WORKDIR /usr/src/app
COPY ./Cargo.toml .
COPY ./Cargo.lock .
RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/main.rs
RUN cargo build --release
RUN rm -rf ./src
COPY ./src ./src
COPY ./sql ./sql
RUN touch -a -m ./src/main.rs
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=rust-builder /usr/src/app/target/release/birthdays /usr/local/bin/
WORKDIR /usr/local/bin
EXPOSE 8080
CMD ["birthdays"]
