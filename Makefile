SHELL := /bin/bash

SPIKE2_HOST ?= spike2
SPIKE2_BUILD_DIR ?= /tmp/codetether-agent-build

.PHONY: build-cuda build-release deploy-spike2-cuda install-spike2-cuda status-spike2-cuda

build-cuda:
	cargo build --release --features candle-cuda

build-release:
	cargo build --release

deploy-spike2-cuda:
	rsync -az --delete --exclude target --exclude .git ./ $(SPIKE2_HOST):$(SPIKE2_BUILD_DIR)/
	ssh $(SPIKE2_HOST) 'set -euo pipefail; source $$HOME/.cargo/env; cd $(SPIKE2_BUILD_DIR); cargo build --release --features candle-cuda'
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
