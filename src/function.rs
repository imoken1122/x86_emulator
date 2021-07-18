
use crate::*;
pub fn get_code8(emu : &mut Emulator, index : usize) -> u8{
    emu.memory[emu.eip as usize + index]

}
pub fn get_code32(emu : &mut Emulator, index : usize) -> u32{
    let mut res : u32 = 0;

    for i in 0..4{
        // litle endian
        res |= (get_code8(emu, index + i) as u32) << (i * 8);
    }

    res 
}

pub fn get_sign_code8(emu: &mut Emulator, index : usize) -> i8{
    emu.memory[emu.eip as usize + index] as i8
}
pub fn get_sign_code32(emu: &mut Emulator, index : usize) -> i32{
    get_code32(emu,index) as i32
}
