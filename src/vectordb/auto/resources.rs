//! System resource detection for embedding-backend selection.

use sysinfo::System;

/// A snapshot of the host capabilities relevant to local model inference.
#[derive(Debug, Clone, Copy)]
pub struct SystemCapability {
    /// Total physical RAM in bytes.
    pub total_memory_bytes: u64,
    /// Number of physical/logical CPUs.
    pub cpu_count: usize,
    /// Whether a CUDA-enabled build is active.
    pub cuda_build: bool,
}

impl SystemCapability {
    /// Probe the current host.
    pub fn detect() -> Self {
        let mut sys = System::new();
        sys.refresh_memory();
        Self {
            total_memory_bytes: sys.total_memory(),
            cpu_count: num_cpus(),
            cuda_build: cfg!(feature = "candle-cuda"),
        }
    }

    /// Whether the host can comfortably run a local transformer embedder.
    ///
    /// Requires either a CUDA build or at least 4 GiB RAM and 4 CPUs.
    pub fn supports_local_embedding(&self) -> bool {
        const MIN_RAM: u64 = 4 * 1024 * 1024 * 1024;
        self.cuda_build || (self.total_memory_bytes >= MIN_RAM && self.cpu_count >= 4)
    }
}

fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
}
