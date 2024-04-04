FROM rust:latest

WORKDIR /usr/src/companion-backend

COPY . .

# compiles the program
RUN cargo build

# runs the program
CMD cargo run

