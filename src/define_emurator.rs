

pub mod de {
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
}