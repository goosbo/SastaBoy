mod cpu;
mod interrupt;
mod memory;
mod timer;
mod sastaboy;
use log::{debug,info,LevelFilter};
use simple_logging;
use crate::sastaboy::SastaBoy;
fn main(){
    // components are their own thing now :D
    simple_logging::log_to_file("gameboy_cpu.log", LevelFilter::Debug).unwrap();
    let sasta_boy = SastaBoy::new();
    sasta_boy.load_rom("test_roms\\11-op a,(hl).gb");
    println!("Emulator: {:?}", sasta_boy);
    println!("weeee wooo");
    sasta_boy.run();
    
}