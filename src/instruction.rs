
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
pub fn mov_r8_imm8(emu: &mut Emulator){
	let reg = get_code8(emu,0) - 0xB0;
	let value = get_code8(emu,1);
	set_register8(emu,reg.try_into().unwrap(),value,3);
	emu.eip += 2;
}
pub fn mov_rm8_r8(emu: &mut Emulator){
	emu.eip += 1;
	let mut modrm = parse_modrm(emu);
	let r8 = get_r8(emu, &mut modrm);
	set_rm8(emu,&mut modrm, r8);
}
pub fn mov_r8_rm8(emu: &mut Emulator){
	emu.eip += 1;
	let mut modrm = parse_modrm(emu);
	let rm8 = get_rm8(emu,&mut modrm);
	set_r8(emu, &mut modrm , rm8);
}
pub fn add_rm32_r32(emu: &mut Emulator){
	emu.eip += 1;
	let mut modrm = parse_modrm(emu);
	let r32 = get_r32(emu, &mut modrm);
	let rm32 = get_rm32(emu, &mut modrm);
	set_rm32(emu, &mut modrm, r32 + rm32);
}

pub fn add_rm32_imm8(emu: &mut Emulator, modrm: &mut ModRM){
	let rm32 = get_rm32(emu,modrm);
	let imm8 =  get_sign_code8( emu,0) as u32;
	emu.eip += 1;
	set_rm32(emu,modrm, rm32 + imm8);
}
pub fn sub_rm32_imm8(emu: &mut Emulator, modrm: &mut ModRM){

	let rm32 : u32 = get_rm32(emu,modrm);
	let imm8 : u32 =  get_sign_code8( emu,0) as u32;
	emu.eip += 1;
	let res = rm32 - imm8;
	set_rm32(emu,modrm, res);
	update_eflags_sub(emu,rm32, imm8,res as u64);
}
pub fn short_jump(emu: &mut Emulator){
	let diff : i8 = get_sign_code8(emu,1);
	emu.eip = emu.eip.wrapping_add((diff + 2) as u32); // ignore overflow

}
pub fn inc_rm32(emu: &mut Emulator, modrm : &mut ModRM){
	let value: u32 = get_rm32(emu, modrm);
	set_rm32(emu, modrm, value + 1);
}
pub fn dec_rm32(emu: &mut Emulator, modrm: &mut ModRM){
	let value: u32 = get_rm32(emu, modrm);
	set_rm32(emu, modrm, value - 1);
}
pub fn code_83(emu: &mut Emulator,){
	emu.eip += 1;
	let mut modrm = parse_modrm(emu);
	match modrm.opcode{
		0 => add_rm32_imm8(emu, &mut modrm),
		5 => sub_rm32_imm8(emu, &mut modrm),
		7 => cmp_rm32_imm8(emu, &mut modrm),
		_ => println!("not implemented 83 {:#04x}", modrm.opcode),
	}
}
pub fn code_ff(emu: &mut Emulator,){
	emu.eip += 1;
	let mut modrm = parse_modrm(emu);
	match modrm.opcode { 
		 0 => inc_rm32(emu, &mut modrm),
		 1 => dec_rm32(emu, &mut modrm),
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
pub fn cmp_r32_rm32(emu: &mut Emulator){
	emu.eip += 1;
	let mut  modrm = parse_modrm(emu);
	let rm32 = get_rm32(emu, &mut modrm);
	let r32 = get_r32(emu, &mut modrm);
	let res = r32 as i64 - rm32 as i64;
	update_eflags_sub(emu , r32, rm32 ,res as u64);
}
pub fn cmp_rm32_imm8(emu: &mut Emulator, modrm: &mut ModRM){
	let rm32 : u32 = get_rm32(emu, modrm);
	let imm8 : u32 = get_sign_code8(emu,0) as u32;
	emu.eip += 1; //
	let res  = rm32 - imm8 ;
	update_eflags_sub(emu , rm32, imm8 ,res as u64);// converting u64 is to know carry 


}


pub fn js(emu : &mut Emulator){
	let mut diff = 0;
	if is_sign(emu){
		diff = get_sign_code8(emu, 1);
	}
	emu.eip = emu.eip.wrapping_add((diff + 2) as u32);
}
pub fn jns(emu: &mut Emulator){
	let mut diff = 0;
	if !is_sign(emu){
		diff = get_sign_code8(emu, 1);

	}
	emu.eip = emu.eip.wrapping_add((diff + 2) as u32);
}

pub fn jc(emu : &mut Emulator){
	let mut diff = 0;
	if is_carry(emu){
		diff = get_sign_code8(emu, 1);
	}
	emu.eip = emu.eip.wrapping_add((diff + 2) as u32);
}
pub fn jnc(emu: &mut Emulator){
	let mut diff = 0;
	if !is_carry(emu){
		diff = get_sign_code8(emu, 1);

	}
	emu.eip = emu.eip.wrapping_add((diff + 2) as u32);
}

pub fn jz(emu : &mut Emulator){
	let mut diff = 0;
	if is_zero(emu){
		diff = get_sign_code8(emu, 1);
	}
	emu.eip = emu.eip.wrapping_add((diff + 2) as u32);
}
pub fn jnz(emu: &mut Emulator){
	let mut diff = 0;
	if !is_zero(emu){
		diff = get_sign_code8(emu, 1);

	}
	emu.eip = emu.eip.wrapping_add((diff + 2) as u32);
}
pub fn jo(emu : &mut Emulator){
	let mut diff = 0;
	if is_overflow(emu){
		diff = get_sign_code8(emu, 1);
	}
	emu.eip = emu.eip.wrapping_add((diff + 2) as u32);
}
pub fn jno(emu: &mut Emulator){
	let mut diff = 0;
	if !is_overflow(emu){
		diff = get_sign_code8(emu, 1);

	}
	emu.eip = emu.eip.wrapping_add((diff + 2) as u32);
}

pub fn jl(emu : &mut Emulator){
	let mut diff = 0;
	// is_sign = 1 if a < b  else 0
	if is_sign(emu) != is_overflow(emu){
		diff = get_sign_code8(emu, 1);
	}
	emu.eip = emu.eip.wrapping_add((diff + 2) as u32);
}
pub fn jle(emu: &mut Emulator){
	let mut diff = 0;
	// is_sign = 1 if a <= b  else 0
	if is_zero(emu) || (is_sign(emu) != is_overflow(emu)){
		diff = get_sign_code8(emu, 1);

	}
	emu.eip = emu.eip.wrapping_add((diff + 2) as u32);
}

pub fn in_al_dx(emu: &mut Emulator){
	let address : u32 = get_register32(emu, Register::EDX as usize) & 0xffff;
	let value : u8 = io_in8(address.try_into().unwrap());
	set_register8(emu,Register::EAX as usize, value, 1); // EAX -> AL
	emu.eip += 1;
}

pub fn out_dx_al(emu : &mut Emulator){
	let address : u32 = get_register32(emu, Register::EDX as usize) & 0xffff;
	let value : u8 = get_register8(emu, Register::EAX as usize, 1); // EAX -> AL
	io_out8(address.try_into().unwrap(), value);
	
	emu.eip += 1;
}

pub fn init_instruction(instructions : &mut InstType){
	instructions[0x01] = Some(add_rm32_r32);
	instructions[0x3b] = Some(cmp_r32_rm32);
	for i in 0..8 {
        instructions[0x50 + i] = Some(push_r32);
    }
    for i in 0..8 {
        instructions[0x58 + i] = Some(pop_r32);
    }
	instructions[0x68] = Some(push_imm32);
	instructions[0x6a] = Some(push_imm8);

	instructions[0x70] = Some(jo);
	instructions[0x71] = Some(jno);
	instructions[0x72] = Some(jc);
	instructions[0x73] = Some(jnc);
	instructions[0x74] = Some(jz);
	instructions[0x75] = Some(jnz);
	instructions[0x78] = Some(js);
	instructions[0x79] = Some(jns);
	instructions[0x7C] = Some(jl);
	instructions[0x7E] = Some(jle);

	instructions[0x83] = Some(code_83);
	instructions[0x88] = Some(mov_rm8_r8);
	instructions[0x89] = Some(mov_rm32_r32);
	instructions[0x8a] = Some(mov_r8_rm8);
	instructions[0x90] = Some(nop);
	instructions[0x8B] = Some(mov_r32_rm32);

	for i in 0..8{
        instructions[0xB0 + i] = Some(mov_r8_imm8);
    }
	for i in 0..8{
		instructions[0xB8 + i] = Some(mov_r32_imm32);
	}
	instructions[0xC3] = Some(ret);
	instructions[0xC7] = Some(mov_rm32_imm32);
  	instructions[0xC9] = Some(leave);
	instructions[0xE8] = Some(call_rel32);
	instructions[0xEB] = Some(short_jump);
	instructions[0xEC] = Some(in_al_dx);
	instructions[0xEE] = Some(out_dx_al);
	instructions[0xE9] = Some(near_jump);
	instructions[0xff] = Some(code_ff);
	

}