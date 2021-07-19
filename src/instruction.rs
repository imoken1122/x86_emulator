
use crate::*;
use function::*;
use modrm::*;
use std::convert::TryInto;


pub fn mov_r32_imm32(emu : &mut Emulator){
	let reg : u8 = get_code8( emu,0) - 0xB8;
	let value : u32 = get_code32( emu, 1);
	set_register32(emu,reg as usize, value);
	emu.eip += 5;
}

pub fn mov_r32_rm32(emu : &mut Emulator){
	emu.eip += 1;
	let mut modrm = parse_modrm(emu);
	let rm32 : u32 = get_rm32(emu,&mut modrm);
	set_r32(emu, &mut modrm , rm32);
}
pub fn mov_rm32_r32(emu: &mut Emulator){
	emu.eip += 1;
	let mut modrm = parse_modrm(emu);
	let r32 : u32 = get_r32(emu, &mut modrm);
	set_rm32(emu,&mut modrm, r32);
}
pub fn mov_rm32_imm32(emu: &mut Emulator){
	emu.eip += 1;
	let mut modrm = parse_modrm(emu);
	let value = get_code32( emu,0);
	emu.eip += 4;
	set_rm32(emu, &mut modrm, value);
}

pub fn add_rm32_r32(emu: &mut Emulator){
	emu.eip += 1;
	let mut modrm = parse_modrm(emu);
	let r32 = get_r32(emu, &mut modrm);
	let rm32 = get_rm32(emu, &mut modrm);
	set_rm32(emu, &mut modrm, r32 + rm32);
}

pub fn add_rm32_imm8(emu: &mut Emulator, modrm: &mut ModRM){
	let rm32 : u32 = get_rm32(emu,modrm);
	let imm8 =  get_sign_code8( emu,0) as u32;
	emu.eip += 1;
	set_rm32(emu,modrm, rm32 + imm8);
}
pub fn sub_rm32_imm8(emu: &mut Emulator, modrm: &mut ModRM){

	let rm32 : u32 = get_rm32(emu,modrm);
	let imm8 =  get_sign_code8( emu,0) as u32;
	emu.eip += 1;
	set_rm32(emu,modrm, rm32 - imm8);
}
pub fn short_jump(emu: &mut Emulator){
	let diff : i8 = get_sign_code8(emu,1);
	emu.eip = emu.eip.wrapping_add((diff + 2) as u32); // ignore overflow

}
pub fn inc_rm32(emu: &mut Emulator, modrm : &mut ModRM){
	let value: u32 = get_rm32(emu, modrm);
	set_rm32(emu, modrm, value + 1);
}

pub fn code_83(emu: &mut Emulator,){
	emu.eip += 1;
	let mut modrm = parse_modrm(emu);
	match modrm.opcode{
		0 => add_rm32_imm8(emu, &mut modrm),
		5 => sub_rm32_imm8(emu, &mut modrm),

		_ => println!("not implemented 83 {:#04x}", modrm.opcode),
	}
}
pub fn code_ff(emu: &mut Emulator,){
	emu.eip += 1;
	let mut modrm = parse_modrm(emu);
	match modrm.opcode { 
		 0 => inc_rm32(emu, &mut modrm),
		 //1=> dec_rm32(emu, &mut modrm),
		 _ => println!("not implmented : FF{:#04x}",modrm.opcode),
	}
}

pub fn near_jump(emu : &mut Emulator){
	let diff : i32 = get_sign_code32(emu,1);
	emu.eip = emu.eip.wrapping_add((diff + 5) as u32); // ignore overflow
}

pub fn nop(emu : &mut Emulator){
	emu.eip +=1;
}
pub fn push32(emu : &mut Emulator, value : u32){
	let address : u32= get_register32( emu, Register::ESP as usize) - 4;
	set_register32( emu,Register::ESP as usize,address.try_into().unwrap());
	set_memory32(emu, address.try_into().unwrap(), value);
}
pub fn push_r32(emu : &mut Emulator){
	let reg : u8 = get_code8(emu,0) - 0x50;
	let value : u32= get_register32(emu, reg.try_into().unwrap());
	push32(emu,value);
	emu.eip +=1;
}
pub fn push_imm32(emu : &mut Emulator){
	let value : u32= get_code32(emu,1);
	push32(emu,value);
	emu.eip += 5;
}
pub fn push_imm8(emu : &mut Emulator){
	let value : u8= get_code8(emu,1);
	push32(emu,value.try_into().unwrap());
	emu.eip += 2;

}
pub fn pop32(emu : &mut Emulator) -> u32 {
	let address : u32 = get_register32(emu, Register::ESP as usize);
	let value : u32 = get_memory32(emu, address.try_into().unwrap()) ;
	set_register32(emu,Register::ESP as usize, (address + 4).try_into().unwrap());
	value
}
pub fn pop_r32(emu : &mut Emulator){
	let reg : u8 = get_code8(emu,0) - 0x58;
	let value : u32 =  pop32(emu);
	set_register32(emu,reg.try_into().unwrap(), value);
	emu.eip += 1;
}

pub fn call_rel32(emu: &mut Emulator){
	let diff : i32 = get_sign_code32(emu,1);
	push32(emu, emu.eip + 5); // push return address
	emu.eip = emu.eip.wrapping_add((diff + 5) as u32); // ignore overflow
}
pub fn ret(emu: &mut Emulator){
	let address : u32 = pop32(emu); //pop return address
	emu.eip = address;
}
pub fn leave(emu: &mut Emulator){
	//mov esp ebp 
	//pop ebp
	let ebp : u32 = get_register32(emu,Register::EBP as usize);
	set_register32(emu,Register::ESP as usize, ebp);
	let ebp_new : u32 = pop32(emu);
	set_register32(emu, Register::EBP as usize, ebp_new);
	emu.eip += 1;

}
pub fn init_instruction(instructions : &mut Inst_type){
	instructions[0x01] = Some(add_rm32_r32);
	for i in 0..8 {
        instructions[0x50 + i] = Some(push_r32);
    }
    for i in 0..8 {
        instructions[0x58 + i] = Some(pop_r32);
    }
	instructions[0x68] = Some(push_imm32);
	instructions[0x6a] = Some(push_imm8);
	instructions[0x83] = Some(code_83);
	instructions[0x89] = Some(mov_rm32_r32);
	instructions[0x90] = Some(nop);
	instructions[0x8B] = Some(mov_r32_rm32);
	for i in 0..8{
		instructions[0xB8 + i] = Some(mov_r32_imm32);
	}
	instructions[0xC3] = Some(ret);
	instructions[0xC7] = Some(mov_rm32_imm32);
  	instructions[0xC9] = Some(leave);
	instructions[0xE8] = Some(call_rel32);
	instructions[0xEB] = Some(short_jump);
	instructions[0xE9] = Some(near_jump);
	instructions[0xff] = Some(code_ff);
	

}