#[allow(dead_code)]
#[derive(Debug,Copy,Clone,PartialEq,Eq)]
#[repr(u8)]
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

pub enum Color{
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
#[repr(transparent)]
struct ColorCode(u8);
impl ColorCode{
    //in vga the first byte is for the caracter and the second byte is for the Color
    //on the second byte, the first 4 bits define the foreground, the next 3 define the background
    //and the last bit whether the character should blink
    //*note that the or here is the binary operator
    fn new(foreground:Color, background:Color) -> ColorCode{
        //this translates to:
        //background bitshifted to the right or* foreground
        ColorCode((background as u8) << 4 | (foreground as u8))
        //an example will be foreground blue and background Cyan
        //blue = 0001 >> 4 or cyan 0011 = 00010011
    }
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
#[repr(C)]
struct ScreenChar{
    ascii_character:u8,
    color_code:ColorCode,
}

#[repr(transparent)]
struct Buffer{
    chars: [[ScreenChar,BUFFER_WIDTH],BUFFER_HEIGHT]
}

pub struct Writer{
    column_position: usize,
    color_code:ColorCode,
    buffer:&'static mut Buffer,
}
