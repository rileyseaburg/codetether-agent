SHELL := /bin/bash

SPIKE2_HOST ?= spike2
SPIKE2_BUILD_DIR ?= /tmp/codetether-agent-build

# sccache with MinIO S3 backend. Export these from your local shell/CI secret
# store; the Makefile must not carry concrete service endpoints or credentials.
export SCCACHE_BUCKET      ?= sccache
export SCCACHE_REGION      ?= us-east-1
export SCCACHE_ENDPOINT    ?= $(CODETETHER_SCCACHE_ENDPOINT)
export SCCACHE_S3_USE_SSL  ?= off
export SCCACHE_S3_KEY_PREFIX ?= rust/
export AWS_ACCESS_KEY_ID   ?= $(CODETETHER_SCCACHE_ACCESS_KEY_ID)
export AWS_SECRET_ACCESS_KEY ?= $(CODETETHER_SCCACHE_SECRET_ACCESS_KEY)

.PHONY: build-cuda build-release build-cached build-windows build-windows-docker sccache-stats deploy-spike2-cuda install-spike2-cuda status-spike2-cuda install-dev

build-cuda:
	./script/cargo-sccache.sh build --release --features candle-cuda

build-release:
	./script/cargo-sccache.sh build --release

# Build with sccache pushing to MinIO (local dev writes the cache that Jenkins reads)
build-cached:
	./script/cargo-sccache.sh build --release

# Cross-compile for Windows (requires mingw-w64 toolchain: sudo apt-get install gcc-mingw-w64-x86-64 g++-mingw-w64-x86-64)
build-windows:
	./script/cargo-sccache.sh build --release --target x86_64-pc-windows-gnu

build-windows-docker:
	set -euo pipefail; \
	mkdir -p dist .docker-cache; \
	docker buildx build \
		--platform linux/amd64 \
		--file docker/release/windows.Dockerfile \
		--target artifact \
		--cache-from type=local,src=.docker-cache/windows \
		--cache-to type=local,dest=.docker-cache/windows-new,mode=max \
		--output type=local,dest=dist \
		.; \
	mkdir -p dist/windows; \
	cp dist/codetether.exe dist/windows/codetether.exe; \
	cp dist/codetether.exe dist/codetether-windows.exe; \
	rm -rf .docker-cache/windows; \
	mv .docker-cache/windows-new .docker-cache/windows

sccache-stats:
	sccache --show-stats

deploy-spike2-cuda:
	rsync -az --delete --exclude target --exclude .git ./ $(SPIKE2_HOST):$(SPIKE2_BUILD_DIR)/
	ssh $(SPIKE2_HOST) 'set -euo pipefail; source $$HOME/.cargo/env; cd $(SPIKE2_BUILD_DIR); ./script/cargo-sccache.sh build --release --features candle-cuda'
	ssh $(SPIKE2_HOST) 'set -euo pipefail; install -m 0755 $(SPIKE2_BUILD_DIR)/target/release/codetether /usr/local/bin/codetether; \
		sed -i "s/^CODETETHER_COGNITION_THINKER_CANDLE_DEVICE=.*/CODETETHER_COGNITION_THINKER_CANDLE_DEVICE=cuda/" /etc/default/codetether-agent; \
		sed -i "s/^CODETETHER_COGNITION_THINKER_CANDLE_CUDA_ORDINAL=.*/CODETETHER_COGNITION_THINKER_CANDLE_CUDA_ORDINAL=0/" /etc/default/codetether-agent; \
		systemctl restart codetether-agent'

install-spike2-cuda:
	scp ./target/release/codetether $(SPIKE2_HOST):/tmp/codetether.new
	ssh $(SPIKE2_HOST) 'set -euo pipefail; install -m 0755 /tmp/codetether.new /usr/local/bin/codetether; rm -f /tmp/codetether.new; \
		sed -i "s/^CODETETHER_COGNITION_THINKER_CANDLE_DEVICE=.*/CODETETHER_COGNITION_THINKER_CANDLE_DEVICE=cuda/" /etc/default/codetether-agent; \
		sed -i "s/^CODETETHER_COGNITION_THINKER_CANDLE_CUDA_ORDINAL=.*/CODETETHER_COGNITION_THINKER_CANDLE_CUDA_ORDINAL=0/" /etc/default/codetether-agent; \
		systemctl restart codetether-agent'

status-spike2-cuda:
	ssh $(SPIKE2_HOST) 'set -euo pipefail; \
		systemctl --no-pager --full status codetether-agent | sed -n "1,35p"; \
		echo "---"; \
		grep -E "^CODETETHER_COGNITION_THINKER_CANDLE_(DEVICE|CUDA_ORDINAL)=" /etc/default/codetether-agent; \
		echo "---"; \
		nvidia-smi --query-gpu=name,memory.used,memory.free,utilization.gpu --format=csv,noheader'
install-dev:
	./script/install-dev.sh && ./script/build-windows-dev.sh
