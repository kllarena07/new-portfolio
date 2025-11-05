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
    sed -i 's/#PermitRootLogin prohibit-password/PermitRootLogin yes/' /etc/ssh/sshd_config

# Create user
RUN useradd -m -s /bin/bash portfolio

# Copy binary from builder
COPY --from=builder /app/target/release/portfolio-v2 /usr/local/bin/portfolio-v2

# Setup SSH for portfolio user
RUN mkdir -p /home/portfolio/.ssh && \
    chmod 700 /home/portfolio/.ssh && \
    chown -R portfolio:portfolio /home/portfolio

# Set the binary as the user's shell to run on SSH login
RUN usermod -s /usr/local/bin/portfolio-v2 portfolio

EXPOSE 22

# Generate SSH host keys and start SSH daemon
CMD ["/bin/bash", "-c", "ssh-keygen -A && /usr/sbin/sshd -D"]
