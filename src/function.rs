
use crate::*;
use std::convert::TryInto;


pub fn get_code8(emu : &mut Emulator, index : usize) -> u8{
    emu.memory[emu.eip as usize + index]

}
pub fn get_code32(emu : &mut Emulator, index : usize) -> u32{
    let mut res : u32 = 0;

    for i in 0..4{
        // little endian
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
pub fn set_register32(emu : &mut Emulator,index: usize, value : u32){
    emu.registers[index] = value;
}
pub fn get_register32(emu : &mut Emulator, index : usize) -> u32{
    emu.registers[index]
}
pub fn get_memory8(emu : &mut Emulator, address : usize) -> u8{
    emu.memory[address]
}
pub fn get_memory32(emu : &mut Emulator, address : usize) -> u32{
    let mut res : u32 = 0;
    for i in 0..4{
        res |= (get_memory8(emu,address+i) as u32) << (8 * i);
    }
    res
}


pub fn set_memory8(emu: &mut Emulator, address : usize, value : u32){
    emu.memory[address] = (value & 0xff).try_into().unwrap();
}
pub fn set_memory32(emu: &mut Emulator, address : usize, value : u32){
    for i in 0..4{
        set_memory8(emu,address + i, value >> (8 * i))
    }
}