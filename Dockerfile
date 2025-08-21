# docker build . -t ghcr.io/curiousjaki/zkvm-host:latest --platform linux/amd64 -f Dockerfile

FROM ghcr.io/curiousjaki/zkvm-base AS builder

RUN apt-get update
RUN apt-get install protobuf-compiler -y

WORKDIR /app

#RUN cargo risczero build-toolchain
#Copy the source code to the container
COPY . .

#Fetch and build the server
RUN cargo fetch

RUN cargo build --release

#Copy the binary to /bin/server
RUN cp ./target/release/host /bin/host

FROM ghcr.io/curiousjaki/zkvm-base AS final

#Install Risc0
#ARG RISC0_VERSION=1.2.5
#RUN curl -L https://risczero.com/install | bash
#RUN ~/.risc0/bin/rzup install cargo-risczero ${RISC0_VERSION}
#RUN curl -L https://risczero.com/install | bash && \
#    source /root/.bashrc && \
#    rzup install cargo-risczero --version ${RISC0_VERSION} --quiet

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/go/dockerfile-user-best-practices/
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# Copy the executable from the "build" stage.
COPY --from=builder /bin/host /bin/server

# Expose the port that the application listens on.
EXPOSE 50051
ENV RISC0_DEV_MODE=0
ENV RUST_LOG="info"

# What the container should run when it is started.
CMD ["/bin/server", "--", "--nocapture"]