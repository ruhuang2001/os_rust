#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
mod vga_buffer;
mod serial;

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // let vga_buffer = 0xb8000 as *mut u8;

    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }
    
    // vga_buffer::print_something();
    
    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("HELLO RUHUANG").unwrap();
    // write!(vga_buffer::WRITER.lock(), ",some numbers: {} {}", 42, 12.42).unwrap();
    use os_rust::memory::active_level_4_table;
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    os_rust::init();

    // x86_64::instructions::interrupts::int3();
    
    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // };
    
    // fn stack_overflow() {
    //     stack_overflow();
    // }

    // stack_overflow();

    // let ptr = 0x2031b2 as *mut u32;
    
    // // read from a code page
    // unsafe { let x = *ptr; }
    // println!("read worked");

    // unsafe { *ptr = 42; }
    // println!("write worked");


    // let (level_4_page_table, _) = Cr3::read();
    // println!("Level 4 page table at {:?}", level_4_page_table.start_address());

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe {
        active_level_4_table(phys_mem_offset)
    };  
    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
        }
    }
    #[cfg(test)]
    test_main();

    println!("It did not crash");
    // loop{
    //     use os_rust::print;
    //     print!("-");
    // }
    os_rust::hlt_loop();
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    println!("Running {} tests ", tests.len());
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);

}

#[test_case]
fn trivial_assertion() {
    serial_print!("trivial assertion... ");
    assert_eq!(1, 1);
    serial_println!("[ok]");    
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}


pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    os_rust::test_panic_handler(_info)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}",_info);
    os_rust::hlt_loop();
}
