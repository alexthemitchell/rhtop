extern crate bytesize;

#[derive(Debug)]
pub struct MemorySnapshot {
    total: u64,
    used: u64,
}

impl MemorySnapshot {
    pub fn get_used_bytes(&self) -> u64 {
        self.used
    }
    pub fn get_total_bytes(&self) -> u64 {
        self.total
    }

    pub fn get_used_display(&self) -> String {
        format!("{}",bytesize::ByteSize(self.get_used_bytes()))
    }

    pub fn get_total_display(&self) -> String {
        format!("{}",bytesize::ByteSize(self.get_total_bytes()))
    }

    fn get_precise_used_percentage(&self) -> f64 {
        (self.used as f64 / self.total as f64) * 100.0
    }

    pub fn get_used_percentage(&self) -> u16 {
        self.get_precise_used_percentage().round() as u16
    }
}

pub fn new_memory_snapshot(total_bytes: u64, used_bytes: u64) -> MemorySnapshot {
    MemorySnapshot {
        total: total_bytes,
        used: used_bytes,
    }
}

#[derive(Debug)]
pub struct DiskSnapshot {
    name: String,
    fs_type: String,
    total: u64,
    available: u64,
}

pub fn new_disk_snapshot(
    name: String,
    fs_type: String,
    total_bytes: u64,
    available_bytes: u64,
) -> DiskSnapshot {
    DiskSnapshot {
        name: name,
        fs_type: fs_type,
        total: total_bytes,
        available: available_bytes,
    }
}

#[derive(Debug)]
pub struct SwapSnapshot {
    total: u64,
    used: u64,
}

pub fn new_swap_snapshot(total_bytes: u64, used_bytes: u64) -> SwapSnapshot {
    SwapSnapshot {
        total: total_bytes,
        used: used_bytes,
    }
}

#[derive(Debug)]
pub struct SystemReport {
    disks: Vec<DiskSnapshot>,
    memory: MemorySnapshot,
    swap: SwapSnapshot,
}

impl SystemReport {
    pub fn get_memory(&self) -> &MemorySnapshot {
        &self.memory
    }
}

pub fn new_system_report(
    disks: Vec<DiskSnapshot>,
    memory: MemorySnapshot,
    swap: SwapSnapshot,
) -> SystemReport {
    SystemReport {
        disks: disks,
        memory: memory,
        swap: swap,
    }
}
