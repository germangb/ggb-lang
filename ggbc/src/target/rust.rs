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

        writeln!(&mut output,
                 "static CONST:[u8;{}] = {:?};",
                 ir.const_.len(),
                 ir.const_)?;
        writeln!(&mut output,
                 "static mut STATIC:[u8;{}] = [0; {}];",
                 ir.static_alloc, ir.static_alloc)?;
        writeln!(&mut output, "static mut REGISTERS:[u8;16] = [0;16];")?;

        for routine in ir.routines.iter() {
            codegen_routine(&ir.routines, &mut output, routine)?;
        }
        writeln!(&mut output, "fn main() {{ unsafe {{ _main([]); }} }}")?;
        Ok(String::from_utf8(output).unwrap())
    }
}

#[warn(unused)]
fn codegen_routine(routines: &[Routine],
                   output: &mut Vec<u8>,
                   routine: &Routine)
                   -> Result<(), std::io::Error> {
    writeln!(output,
             "unsafe fn _{}(args:[u8;{}])->[u8;{}] {{",
             routine.debug_name.as_ref().unwrap(),
             routine.args_size,
             routine.return_size)?;
    writeln!(output, "    let mut RETURN=[0;256];")?;
    writeln!(output, "    let mut STACK=[0;256];")?;
    for i in 0..routine.args_size {
        writeln!(output, "    STACK[{}] = args[{}];", i, i)?;
    }
    writeln!(output, "    let mut ret=[0;{}];", routine.return_size)?;
    writeln!(output, "    let mut pc = 0;")?;
    writeln!(output, "    loop {{")?;
    writeln!(output, "        match pc {{")?;
    for (i, statement) in routine.statements.iter().enumerate() {
        write!(output, "            {} => ", i)?;
        //write!(output, "{{")?;
        codegen_statement(routines, output, statement)?;
        //write!(output, "}}")?;
        write!(output, ",")?;
        //write!(output, " // {}", statement.display())?;
        writeln!(output);
    }
    writeln!(output, "            _ => panic!(),")?;
    writeln!(output, "        }}")?;
    writeln!(output, "        pc += 1;")?;
    writeln!(output, "    }}")?;
    writeln!(output, "    ret")?;
    writeln!(output, "}}")?;
    Ok(())
}

#[warn(unused)]
fn codegen_statement(routines: &[Routine],
                     output: &mut Vec<u8>,
                     statement: &Statement)
                     -> Result<(), std::io::Error> {
    use Statement::*;
    match statement {
        Nop(_) => write!(output, "{{}}")?,
        Stop => write!(output, "std::process::exit(0)")?,
        Ld { source,
             destination, } => write!(output, "{}={}", dest(destination), src(source))?,
        Inc { source,
              destination, } => write!(output,
                                       "{}=({} as u8).wrapping_add(1u8)",
                                       dest(destination),
                                       src(source))?,
        Dec { source,
              destination, } => write!(output,
                                       "{}=({} as u8).wrapping_sub(1u8)",
                                       dest(destination),
                                       src(source))?,
        Add { destination,
              left,
              right, } => write!(output,
                                 "{}=({} as u8).wrapping_add({} as u8)",
                                 dest(destination),
                                 src(left),
                                 src(right))?,
        Sub { destination,
              left,
              right, } => write!(output,
                                 "{}=({} as u8).wrapping_sub({} as u8)",
                                 dest(destination),
                                 src(left),
                                 src(right))?,
        And { destination,
              left,
              right, } => write!(output, "{}={}&{}", dest(destination), src(left), src(right))?,
        Xor { destination,
              left,
              right, } => write!(output, "{}={}^{}", dest(destination), src(left), src(right))?,
        Or { destination,
             left,
             right, } => write!(output, "{}={}|{}", dest(destination), src(left), src(right))?,
        LeftShift { destination,
                    left,
                    right, } => write!(output,
                                       "{}={}<<{}",
                                       dest(destination),
                                       src(left),
                                       src(right))?,
        RightShift { destination,
                     left,
                     right, } => write!(output,
                                        "{}={}>>{}",
                                        dest(destination),
                                        src(left),
                                        src(right))?,
        Mul { destination,
              left,
              right, } => write!(output,
                                 "{}={}.wrapping_mul({})",
                                 dest(destination),
                                 src(left),
                                 src(right))?,
        Div { destination,
              left,
              right, } => write!(output,
                                 "{}={}.wrapping_div({})",
                                 dest(destination),
                                 src(left),
                                 src(right))?,
        Rem { destination,
              left,
              right, } => write!(output,
                                 "{}={}.wrapping_rem({})",
                                 dest(destination),
                                 src(left),
                                 src(right))?,
        Eq { destination,
             left,
             right, } => write!(output,
                                "{}=if {}=={}{{1}}else{{0}}",
                                dest(destination),
                                src(left),
                                src(right))?,
        NotEq { destination,
                left,
                right, } => write!(output,
                                   "{}=if {}!={}{{1}}else{{0}}",
                                   dest(destination),
                                   src(left),
                                   src(right))?,
        Greater { destination,
                  left,
                  right, } => write!(output,
                                     "{}=if {}>{}{{1}}else{{0}}",
                                     dest(destination),
                                     src(left),
                                     src(right))?,
        GreaterEq { destination,
                    left,
                    right, } => write!(output,
                                       "{}=if {}>={}{{1}}else{{0}}",
                                       dest(destination),
                                       src(left),
                                       src(right))?,
        Less { destination,
               left,
               right, } => write!(output,
                                  "{}=if {}<{}{{1}}else{{0}}",
                                  dest(destination),
                                  src(left),
                                  src(right))?,
        LessEq { destination,
                 left,
                 right, } => write!(output,
                                    "{}=if {}<={}{{1}}else{{0}}",
                                    dest(destination),
                                    src(left),
                                    src(right))?,
        Jmp { location: Location::Relative(r), } => {
            if *r >= 0 {
                write!(output, "pc += {}", r)?
            } else {
                write!(output, "pc -= {}", -r)?
            }
        }
        JmpCmp { location: Location::Relative(r),
                 source, } => write!(output, "{{}}")?,
        JmpCmpNot { location: Location::Relative(r),
                    source, } => write!(output, "{{}}")?,
        Call { routine, range } => {
            let routine = &routines[*routine];
            write!(output, "{{")?;
            write!(output,
                   "let args:[u8;{}] = [0; {}];",
                   routine.args_size, routine.args_size)?;
            write!(output, "}}")?;
        }
        Ret => write!(output, "return ret")?,
        _ => write!(output, "unimplemented!()")?,
    }
    Ok(())
}

#[warn(unused)]
fn dest(destination: &Destination) -> String {
    use Destination as D;
    match destination {
        Destination::Pointer { base, offset } => {
            let offset = offset.as_ref()
                               .map(|s| src(s))
                               .unwrap_or_else(|| "0".to_string());
            match base {
                Pointer::Absolute(a) => todo!(),
                Pointer::Static(a) => format!("STATIC[{}+{} as usize]", a, offset),
                Pointer::Const(a) => format!("CONST[{}+{} as usize]", a, offset),
                Pointer::Stack(a) => format!("STACK[{}+{} as usize]", a, offset),
                Pointer::Return(a) => format!("ret[{}+{} as usize]", a, offset),
            }
        }
        Destination::Register(register) => format!("REGISTERS[{}]", register),
    }
}

#[warn(unused)]
fn src(source: &Source<u8>) -> String {
    use Source as S;
    match source {
        S::Pointer { base, offset } => {
            let offset = offset.as_ref()
                               .map(|s| src(s))
                               .unwrap_or_else(|| "0".to_string());
            match base {
                Pointer::Absolute(a) => todo!(),
                Pointer::Static(a) => format!("STATIC[{}+{} as usize]", a, offset),
                Pointer::Const(a) => format!("CONST[{}+{} as usize]", a, offset),
                Pointer::Stack(a) => format!("STACK[{}+{} as usize]", a, offset),
                Pointer::Return(a) => format!("RETURN[{}+{} as usize]", a, offset),
            }
        }
        S::Register(register) => format!("REGISTERS[{}]", register),
        S::Literal(literal) => format!("{}", literal),
    }
}
