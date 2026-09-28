#![no_std]
#![no_main]

//  TODO: find better way to import the kernel that to create its exact copy in byte array

mod boot_info;

pub mod kernel_binary;

//use libpages::{PageTable, PdptEntry, Pml4Entry, PtEntry, PtKernelBits};
use uefi::{allocator::Allocator, boot::{AllocateType, allocate_pages}};

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

use uefi::{print, println};

use core::{num::NonZero, panic::PanicInfo, ptr::NonNull, time::Duration};

use uefi::{boot::{MemoryAttribute, MemoryType, get_handle_for_protocol, stall}, mem::memory_map::MemoryMap, prelude::*};

use libelf::{Elf64, api::{ElfParseError, HeaderEntryType}};

use crate::kernel_binary::KERNEL_BINARY;


#[entry]
fn main() -> Status {

    if let Err(e) = uefi::helpers::init() {
        panic!("failed to initialize UEFI helpers: {}", e);
    }

    let elf = match Elf64::parse(&KERNEL_BINARY.data) {
        Ok(elf) => elf,
        Err(e) => panic!("ELF file parsing failed: {e:?}"),
    };

    for head in elf.program_headers() {
        println!("Header:");
        println!("    type: {:?}", head.entry_type());
        println!("    flags: {:?}, loadable: {}, {:?}", head.flags(), head.is_loadable(), head.permissions());
    }



    /*let pml4 = unsafe { allocate_table().cast::<PageTable<Pml4Entry>>().as_mut() };

    let pdpt = unsafe { allocate_table().cast::<PageTable<PdptEntry>>().as_mut() };*/




    loop {
        boot::stall(Duration::from_secs(1));
    }
}


fn allocate_table() -> NonNull<[u64; 512]> {
    let page = match boot::allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, 1) {
        Ok(page) => page.cast::<[u64; 512]>(),
        Err(e) => panic!("allocation failed: {e}"),
    };

    unsafe { page.write_bytes(0, size_of::<[u64; 512]>()) }

    page
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {

    println!("panic: {}", info.message());

    if let Some(location) = info.location() {
        println!("\tat {}", location);
    }

    println!("halting the system...");

    loop {
        stall(Duration::from_secs(1));
    }

}
