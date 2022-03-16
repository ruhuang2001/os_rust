#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::panic::PanicInfo;
mod vga_buffer;
mod serial;

#[no_mangle]
pub extern "C" fn _start() -> ! {
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

    println!("Hello World{}", "!");
    os_rust::init();

    x86_64::instructions::interrupts::int3();


    #[cfg(test)]
    test_main();

    println!("It did not crash");
    loop{}

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
    loop {}
}
