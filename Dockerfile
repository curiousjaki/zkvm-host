FROM rust:1.86-bookworm AS builder

#Install Risc0

#RUN curl -L https://risczero.com/install | bash
#RUN ~/.risc0/bin/rzup install rust
#RUN ~/.risc0/bin/rzup install cpp
# Install Risc0
# Install the Protobuf-Compiler
RUN apt-get update
RUN apt-get -y install ninja-build
RUN apt-get -y install cmake

#ARG RISC0_VERSION=1.2.5
RUN git clone https://github.com/risc0/risc0.git
WORKDIR risc0
RUN git checkout release-1.2

RUN cargo install --force --path risc0/cargo-risczero
RUN cargo risczero --version
RUN cargo risczero build-toolchain


RUN apt-get install protobuf-compiler -y

#RUN cargo toolchain build rust
#RUN cargo install --path risc0/cargo-risczero


#    source /root/.bashrc && \
#    rzup install cargo-risczero --version ${RISC0_VERSION} --quiet

#COPY ./target/release/deps /app/target/release/deps
WORKDIR /app

#RUN cargo risczero build-toolchain
#Copy the source code to the container
COPY . .

#Fetch and build the server
RUN cargo fetch

RUN cargo build --release

#Copy the binary to /bin/server
RUN cp ./target/release/host /bin/host

FROM rust:1.86-bookworm AS final

#Install Risc0
ARG RISC0_VERSION=1.2.5
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

# What the container should run when it is started.
CMD ["/bin/server"]