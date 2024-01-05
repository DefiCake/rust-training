use std::{fs::File, io::Write};

pub type Memory<const SIZE: usize> = Box<[u8; SIZE]>;

/// Maximum memory in MiB
pub const FUEL_MAX_MEMORY_SIZE: u64 = 64;

/// Maximum VM RAM, in bytes.
pub const VM_MAX_RAM: u64 = 1024 * 1024 * FUEL_MAX_MEMORY_SIZE;

/// Size of the VM memory, in bytes.
#[allow(clippy::cast_possible_truncation)]
pub const MEM_SIZE: usize = VM_MAX_RAM as usize;

fn main() {
    let mut file = File::create(String::from("blob")).expect("Could not create file");

    let memory: Vec<u8> = vec![0; MEM_SIZE];

    file.write_all(&memory).expect("Could not write");
}