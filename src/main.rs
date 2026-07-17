#![no_std]
#![no_main]

//  TODO: find better way to import the kernel that to create its exact copy in byte array

mod boot_info;

pub mod kernel_binary;

use uefi::{allocator::Allocator, boot::{AllocateType, allocate_pages}};

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

use libequinox::mem::alloc::*;
use uefi::{print, println};

use core::{num::NonZero, panic::PanicInfo, ptr::NonNull, time::Duration};

use uefi::{boot::{MemoryAttribute, MemoryType, get_handle_for_protocol, stall}, mem::memory_map::MemoryMap, prelude::*};

use libelf::api::HeaderEntryType;

use crate::kernel_binary::KERNEL_BINARY;


#[entry]
fn main() -> Status {

    if let Err(e) = uefi::helpers::init() {
        panic!("failed to initialize UEFI helpers: {}", e);
    }


    println!("kernel binary ptr: {:p}", (&KERNEL_BINARY.data) as *const u8);

    match libelf::Elf64::parse(&KERNEL_BINARY.data) {
        Ok(e) => {

            for header in e.program_headers() {
                if header.is_loadable() {
                    println!("    loading segment: {header:?}");

                    let pages = match allocate_pages(AllocateType::Address(header.virtual_address().as_ptr() as usize as u64), MemoryType::LOADER_DATA, header.size_in_memory()/4096 + 1) {
                        Ok(allocated) => allocated,
                        Err(e) => {
                            println!("allocation failed: {e}");
                            panic!("allocation failed");
                        },
                    };

                    println!("allocated: {:p}, expected: {:p}", pages, header.virtual_address());


                } else {
                    println!("    segment is not loadable");
                }
            }

        },
        Err(e) => println!("libelf returned error: {e:?}"),
    }


    loop {
        boot::stall(Duration::from_secs(1));
    }
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
