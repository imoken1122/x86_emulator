
use crate::*;
use std::convert::TryInto;
use std::process;

pub struct ModRM {
	pub modv : u8,
	pub rm : u8,
	pub sib : u8,
	pub opcode : u8,
	pub reg_index : u8,
	pub disp8 :i8,
	pub disp32 : u32,
}

pub fn parse_modrm(emu : &mut Emulator) -> ModRM{
	let code = get_code8(emu,0);

	let mut modrm = ModRM{
		modv : ((code & 0xC0) >> 6), 
		opcode :  ((code & 0x38) >> 3) ,
		reg_index :  ((code & 0x38) >> 3) ,
		rm : (code & 0x07),
		disp32:0,
		disp8 :0,
		sib:0,
	};

	emu.eip += 1; //get_code8
	
	if  modrm.modv != 3 && modrm.rm == 4 {
		modrm.sib = get_code8(emu,0);
		emu.eip += 1;
	}

	if  (modrm.modv == 0 && modrm.rm == 5)  || modrm.modv==2 {
		modrm.disp32 = get_sign_code32(emu, 0).try_into().unwrap();
		emu.eip += 4;
	}
	else if modrm.modv == 1 { 
		modrm.disp8 = get_sign_code8(emu,0).try_into().unwrap();
		emu.eip += 1;
	}
	modrm
}

pub fn calc_memory_address(emu : &mut Emulator, modrm : &mut ModRM) -> u32{
	if modrm.modv == 0{
		if modrm.rm == 4 { // sib
			println!("not implemented ModRM mod = 0 , rm = 4 \n");
			process::exit(0);
		}
		else if  modrm.rm == 5{
			modrm.disp32
		}
		else {
			get_register32(emu,modrm.rm.try_into().unwrap())
		}
	}
	else if modrm.modv == 1 { 
		if modrm.rm == 4{
			println!("not implemented ModRM mod = 1 , rm = 4 \n");
			process::exit(0);
		
		}
		else{
			(get_register32(emu,modrm.rm.try_into().unwrap()) as i32 + modrm.disp8 as i32) as u32

		}
	}
	else if modrm.modv == 2{
		if modrm.rm == 4{
			println!("not implemented ModRM mod = 2 , rm = 4 \n");
			process::exit(0);
		}
		
		else { 
			get_register32(emu,modrm.rm.try_into().unwrap() ) + modrm.disp32
		}
	
	}
	else {
		println!("not implemented ModRM mod = 3 \n");
		process::exit(0);
		}
}
pub fn get_rm32(emu : &mut Emulator, modrm : &mut ModRM) -> u32{ // [r + disp]
	if modrm.modv == 3 { 
		get_register32(emu, modrm.rm.try_into().unwrap())
	}
	else{
		let address : u32 = calc_memory_address(emu,modrm);
		get_memory32(emu,address.try_into().unwrap())
	}
}
pub fn get_rm8(emu: &mut Emulator,modrm : &mut ModRM) -> u8{
	if modrm.modv == 3 { 
		get_register8(emu, modrm.rm.try_into().unwrap())
	}
	else{
		let address = calc_memory_address(emu,modrm);
		get_memory8(emu,address.try_into().unwrap())
	}	
}

pub fn set_rm32(emu : &mut Emulator, modrm: &mut ModRM, value : u32) {
	if modrm.modv == 3 { 
		set_register32(emu,modrm.rm.try_into().unwrap(), value);
	}
	else{
		let address : u32 = calc_memory_address(emu,modrm);
		set_memory32(emu,address.try_into().unwrap(),value);
	}

}

pub fn set_rm8(emu: &mut Emulator, modrm : &mut ModRM, value : u8){
	if modrm.modv == 3 { 
		set_register8(emu,modrm.rm.try_into().unwrap(), value,);
	}
	else{
		let address = calc_memory_address(emu,modrm);
		set_memory8(emu,address.try_into().unwrap(),value.try_into().unwrap());
	}

}

pub fn set_r32(emu: &mut Emulator, modrm : &mut ModRM, value: u32) {
	set_register32(emu,modrm.reg_index.try_into().unwrap(), value);
}
pub fn set_r8(emu: &mut Emulator, modrm : &mut ModRM, value : u8){
	set_register8(emu,modrm.reg_index.try_into().unwrap(), value,);
}

pub fn get_r32(emu: &mut Emulator, modrm: &mut ModRM) -> u32{
	get_register32(emu,modrm.reg_index.try_into().unwrap())
}
pub fn get_r8(emu: &mut Emulator, modrm: &mut ModRM) -> u8{
	get_register8(emu,modrm.reg_index.try_into().unwrap(),)
}