ARG RUST_VERSION=1.84.1
ARG APP_NAME=actix-webapp

FROM rust:${RUST_VERSION}-bullseye as builder
ARG APP_NAME
WORKDIR /app

################################################################################
# Create a DEV stage for building the application.
FROM builder AS dev

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install clang lld \
    && apt-get autoremove -y && apt-get clean -y

RUN cargo install cargo-watch

# Install rustfmt
RUN rustup component add rustfmt

# Copy the project files into the container
COPY . .

# Ensure dependencies are downloaded and compiled
RUN cargo fetch
RUN cargo build --locked --release && \
    cp ./target/release/$APP_NAME /bin/server

# CMD /app/target/release/${APP_NAME}
# CMD ["/bin/sh", "-c", "cargo run"]
# COPY /bin/server /bin/

CMD [ "/bin/server" ]
