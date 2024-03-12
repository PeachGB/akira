//The std requires an OS for syscalls so we disable it.
//Also the fn main requires the STD so we disable it as well 
#![no_std]
#![no_main]

mod vga_buffer;
use core::panic::PanicInfo;
//this fn is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {loop{}}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] //this atribute is so the compiler outputs a function with the name "_start" very
pub extern "C" fn _start() -> ! { //for the "extern "C"" explanation, google "C calling convention"
    vga_buffer::print("PRINT NASHE");
    loop{}
}


