#![no_std]
#![no_main]
use core::panic::PanicInfo;
mod vga_buffer;
use core::fmt::Write;
use vga_buffer::{Buffer, Color, ColorCode, Writer};

// Called on panic, global exception handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // just loop infinitely for now, no need to do anything
    // interesting here yet
    loop {}
}

// rust mangles function names by default, can't do
// that or else linux bindings won't work, system expects _start
#[no_mangle]
// linker looks for this function name, explicitly delcare that
// we're exposing a C binding
pub extern "C" fn _start() -> ! {
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    for i in 0x21..0x7e {
        writer.write_byte(i as u8);
    }
    write!(writer, "The numbers are {} and {}", 41, 1.0 / 3.0).unwrap();
    loop {}
}
