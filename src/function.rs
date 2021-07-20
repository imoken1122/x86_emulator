
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

const CARRY_FLAG : u32 = 1;
const ZERO_FLAG : u32 = 1 << 6;
const SIGN_FLAG : u32 = 1 << 7;
const OVERFLOW_FLAG : u32 = 1 << 11;

pub fn set_carry(emu : &mut Emulator, is_carry : bool){
    if is_carry {
        emu.eflags |= CARRY_FLAG;
    }
    else {
        emu.eflags &= !CARRY_FLAG;
    }
}

pub fn set_zero(emu : &mut Emulator, is_zero : bool){
    if is_zero {
        emu.eflags |= ZERO_FLAG;
    }
    else {
        emu.eflags &= !ZERO_FLAG;
    }
}
pub fn set_sign(emu : &mut Emulator, is_sign : bool){
    if is_sign {
        emu.eflags |= SIGN_FLAG;
    }
    else {
        emu.eflags &= !SIGN_FLAG;
    }
}
pub fn set_overflow(emu : &mut Emulator, is_of : bool){
    if is_of {
        emu.eflags |= OVERFLOW_FLAG;
    }
    else {
        emu.eflags &= !OVERFLOW_FLAG;
    }
}

pub fn update_eflags_sub(emu : &mut Emulator, v1 : u32, v2 : u32 , res : u64){
    let sign1 = v1 >> 31;
    let sign2 = v2 >> 31;
    let signr = (res >> 31) & 1;

    set_carry(emu, (res >> 32) != 0);
    set_zero(emu, res == 0);
    set_sign(emu, signr != 0);

    // v1 - (-v2) = v1 + v2 => case OF, result negative
    // -v1 - (v2) = -v1 - v2 => case OF, result negative
    set_overflow(emu, (sign1 != sign2) && (sign1 != signr as u32));


}
pub fn is_carry(emu: &mut Emulator) -> bool{
    (emu.eflags & CARRY_FLAG) !=0
}
pub fn is_zero(emu: &mut Emulator) -> bool{
    (emu.eflags & ZERO_FLAG) !=0
}
pub fn is_sign(emu: &mut Emulator) -> bool{
    (emu.eflags & SIGN_FLAG) !=0
}
pub fn is_overflow(emu: &mut Emulator) -> bool{
    (emu.eflags & OVERFLOW_FLAG) !=0
}