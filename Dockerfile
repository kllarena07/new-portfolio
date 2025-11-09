# Build stage
FROM rust:1.82 as builder
RUN rustup toolchain install nightly && rustup default nightly

WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install OpenSSH server
RUN apt-get update && \
    apt-get install -y openssh-server && \
    rm -rf /var/lib/apt/lists/*

# Create SSH directory
RUN mkdir /var/run/sshd

# Configure SSH for passwordless access with public key auth
RUN sed -i 's/#PasswordAuthentication yes/PasswordAuthentication no/' /etc/ssh/sshd_config && \
    sed -i 's/#PubkeyAuthentication yes/PubkeyAuthentication yes/' /etc/ssh/sshd_config && \
    sed -i 's/#PermitRootLogin prohibit-password/PermitRootLogin no/' /etc/ssh/sshd_config

# Create user with restricted shell
RUN useradd -m -s /usr/sbin/nologin -d /usr/local/bin portfolio

# Copy binary from builder
COPY --from=builder /app/target/release/portfolio-v2 /usr/local/bin/portfolio-v2

WORKDIR /app

# Copy frames_cache.bin file
COPY hikari-dance/frames_cache.bin /usr/local/bin/hikari-dance/frames_cache.bin

# Setup SSH for portfolio user
RUN mkdir -p /usr/local/bin/.ssh && \
    chmod 700 /usr/local/bin/.ssh && \
    chown -R portfolio:portfolio /usr/local/bin/.ssh

# Copy SSH authorized keys
COPY authorized_keys /usr/local/bin/.ssh/authorized_keys
RUN chmod 600 /usr/local/bin/.ssh/authorized_keys && \
    chown portfolio:portfolio /usr/local/bin/.ssh/authorized_keys

# Set the binary as the user's shell to run on SSH login
RUN usermod -s /usr/local/bin/portfolio-v2 portfolio

EXPOSE 22

# Generate SSH host keys and start SSH daemon
CMD ["/bin/bash", "-c", "ssh-keygen -A && /usr/sbin/sshd -D"]
