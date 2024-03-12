//The std requires an OS for syscalls so we disable it.
//Also the fn main requires the STD so we disable it as well 
#![no_std]
#![no_main]

use core::panic::PanicInfo;
//this fn is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {loop{}}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] //this atribute is so the compiler outputs a function with the name "_start" very
pub extern "C" fn _start() -> ! { //for the "extern "C"" explanation, google "C calling convention"
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate(){
        unsafe{
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop{}
}


