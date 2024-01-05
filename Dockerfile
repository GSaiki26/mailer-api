# BASE
FROM rust:1.75-alpine as base
WORKDIR /app

# Env
ENV TZ="America/Sao_Paulo"

# Update the layer
RUN apk upgrade --no-cache --update
RUN apk add --no-cache bash musl-dev tzdata
RUN date

# Configure the user
RUN adduser --disabled-password prod
RUN chown prod -R /app

# CHEF
FROM base as chef

# Install the layer's dependencies
RUN apk add --no-cache libressl-dev protoc protobuf-dev
RUN cargo install cargo-chef

# PLANNER
FROM chef as planner

# Prepare the recipe.
COPY ./Cargo.* ./
COPY ./src ./src
RUN cargo chef prepare --recipe-path ./recipe.json

# BUILDER
FROM chef as builder

# Install the project's dependencies
COPY --from=planner /app/recipe.json ./recipe.json
RUN cargo chef cook --release --recipe-path ./recipe.json

# Build the project
COPY ./proto ./proto
COPY ./Cargo.* ./
COPY ./src ./src
COPY ./build.rs .
RUN cargo build --release

# RUNNER
FROM base as runner
WORKDIR /app
USER prod

# Copy the project's dependencies
COPY --from=builder /app/target/release ./
CMD /app/app
