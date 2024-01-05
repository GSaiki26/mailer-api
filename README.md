# Mailer API
The Mailer API is a GRPC server written in Rust, responsible for sending and retriving email (made by it) using a Google Account.

# Usage
You may want to check the environment variables in `.env.example` file.

To run, simply run it with docker:
```sh
# Cloning and building yourself
docker build -t mailer-api .;
docker run -ti --env-file=.env -p 3000:50051 --name mailer-api mailer-api;

# Or using the pulled image in Docker hub
docker run -ti --env-file=.env -p 3000:50051 --name mailer-api gsaiki26/mailer-api:latest;
```

Or with rust/bin:
```sh
cargo build --release;
cargo run --release;
```
