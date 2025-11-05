# Docker SSH Setup for portfolio-v2

This setup creates a Docker container that runs the portfolio-v2 binary when you SSH into it, without requiring a password.

## Setup Instructions

### 1. Add your SSH public key

Create an `authorized_keys` file with your public key:

```bash
cat ~/.ssh/id_rsa.pub > authorized_keys
# Or if you use a different key:
# cat ~/.ssh/id_ed25519.pub > authorized_keys
```

### 2. Build and run the container

```bash
docker-compose up -d --build
```

### 3. SSH into the container

```bash
ssh -p 2222 portfolio@localhost
```

The binary will run automatically when you connect!

## How it works

- The Dockerfile builds your Rust project in a multi-stage build
- SSH server is configured to allow public key authentication only (no passwords)
- The portfolio-v2 binary is set as the login shell for the `portfolio` user
- When you SSH in, the binary executes immediately

## Stopping the container

```bash
docker-compose down
```

## Troubleshooting

If you get a "connection refused" error, make sure:
- The container is running: `docker-compose ps`
- Port 2222 is not already in use: `lsof -i :2222`

If SSH complains about host key verification, remove the old key:
```bash
ssh-keygen -R "[localhost]:2222"
```
