const VGA_BUFFER: usize = 0xb8000;
#[allow(dead_code)]
#[derive(Debug,Copy,Clone,PartialEq,Eq)]
#[repr(u8)]
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

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
#[repr(transparent)]
struct Buffer{
    chars: [[ScreenChar;BUFFER_WIDTH];BUFFER_HEIGHT]
}

pub struct Writer{
    column_position: usize,
    color_code:ColorCode,
    buffer:&'static mut Buffer,
}
impl Writer{
        pub fn newline(&mut self){}  

        pub fn write_byte(&mut self, byte: u8){
        match byte{
            b'\n' => self.newline(),
            byte =>{
                if self.column_position >= BUFFER_WIDTH {
                    self.newline();
                }
                
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                let color_code = self.color_code;
                
                self.buffer.chars[row][col] =ScreenChar{
                        ascii_character:byte,
                        color_code:color_code,
                    };
                self.column_position += 1 
            }
        }

    }
    pub fn write_string(&mut self, s:&str){
        for byte in s.bytes(){
            match byte{
                //if the byte is a printeable ascii char we print it else we print a ■
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _=> self.write_byte(0xfe),
            }
        }
    } 
   

}
pub fn print(s: &[u8]){

    let vga_buffer = VGA_BUFFER as *mut u8;

   
    for (i, &byte) in s.iter().enumerate(){
        unsafe{
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
}
   
fn printv(){

}
