FROM rust:latest as builder
WORKDIR /usr/src/web_app
COPY . .
RUN cargo build --release

FROM debian:buster-slim as runner
COPY --from=builder /usr/src/web_app/target/release/take_notes_rs /usr/local/bin/take_notes_rs
ENV ROCKET_ADDRESS=0.0.0.0
CMD ["take_notes_rs"]