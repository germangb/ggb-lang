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

impl Target for Rust {
    type ByteOrder = NativeEndian;
    type Output = String;
    type Error = std::io::Error;

    #[warn(unused)]
    fn codegen(ir: &Ir<Self::ByteOrder>) -> Result<Self::Output, Self::Error> {
        let mut output = Vec::new();

        write!(
            &mut output,
            "static CONST:[u8;{}] = {:?};",
            ir.const_.len(),
            ir.const_
        )?;
        write!(
            &mut output,
            "static mut STATIC:[u8;{}] = [0; {}];",
            ir.static_alloc, ir.static_alloc
        )?;
        write!(&mut output, "static mut REGISTERS:[u8;16] = [0;16];")?;
        write!(&mut output, "static mut RETURN:[u8;{}] = [0; {}];", 16, 16)?;

        for (i, routine) in ir.routines.iter().enumerate() {
            codegen_routine(&ir.routines, &mut output, (i, routine))?;
        }
        write!(
            &mut output,
            "fn main(){{unsafe{{ _{main}([]);}}}}",
            main = ir.handlers.main
        )?;
        Ok(String::from_utf8(output).unwrap())
    }
}

#[warn(unused)]
fn codegen_routine(
    routines: &[Routine],
    output: &mut Vec<u8>,
    (i, routine): (usize, &Routine),
) -> Result<(), std::io::Error> {
    write!(
        output,
        "unsafe fn _{}(args:[u8;{}])->[u8;{}] {{",
        //routine.debug_name.as_ref().unwrap(),
        i,
        routine.args_size,
        routine.return_size
    )?;
    write!(output, "let mut stack=[0;256];")?;
    for i in 0..routine.args_size {
        write!(output, "stack[{}] = args[{}];", i, i)?;
    }
    write!(output, "let mut pc=0;")?;
    write!(output, "loop {{")?;
    write!(output, "match pc{{")?;
    for (i, statement) in routine.statements.iter().enumerate() {
        write!(output, "{}=>{{", i)?;
        codegen_statement(routines, output, statement, routine)?;
        write!(output, "}},")?;
        //output!(output, "/*{}*/", statement.display())?;
        //write!(output);
    }
    write!(output, "_=>panic!(),")?;
    write!(output, "}}")?;
    write!(output, "pc += 1;")?;
    write!(output, "}}")?;
    write!(output, "}}")?;
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
        Statement::Nop(_) =>
            write!(output, "{{}}")?,
        Statement::Stop =>
            write!(output, "std::process::exit(0)")?,
        Statement::Ld { source, destination, } =>
            write!(output, "{}={}", dest(destination), src(source))?,
        Statement::Inc { source, destination, } =>
            write!(output, "{}=({} as u8).wrapping_add(1u8)", dest(destination), src(source))?,
        Statement::Dec { source, destination, } =>
            write!(output, "{}=({} as u8).wrapping_sub(1u8)", dest(destination), src(source))?,
        Statement::Add { destination, left, right, } =>
            write!(output, "{}=({} as u8).wrapping_add({} as u8)", dest(destination), src(left), src(right))?,
        Statement::Sub { destination, left, right, } =>
            write!(output, "{}=({} as u8).wrapping_sub({} as u8)", dest(destination), src(left), src(right))?,
        Statement::And { destination, left, right, } =>
            write!(output, "{}={}&{}", dest(destination), src(left), src(right))?,
        Statement::Xor { destination, left, right, } =>
            write!(output, "{}={}^{}", dest(destination), src(left), src(right))?,
        Statement::Or { destination, left, right, } =>
            write!(output, "{}={}|{}", dest(destination), src(left), src(right))?,
        Statement::LeftShift { destination, left, right, } =>
            write!(output, "{}={}<<{}", dest(destination), src(left), src(right))?,
        Statement::RightShift { destination, left, right, } =>
            write!(output, "{}={}>>{}", dest(destination), src(left), src(right))?,
        Statement::Mul { destination, left, right, } =>
            write!(output, "{}=({} as u8).wrapping_mul({} as u8)", dest(destination), src(left), src(right))?,
        Statement::Div { destination, left, right, } =>
            write!(output, "{}=({} as u8).wrapping_div({} as u8)", dest(destination), src(left), src(right))?,
        Statement::Rem { destination, left, right, } =>
            write!(output, "{}=({} as u8).wrapping_rem({} as u8)", dest(destination), src(left), src(right))?,
        Statement::Eq { destination, left, right, } =>
            write!(output, "{}=if {}=={}{{1}}else{{0}}", dest(destination), src(left), src(right))?,
        Statement::NotEq { destination, left, right, } =>
            write!(output, "{}=if {}!={}{{1}}else{{0}}", dest(destination), src(left), src(right))?,
        Statement::Greater { destination, left, right, } =>
            write!(output, "{}=if {}>{}{{1}}else{{0}}", dest(destination), src(left), src(right))?,
        Statement::GreaterEq { destination, left, right, } =>
            write!(output, "{}=if {}>={}{{1}}else{{0}}", dest(destination), src(left), src(right))?,
        Statement::Less { destination, left, right, } =>
            write!(output, "{}=if {}<{}{{1}}else{{0}}", dest(destination), src(left), src(right))?,
        Statement::LessEq { destination, left, right, } =>
            write!(output, "{}=if {}<={}{{1}}else{{0}}", dest(destination), src(left), src(right))?,
        Statement::Jmp { location: Location::Relative(r), } => {
            if *r >= 0 {
                write!(output, "pc+={}", r)?
            } else {
                write!(output, "pc-={}", -r)?
            }
        }
        Statement::JmpCmp { location: Location::Relative(r),
                 source, } => {
            if *r >= 0 {
                write!(output, "if {}!=0{{pc+={}}}", src(source), r)?
            } else {
                write!(output, "if {}!=0{{pc+={}}}", src(source), r)?
            }
        },
        Statement::JmpCmpNot { location: Location::Relative(r),
                    source, } => {
            if *r >= 0 {
                write!(output, "if {}==0{{pc+={}}}", src(source), r)?
            } else {
                write!(output, "if {}==0{{pc-={}}}", src(source), -r)?
            }
        }
        Statement::Call { routine, range } => {
            let routine_ = &routines[*routine];
            write!(output, "{{")?;
            write!(output,
                   "let mut args:[u8;{}]=[0;{}];",
                   routine_.args_size, routine_.args_size)?;
            for (i, offset) in range.clone().enumerate() {
                write!(output, "args[{}]=stack[{}];", i, offset)?;
            }
            write!(output, "let ret=_{}(args);", routine)?;
            for i in 0..routine_.return_size {
                write!(output, "RETURN[{}]=ret[{}];", i, i)?;
            }
            write!(output, "}}")?;
        }
        Statement::Ret => {
            write!(output, "let mut ret=[0;{}];", routine.return_size);
            for i in 0..routine.return_size {
                write!(output, "ret[{}]=RETURN[{}];", i, i)?;
            }
            write!(output, "return ret")?
        },
        _ => write!(output, "unimplemented!()")?,
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
    let offset = offset
        .as_ref()
        .map(|s| src(s))
        .unwrap_or_else(|| "0".to_string());
    match base {
        Pointer::Absolute(a) => format!("STATIC[{}+{} as usize]", a, offset),
        Pointer::Static(a) => format!("STATIC[{}+{} as usize]", a, offset),
        Pointer::Const(a) => format!("CONST[{}+{} as usize]", a, offset),
        Pointer::Stack(a) => format!("stack[{}+{} as usize]", a, offset),
        Pointer::Return(a) => format!("RETURN[{}+{} as usize]", a, offset),
    }
}
