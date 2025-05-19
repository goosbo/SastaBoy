mod cpu;
mod interrupt;
mod memory;
mod timer;
mod sastaboy;

use crate::sastaboy::SastaBoy;
fn main(){
    // components are their own thing now :D
    let sasta_boy = SastaBoy::new();
    sasta_boy.run();
    println!("Emulator: {:?}", sasta_boy);
    println!("weeee wooo");
}