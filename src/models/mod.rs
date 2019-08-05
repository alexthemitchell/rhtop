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
        format!("{}", bytesize::ByteSize(self.get_used_bytes()))
    }

    pub fn get_total_display(&self) -> String {
        format!("{}", bytesize::ByteSize(self.get_total_bytes()))
    }

    fn get_precise_used_percentage(&self) -> f64 {
        (self.used as f64 / self.total as f64) * 100.0
    }

    pub fn get_used_percentage(&self) -> u16 {
        self.get_precise_used_percentage().round() as u16
    }

    pub fn new(total_bytes: u64, used_bytes: u64) -> MemorySnapshot {
        MemorySnapshot {
            total: total_bytes,
            used: used_bytes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEN_GB_IN_BYTES: u64 = 1e+10 as u64;

    #[test]
    fn test_get_display() {
        let snapshot = MemorySnapshot::new(TEN_GB_IN_BYTES, 0);
        assert_eq!(snapshot.get_total_display(), "10.0 GB");
        assert_eq!(snapshot.get_used_display(), "0 B");
    }
}

#[derive(Debug)]
pub struct DiskSnapshot {
    name: String,
    fs_type: String,
    total: u64,
    available: u64,
}

impl DiskSnapshot {
    pub fn new(
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
}


#[derive(Debug)]
pub struct SwapSnapshot {
    total: u64,
    used: u64,
}

impl SwapSnapshot{
    pub fn new(total_bytes: u64, used_bytes: u64) -> SwapSnapshot {
        SwapSnapshot {
            total: total_bytes,
            used: used_bytes,
        }
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
    pub fn new(
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
}
