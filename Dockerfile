FROM --platform=linux/amd64 rust:latest
WORKDIR /usr/src/myapp
COPY . .
RUN apt-get update && apt-get install -y libssl-dev
RUN cargo build --release
