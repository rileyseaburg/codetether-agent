# Linux-hosted Windows Installer authoring tools; no Wine or Rust build required.
FROM rust:1.95-slim-bookworm
COPY docker/release/apt-https.sh /tmp/apt-https.sh
RUN sh /tmp/apt-https.sh && apt-get update && apt-get install -y --no-install-recommends \
    wixl msitools ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /workspace
ENTRYPOINT ["wixl"]