#![no_std]
#![no_main]
use core::panic::PanicInfo;
mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}",_info);
    loop {}
}

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
    panic!("Some panic message");

}
