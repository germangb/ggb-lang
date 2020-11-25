// static CONST:[u8;0] = [];static mut STATIC:[u8;1] = [0; 1];static mut
// REGISTERS:[u8;16] = [0;16];static mut RETURN:[u8;16] = [0; 16];unsafe fn
// _0(args:[u8;2])->[u8;1] {let mut stack=[0;256];stack[0] = args[0];stack[1] =
// args[1];let mut pc=0;loop {match pc{0=>{{}},1=>{REGISTERS[0]=if stack[0+0 as
// usize]<stack[1+0 as usize]{1}else{0}},2=>{if
// REGISTERS[0]==0{pc+=2}},3=>{RETURN[0+0 as usize]=stack[0+0 as usize]},4=>{let
// mut ret=[0;1];ret[0]=RETURN[0];return ret},5=>{RETURN[0+0 as usize]=stack[1+0
// as usize]},6=>{let mut ret=[0;1];ret[0]=RETURN[0];return ret},_=>panic!(),}pc
// += 1;}}unsafe fn _1(args:[u8;2])->[u8;1] {let mut stack=[0;256];stack[0] =
// args[0];stack[1] = args[1];let mut pc=0;loop {match
// pc{0=>{{}},1=>{REGISTERS[0]=if stack[0+0 as usize]>=stack[1+0 as
// usize]{1}else{0}},2=>{if REGISTERS[0]==0{pc+=2}},3=>{RETURN[0+0 as
// usize]=stack[0+0 as usize]},4=>{let mut ret=[0;1];ret[0]=RETURN[0];return
// ret},5=>{RETURN[0+0 as usize]=stack[1+0 as usize]},6=>{let mut
// ret=[0;1];ret[0]=RETURN[0];return ret},_=>panic!(),}pc += 1;}}unsafe fn
// _2(args:[u8;0])->[u8;0] {let mut stack=[0;256];let mut pc=0;loop {match
// pc{0=>{{}},1=>{stack[0+0 as usize]=4},2=>{stack[1+0 as usize]=2},3=>{{let mut
// args:[u8;2]=[0;2];args[0]=stack[0];args[1]=stack[1];let
// ret=_1(args);RETURN[0]=ret[0];}},4=>{stack[0+0 as usize]=RETURN[0+0 as
// usize]},5=>{stack[1+0 as usize]=4},6=>{stack[2+0 as usize]=2},7=>{{let mut
// args:[u8;2]=[0;2];args[0]=stack[1];args[1]=stack[2];let
// ret=_0(args);RETURN[0]=ret[0];}},8=>{stack[1+0 as usize]=RETURN[0+0 as
// usize]},9=>{REGISTERS[0]=(stack[0+0 as usize] as u8).wrapping_sub(stack[1+0
// as usize] as u8)},10=>{STATIC[0+0 as
// usize]=REGISTERS[0]},11=>{std::process::exit(0)},_=>panic!(),}pc += 1;}}fn
// main(){unsafe{ _2([]);}}
use ggbc::target::Rust;
use std::{
    io,
    io::{BufWriter, Cursor},
};

fn main() {
    let rust = ggbc::compile::<Rust>(include_str!("../../vm/tests/programs/function.ggb")).unwrap();

    std::io::copy(&mut Cursor::new(rust.as_bytes()),
                  &mut BufWriter::new(io::stdout())).unwrap();
}
