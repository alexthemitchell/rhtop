extern crate bytesize;

#[derive(Debug, Clone)]
pub struct MemorySnapshot {
    total: u64,
    used: u64,
}

impl Gaugable for MemorySnapshot {
    fn get_percentage(&self) -> f64 {
        (self.used as f64 / self.total as f64) * 100.0
    }

    fn get_name(&self) -> String {
        "Memory".to_string()
    }

    fn display<'a>(&self) -> String {
        format!(
            "{}/{} ({}%)",
            self.get_used_display(),
            self.get_total_display(),
            self.get_percentage(),
        )
    }
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

#[derive(Debug, Clone)]
pub struct DiskSnapshot {
    name: String,
    fs_type: String,
    total: u64,
    available: u64,
}

impl DiskSnapshot {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_fs_type(&self) -> &String {
        &self.fs_type
    }

    pub fn get_total_bytes(&self) -> u64 {
        self.total
    }

    pub fn get_available_bytes(&self) -> u64 {
        self.available
    }
    pub fn get_used_bytes(&self) -> u64 {
        self.get_total_bytes() - self.get_available_bytes()
    }

    pub fn get_available_display(&self) -> String {
        format!("{}", bytesize::ByteSize(self.get_available_bytes()))
    }

    pub fn get_total_display(&self) -> String {
        format!("{}", bytesize::ByteSize(self.get_total_bytes()))
    }

    pub fn description(&self) -> String {
        format!(
            "{} ({}) {}/{}",
            self.get_name(),
            self.get_fs_type(),
            self.get_used_bytes(),
            self.get_total_bytes(),
        )
    }

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

#[derive(Debug, Clone)]
pub struct SwapSnapshot {
    total: u64,
    used: u64,
}

impl SwapSnapshot {
    pub fn new(total_bytes: u64, used_bytes: u64) -> SwapSnapshot {
        SwapSnapshot {
            total: total_bytes,
            used: used_bytes,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SystemReport {
    disks: Vec<DiskSnapshot>,
    memory: MemorySnapshot,
    swap: SwapSnapshot,
    processors: Vec<ProcessorSnapshot>,
    time: std::time::SystemTime,
}

impl SystemReport {
    pub fn get_memory(&self) -> &MemorySnapshot {
        &self.memory
    }

    pub fn get_time(&self) -> &std::time::SystemTime {
        &self.time
    }

    pub fn get_processors(&self) -> &Vec<ProcessorSnapshot> {
        &self.processors
    }

    pub fn new(
        disks: Vec<DiskSnapshot>,
        memory: MemorySnapshot,
        swap: SwapSnapshot,
        processors: Vec<ProcessorSnapshot>,
        time: std::time::SystemTime,
    ) -> SystemReport {
        SystemReport {
            disks: disks,
            memory: memory,
            swap: swap,
            processors: processors,
            time: time,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProcessorSnapshot {
    usage_percent: f32,
    name: String,
}

impl ProcessorSnapshot {
    pub fn new(name: String, usage_percent: f32) -> ProcessorSnapshot {
        ProcessorSnapshot {
            name: name,
            usage_percent: usage_percent,
        }
    }
}

impl Gaugable for ProcessorSnapshot {
    fn get_percentage(&self) -> f64 {
        self.usage_percent as f64 * 100.0
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn display<'a>(&self) -> String {
        format!("{} - {}%", self.get_name(), self.get_percentage())
    }
}

pub trait Gaugable {
    fn get_name(&self) -> String;
    fn get_percentage(&self) -> f64;
    fn display(&self) -> String;
}
