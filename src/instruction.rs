


pub fn mov_r32_rm32(emu:&mut Emulator){
	let reg : u8 = get_code8( emu,0) - 0xB8;
	let value : u32 = get_code32( emu,1);
	emu.registers[reg as usize] = value;
	emu.eip += 5;

}
pub fn short_jump(emu: &mut Emulator){
	let diff : i8 = get_sign_code8(emu,1);
	emu.eip = emu.eip.wrapping_add((diff + 2) as u32); // ignore overflow

}
pub fn near_jump(emu : &mut Emulator){
	let diff : i32 = get_sign_code32(emu,1);
	emu.eip = emu.eip.wrapping_add((diff + 5) as u32); // ignore overflow
}
pub fn init_instruction(instructions : &mut [Option<fn(&mut Emulator)>; 256]){
	for i in 0..8{
		instructions[0xB8 + i] = Some(mov_r32_rm32);
	}
	instructions[0xEB] = Some(short_jump);
	instructions[0xE9] = Some(near_jump);
}