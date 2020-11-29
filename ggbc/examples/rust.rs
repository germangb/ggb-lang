// static CONST:[u8;16] = [15, 6, 13, 10, 2, 12, 9, 0, 7, 14, 5
// , 4, 3, 11, 1, 8];static mut STATIC:[u8;16] = [0; 16];static
//  mut REGISTERS:[u8;16] = [0;16];static mut RETURN:[u8;16] =
// [0; 16];unsafe fn _0(args:[u8;0])->[u8;0] {let mut stack=[0;
// 256];let mut pc=0;loop {match pc{0=>{{}},1=>{stack[0+0 as us
// ize]=0},2=>{REGISTERS[0]=16},3=>{REGISTERS[1]=(REGISTERS[0]
// as u8).wrapping_sub(stack[0+0 as usize] as u8)},4=>{if REGIS
// TERS[1]!=0{pc+=1}},5=>{pc+=3},6=>{STATIC[0+stack[0+0 as usiz
// e] as usize]=CONST[0+stack[0+0 as usize] as usize]},7=>{stac
// k[0+0 as usize]=(stack[0+0 as usize] as u8).wrapping_add(1u8
// )},8=>{pc-=6},9=>{stack[0+0 as usize]=0},10=>{REGISTERS[0]=1
// 6},11=>{REGISTERS[1]=(REGISTERS[0] as u8).wrapping_sub(stack
// [0+0 as usize] as u8)},12=>{if REGISTERS[1]!=0{pc+=1}},13=>{
// pc+=15},14=>{stack[1+0 as usize]=0},15=>{REGISTERS[1]=16},16
// =>{REGISTERS[2]=(REGISTERS[1] as u8).wrapping_sub(stack[1+0
// as usize] as u8)},17=>{if REGISTERS[2]!=0{pc+=1}},18=>{pc+=8
// },19=>{stack[2+0 as usize]=STATIC[0+stack[0+0 as usize] as u
// size]},20=>{stack[3+0 as usize]=STATIC[0+stack[1+0 as usize]
//  as usize]},21=>{REGISTERS[2]=if stack[2+0 as usize]<stack[3
// +0 as usize]{1}else{0}},22=>{if REGISTERS[2]==0{pc+=2}},23=>
// {STATIC[0+stack[0+0 as usize] as usize]=stack[3+0 as usize]}
// ,24=>{STATIC[0+stack[1+0 as usize] as usize]=stack[2+0 as us
// ize]},25=>{stack[1+0 as usize]=(stack[1+0 as usize] as u8).w
// rapping_add(1u8)},26=>{pc-=11},27=>{stack[0+0 as usize]=(sta
// ck[0+0 as usize] as u8).wrapping_add(1u8)},28=>{pc-=18},29=>
// {std::process::exit(0)},_=>panic!(),}pc += 1;}}fn main(){uns
// afe{ _0([]);}}
use ggbc::target::Rust;
use std::{
    io,
    io::{BufWriter, Cursor},
};

fn main() {
    let rust = ggbc::compile::<Rust>(include_str!("../../vm/tests/programs/function.ggb")).unwrap();

    std::io::copy(
        &mut Cursor::new(rust.as_bytes()),
        &mut BufWriter::new(io::stdout()),
    )
    .unwrap();
}
