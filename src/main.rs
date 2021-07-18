
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io;

pub mod instruction;
pub mod function;
use instruction::*;
use function::*;

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
