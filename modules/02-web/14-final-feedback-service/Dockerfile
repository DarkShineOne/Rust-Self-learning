FROM rust:1-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p web-14-final-feedback

FROM debian:bookworm-slim
RUN useradd --create-home --uid 10001 feedback
USER 10001
WORKDIR /home/feedback
COPY --from=builder /app/target/release/web-14-final-feedback /usr/local/bin/feedback-api
EXPOSE 3000
ENTRYPOINT ["/usr/local/bin/feedback-api"]
