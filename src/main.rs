
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io;


pub const MEMORY_SIZE: u32 = 1024 * 1024;
pub const REGISTER_COUNT : usize = 8;
pub const BIOS_OFFSET: usize = 0x7c00;

pub enum Register {EAX,ECX,EDX,EBX,ESP,EBP,ESI,EDI}
pub const register_name:[&str;REGISTER_COUNT]= ["EAX", "ECX", "EDX", "EBX", "ESP", "EBP", "ESI", "EDI"];
pub struct Emulator{ 
    registers : [u32; REGISTER_COUNT],
    eflags : u32,
    memory : Vec<u8>,
    eip : u32

}

pub fn create_emulator(size: usize,eip:u32,esp : u32 ) -> Emulator 
{
    let mut emu = Emulator{
        registers :[0;REGISTER_COUNT as usize],
        eflags : 0,
        memory : vec![0;size],
        eip : eip
    };
    emu.registers[Register::ESP as usize] = esp;

    emu
}
fn dump_registers(emu:&mut Emulator){
    for i in 0..REGISTER_COUNT{
        println!("{} = {:#010x}", register_name[i as usize],emu.registers[i as usize]); 
    }
    println!("EIP = {:#08x}\n",emu.eip);
}
fn read_to_memory(file : &mut File, emu : &mut Emulator) -> Result<usize, io::Error> {
    let mut cnt : usize= 0;

    for b in file.bytes() {

        emu.memory[BIOS_OFFSET + cnt] = b?;
        cnt += 1;
    }
    
    Ok(cnt)
}



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

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() != 2 { 
        println!("usage : not filename\n")
    }

    let mut emu = create_emulator(MEMORY_SIZE as usize ,0x7c00,0x7c00);
    let mut f = File::open(&args[1]).expect("file not found");
    read_to_memory(&mut f, &mut emu).expect("faild to read file");

    let mut instructions : [Option<fn(&mut Emulator)>; 256] = [None; 256];
    init_instruction(&mut instructions);

    while emu.eip < MEMORY_SIZE{
        
        let code : u8 = get_code8(&mut emu, 0);

        println!("EIP = {:#010x}, Code = {:#04x}", emu.eip, code);

        match instructions[code as usize]{
            Some(inst) => inst(&mut emu),
            None => {
                println!("\n\nNot implemented : {:#04x}\n", code);
                break;
            }
        }

        if emu.eip == 0x00{
            println!("\n\nend of program \n\n");
            break;
        }
    }

    dump_registers(&mut emu);





}
