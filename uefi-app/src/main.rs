#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![feature(alloc_error_handler)]

extern crate alloc;
use core::{fmt::Write, panic::PanicInfo, alloc::Layout};
use alloc::vec::Vec;
use uefi::{prelude::entry, table::cfg, proto::console::gop::GraphicsOutput};


#[entry]
fn efi_main(
    image: uefi::Handle,
    mut system_table: uefi::table::SystemTable<uefi::table::Boot>
) -> uefi::Status {
    // initialize the allocator
    unsafe {
        uefi::alloc::init(
            system_table.boot_services()
        );
    }
    let mut config_entries = system_table.config_table().iter();
    let rsdp_addr = config_entries
        .find(|entry| matches!(entry.guid, cfg::ACPI_GUID | cfg::ACPI2_GUID))
        .map(|entry| entry.address);
        
    let protocal = system_table.boot_services().get_handle_for_protocol::<GraphicsOutput>().unwrap();


    let stdout = system_table.stdout();
    stdout.clear().unwrap();
    writeln!(stdout, "Hello world!").unwrap();
    
    writeln!(stdout, "alloc").unwrap();
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    writeln!(stdout, "v = {:?}", v).unwrap();
    
    writeln!(stdout, "rsdp addr: {:?}", rsdp_addr).unwrap();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    panic!("out of memory")
}