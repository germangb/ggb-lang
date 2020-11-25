//! Rust compilation target.
use crate::{
    byteorder::NativeEndian,
    ir::{
        opcodes::{Destination, Location, Pointer, Source, Statement},
        Ir, Routine,
    },
    target::Target,
};
use std::io::Write;

/// Rust compilation target.
#[derive(Debug)]
#[warn(clippy::empty_enum)]
pub enum Rust {}

macro_rules! output {
    ($expr:expr) => {};
    ($expr:expr, $($args:tt)*) => {
        write!($expr, $($args)*)
    }
}

impl Target for Rust {
    type ByteOrder = NativeEndian;
    type Output = String;
    type Error = std::io::Error;

    #[warn(unused)]
    fn codegen(ir: &Ir<Self::ByteOrder>) -> Result<Self::Output, Self::Error> {
        let mut output = Vec::new();

        output!(&mut output,
                "static CONST:[u8;{}] = {:?};",
                ir.const_.len(),
                ir.const_)?;
        output!(&mut output,
                "static mut STATIC:[u8;{}] = [0; {}];",
                ir.static_alloc,
                ir.static_alloc)?;
        output!(&mut output, "static mut REGISTERS:[u8;16] = [0;16];")?;
        output!(&mut output, "static mut RETURN:[u8;{}] = [0; {}];", 16, 16)?;

        for (i, routine) in ir.routines.iter().enumerate() {
            codegen_routine(&ir.routines, &mut output, (i, routine))?;
        }
        output!(&mut output,
                "fn main(){{unsafe{{ _{main}([]);}}}}",
                main = ir.handlers.main)?;
        Ok(String::from_utf8(output).unwrap())
    }
}

#[warn(unused)]
fn codegen_routine(routines: &[Routine],
                   output: &mut Vec<u8>,
                   (i, routine): (usize, &Routine))
                   -> Result<(), std::io::Error> {
    output!(output,
            "unsafe fn _{}(args:[u8;{}])->[u8;{}] {{",
            //routine.debug_name.as_ref().unwrap(),
            i,
            routine.args_size,
            routine.return_size)?;
    output!(output, "let mut STACK=[0;256];")?;
    for i in 0..routine.args_size {
        output!(output, "STACK[{}] = args[{}];", i, i)?;
    }
    output!(output, "let mut pc=0;")?;
    output!(output, "loop {{")?;
    output!(output, "match pc{{")?;
    for (i, statement) in routine.statements.iter().enumerate() {
        output!(output, "{}=>", i)?;
        output!(output, "{{")?;
        codegen_statement(routines, output, statement, routine)?;
        output!(output, "}}")?;
        output!(output, ",")?;
        //output!(output, "/*{}*/", statement.display())?;
        output!(output);
    }
    output!(output, "_=>panic!(),")?;
    output!(output, "}}")?;
    output!(output, "pc += 1;")?;
    output!(output, "}}")?;
    output!(output, "}}")?;
    Ok(())
}

#[warn(unused)]
#[rustfmt::skip]
fn codegen_statement(routines: &[Routine],
                     output: &mut Vec<u8>,
                     statement: &Statement,
                     routine: &Routine)
                     -> Result<(), std::io::Error> {
    use Statement::*;
    match statement {
        Nop(_) => output!(output, "{{}}")?,
        Stop => output!(output, "std::process::exit(0)")?,
        Ld { source, destination, } => output!(output, "{}={}", dest(destination), src(source))?,
        Inc { source, destination, } => output!(output, "{}=({} as u8).wrapping_add(1u8)", dest(destination), src(source))?,
        Dec { source, destination, } => output!(output, "{}=({} as u8).wrapping_sub(1u8)", dest(destination), src(source))?,
        Add { destination, left, right, } => output!(output, "{}=({} as u8).wrapping_add({} as u8)", dest(destination), src(left), src(right))?,
        Sub { destination, left, right, } => output!(output, "{}=({} as u8).wrapping_sub({} as u8)", dest(destination), src(left), src(right))?,
        And { destination, left, right, } => output!(output, "{}={}&{}", dest(destination), src(left), src(right))?,
        Xor { destination, left, right, } => output!(output, "{}={}^{}", dest(destination), src(left), src(right))?,
        Or { destination, left, right, } => output!(output, "{}={}|{}", dest(destination), src(left), src(right))?,
        LeftShift { destination, left, right, } => output!(output, "{}={}<<{}", dest(destination), src(left), src(right))?,
        RightShift { destination, left, right, } => output!(output, "{}={}>>{}", dest(destination), src(left), src(right))?,
        Mul { destination, left, right, } => output!(output, "{}={}.wrapping_mul({})", dest(destination), src(left), src(right))?,
        Div { destination, left, right, } => output!(output, "{}={}.wrapping_div({})", dest(destination), src(left), src(right))?,
        Rem { destination, left, right, } => output!(output, "{}={}.wrapping_rem({})", dest(destination), src(left), src(right))?,
        Eq { destination, left, right, } => output!(output, "{}=if {}=={}{{1}}else{{0}}", dest(destination), src(left), src(right))?,
        NotEq { destination, left, right, } => output!(output, "{}=if {}!={}{{1}}else{{0}}", dest(destination), src(left), src(right))?,
        Greater { destination, left, right, } => output!(output, "{}=if {}>{}{{1}}else{{0}}", dest(destination), src(left), src(right))?,
        GreaterEq { destination, left, right, } => output!(output, "{}=if {}>={}{{1}}else{{0}}", dest(destination), src(left), src(right))?,
        Less { destination, left, right, } => output!(output, "{}=if {}<{}{{1}}else{{0}}", dest(destination), src(left), src(right))?,
        LessEq { destination, left, right, } => output!(output, "{}=if {}<={}{{1}}else{{0}}", dest(destination), src(left), src(right))?,
        Jmp { location: Location::Relative(r), } => {
            if *r >= 0 {
                output!(output, "pc+={}", r)?
            } else {
                output!(output, "pc-={}", -r)?
            }
        }
        JmpCmp { location: Location::Relative(r),
                 source, } => {
            if *r >= 0 {
                output!(output, "if {}!=0{{pc+={}}}", src(source), r)?
            } else {
                output!(output, "if {}!=0{{pc+={}}}", src(source), r)?
            }
        },
        JmpCmpNot { location: Location::Relative(r),
                    source, } => {
            if *r >= 0 {
                output!(output, "if {}==0{{pc+={}}}", src(source), r)?
            } else {
                output!(output, "if {}==0{{pc-={}}}", src(source), -r)?
            }
        }
        Call { routine, range } => {
            let routine_ = &routines[*routine];
            output!(output, "{{")?;
            output!(output,
                   "let mut args:[u8;{}]=[0;{}];",
                   routine_.args_size, routine_.args_size)?;
            for (i, offset) in range.clone().enumerate() {
                output!(output, "args[{}]=STACK[{}];", i, offset)?;
            }
            output!(output, "let ret=_{}(args);", routine)?;
            for i in 0..routine_.return_size {
                output!(output, "RETURN[{}]=ret[{}];", i, i)?;
            }
            output!(output, "}}")?;
        }
        Ret => {
            output!(output, "let mut ret=[0;{}];", routine.return_size);
            for i in 0..routine.return_size {
                output!(output, "ret[{}]=RETURN[{}];", i, i)?;
            }
            output!(output, "return ret")?
        },
        _ => output!(output, "unimplemented!()")?,
    }
    Ok(())
}

fn dest(destination: &Destination) -> String {
    match destination {
        Destination::Pointer { base, offset } => pointer(base, offset),
        Destination::Register(register) => format!("REGISTERS[{}]", register),
    }
}

fn src(source: &Source<u8>) -> String {
    match source {
        Source::Pointer { base, offset } => pointer(base, offset),
        Source::Register(register) => format!("REGISTERS[{}]", register),
        Source::Literal(literal) => format!("{}", literal),
    }
}

fn pointer(base: &Pointer, offset: &Option<Box<Source<u8>>>) -> String {
    let offset = offset.as_ref()
                       .map(|s| src(s))
                       .unwrap_or_else(|| "0".to_string());
    match base {
        Pointer::Absolute(_) => todo!(),
        Pointer::Static(a) => format!("STATIC[{}+{} as usize]", a, offset),
        Pointer::Const(a) => format!("CONST[{}+{} as usize]", a, offset),
        Pointer::Stack(a) => format!("STACK[{}+{} as usize]", a, offset),
        Pointer::Return(a) => format!("RETURN[{}+{} as usize]", a, offset),
    }
}
