###############################################################################
## Builder
###############################################################################
FROM rust:1.69 AS builder

LABEL maintainer="Lorenzo Carbonell <a.k.a. atareao> lorenzo.carbonell.cerezo@gmail.com"

RUN rustup target add x86_64-unknown-linux-musl && \
    apt-get update && \
    apt-get install -y \
        --no-install-recommends\
        pkg-config \
        musl-tools \
        build-essential \
        cmake \
        libssl-dev \
        musl-dev \
        perl \
        pkg-config \
        && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src src

RUN cargo build --release --target x86_64-unknown-linux-musl && \
    cp /app/target/x86_64-unknown-linux-musl/release/publirs /app/publirs

###############################################################################
## Final image
###############################################################################
FROM alpine:3.18

ENV USER=app
ENV UID=10001

RUN apk add --update --no-cache \
            sqlite~=3.41 &&\
    rm -rf /var/cache/apk && \
    rm -rf /var/lib/app/lists*

# Copy our build
COPY --from=builder /app/publirs /app/

# Create the user
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/${USER}" \
    --shell "/sbin/nologin" \
    --uid "${UID}" \
    "${USER}" && \
    chown -R app:app /app

COPY migrations/ /app/migrations/
COPY templates/ /app/templates/
COPY assets/ /app/assets/

RUN mkdir -p /app/db && \
    chown -R app: /app

WORKDIR /app
USER app

CMD ["/app/publirs"]

