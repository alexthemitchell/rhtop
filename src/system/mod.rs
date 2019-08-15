extern crate sysinfo;

use std::ffi::OsStr;
use sysinfo::{DiskExt, ProcessorExt, System, SystemExt};

use super::models::{DiskSnapshot, MemorySnapshot, ProcessorSnapshot, SwapSnapshot, SystemReport};

pub fn new_report() -> SystemReport {
    let mut sys = System::new();
    sys.refresh_all();
    sys.refresh_all();
    let now = std::time::SystemTime::now();
    return SystemReport::new(
        disks_from_sys(&sys),
        memory_from_sys(&sys),
        swap_from_sys(&sys),
        processors_from_sys(&sys),
        now,
    );
}

fn osstr_to_str(osstr: &OsStr) -> String {
    match osstr.to_str() {
        Some(str) => str.to_string(),
        None => "".to_string(),
    }
}

fn fs_type_for_disk(d: &sysinfo::Disk) -> String {
    match std::str::from_utf8(&d.get_file_system()) {
        Ok(str) => str.to_string(),
        Err(_) => "unknown".to_string(),
    }
}

fn disks_from_sys(sys: &System) -> Vec<DiskSnapshot> {
    sys.get_disks()
        .iter()
        .map(|d| {
            DiskSnapshot::new(
                osstr_to_str(d.get_name()),
                fs_type_for_disk(d),
                d.get_available_space(),
                d.get_total_space(),
            )
        })
        .collect()
}

fn memory_from_sys(sys: &System) -> MemorySnapshot {
    // sysinfo has data in mB
    MemorySnapshot::new(sys.get_total_memory() * 1000, sys.get_used_memory() * 1000)
}

fn swap_from_sys(sys: &System) -> SwapSnapshot {
    SwapSnapshot::new(sys.get_total_swap(), sys.get_used_swap())
}

fn processors_from_sys(sys: &System) -> Vec<ProcessorSnapshot> {
    sys.get_processor_list()
        .iter()
        .map(|p| ProcessorSnapshot::new(p.get_name().to_string(), p.get_cpu_usage()))
        .collect()
}

/*
fn get_status<'a>() -> SystemReport {
    // Network data:
    println!("input data : {} B", sys.get_network().get_income());
    println!("output data: {} B", sys.get_network().get_outcome());

    // Components temperature:
    for component in sys.get_components_list() {
        println!("{:?}", component);
    }

    // Number of processors
    return new_report();
}

*/
