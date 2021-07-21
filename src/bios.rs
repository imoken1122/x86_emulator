

use crate::*;
use function::*;
const bios_to_terminal : [u8; 8] = [30, 34, 32, 36, 31, 35, 33, 37]; 


pub fn put_string(s: &str ){

	for c in s.chars(){
		io_out8(0x03f8, c as u8);
	}
}


pub fn bios_video_teletype(emu : &mut Emulator){

	let color = get_register8(emu,Register::EBX as usize) & 0x0f; // bl の下位4bitで光度と色
	let ch = get_register8(emu,Register::EAX as usize );
	let terminal_color = bios_to_terminal[ (color & 0x07) as usize]; // 3bitでcolor
	let mut bright = 0 ;
	if (color & 0x08) != 0 {
		bright = 1;
	}
	let bytes: &[u8] = &[ch];
	let converted: String = String::from_utf8(bytes.to_vec()).unwrap();
	let buf = format!("\x1b[{};{}m{}\x1b[0m",bright,terminal_color,converted);

	put_string(&buf)
}

pub fn bios_video(emu : &mut Emulator){

	let func : u8 = get_register8(emu,Register::EAX as usize + 4 );

	match func{
		0x0e => bios_video_teletype(emu),
		_ => println!("not impl BIOS video function: {:#02x}\n", func),
	}
}