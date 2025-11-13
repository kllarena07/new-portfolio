sudo dnf install -y libcap
sudo setcap 'cap_net_bind_service=+ep' target/release/portfolio-v2
getcap target/release/portfolio-v2
