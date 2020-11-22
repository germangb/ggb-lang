#![deny(clippy::all,
        clippy::doc_markdown,
        clippy::dbg_macro,
        clippy::todo,
        clippy::empty_enum,
        clippy::enum_glob_use,
        clippy::pub_enum_variant_names,
        clippy::mem_forget,
        clippy::use_self,
        clippy::filter_map_next,
        clippy::needless_continue,
        clippy::needless_borrow,
        unused,
        rust_2018_idioms,
        future_incompatible,
        nonstandard_style)]

use ggbc::{
    byteorder::ByteOrder,
    ir::{Destination, Ir, Location, Pointer, Source, Statement},
};
use memory::Memory;
use registers::Registers;
use std::ops::Range;

pub mod memory;
pub mod registers;

type Stack<T> = Vec<T>;

/// Virtual Machine instantiation params.
#[derive(educe::Educe)]
#[educe(Default)]
pub struct Opts {
    /// Stack memory space size.
    #[educe(Default(expression = "0x10000"))]
    pub stack_size: usize,

    /// Static memory space size.
    #[educe(Default(expression = "0x10000"))]
    pub static_size: usize,

    /// Return memory space size.
    ///
    /// It is a good idea to keep this value small, as most functions will
    /// likely return small values.
    #[educe(Default(expression = "0x10"))]
    pub return_size: usize,

    /// number of virtual registers.
    #[educe(Default(expression = "0x10"))]
    pub registers: usize,
}

/// Virtual machine.
pub struct Machine<'a, B: ByteOrder> {
    running: bool,
    ir: &'a Ir<B>,
    routine: Stack<usize>,
    program_counter: Stack<usize>,
    memory: Memory,
    reg8: Stack<Registers<u8>>,
    reg16: Stack<Registers<u16>>,
    _phantom: std::marker::PhantomData<B>,
}

impl<'a, B: ByteOrder> Machine<'a, B> {
    /// Create a new VM to run the IR statements.
    pub fn new(ir: &'a Ir<B>, opts: Opts) -> Self {
        Self { running: true,
               ir,
               routine: Stack::new(),
               program_counter: vec![0],
               memory: Memory::new(&opts),
               reg8: vec![Registers::with_capacity(opts.registers)],
               reg16: vec![Registers::with_capacity(opts.registers)],
               _phantom: std::marker::PhantomData }
    }

    /// Return the current program counter.
    pub fn program_counter(&self) -> usize {
        *self.program_counter.last().unwrap()
    }

    /// Return memory.
    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    /// Return registers.
    pub fn registers(&self) -> (&Registers<u8>, &Registers<u16>) {
        (self.reg8.last().unwrap(), self.reg16.last().unwrap())
    }

    /// Run virtual machine to completion.
    /// Returns the memory state at the end of the program execution.
    pub fn run(mut self) -> Memory {
        while self.running {
            self.step()
        }
        self.memory
    }

    /// Fetch, decode, and execute next instruction.
    pub fn step(&mut self) {
        if self.running {
            let routine = self.routine
                              .last()
                              .map(|i| &self.ir.routines[*i])
                              .unwrap_or(&self.ir.routines[self.ir.handlers.main]);

            let statement = &routine.statements[self.program_counter()].clone();
            self.execute(&statement);
            *self.program_counter.last_mut().unwrap() += 1;
        }
    }

    #[rustfmt::skip]
    fn execute(&mut self, statement: &Statement) {
        use Statement::{
            Add, AddW, And, AndW, Call, Dec, DecW, Div, DivW, Eq, Greater, GreaterEq, Inc, IncW,
            Jmp, JmpCmp, JmpCmpNot, Ld, LdW, LeftShift, LeftShiftW, Less, LessEq, Mul, MulW, Nop,
            NotEq, Or, OrW, Rem, RemW, Ret, RightShift, RightShiftW, Stop, Sub, SubW, Xor, XorW,
        };

        match statement {
            Nop(_) => {}
            Stop => self.running = false,

            // store and load instructions
            Ld  { source, destination } => self.ld(source, destination),
            LdW { source, destination } => self.ld16(source, destination),

            // arithmetic unary operators
            Inc  { source, destination } => self.inc(source, destination),
            Dec  { source, destination } => self.dec(source, destination),
            IncW { source, destination } => self.inc16(source, destination),
            DecW { source, destination } => self.dec16(source, destination),

            // arithmetic binary operators
            Add { left, right, destination } => self.add(left, right, destination),
            Sub { left, right, destination } => self.sub(left, right, destination),
            And { left, right, destination } => self.and(left, right, destination),
            Or  { left, right, destination } => self.or(left, right, destination),
            Xor { left, right, destination } => self.xor(left, right, destination),
            Mul { left, right, destination } => self.mul(left, right, destination),
            Div { left, right, destination } => self.div(left, right, destination),
            Rem { left, right, destination } => self.rem(left, right, destination),
            LeftShift  { left, right, destination } => self.left_shift(left, right, destination),
            RightShift { left, right, destination } => self.right_shift(left, right, destination),
            #[warn(unused)] MulW { left, right, destination } => todo!(),
            #[warn(unused)] DivW { left, right, destination } => todo!(),
            #[warn(unused)] RemW { left, right, destination } => todo!(),
            #[warn(unused)] LeftShiftW { left, right, destination } => todo!(),
            #[warn(unused)] RightShiftW { left, right, destination } => todo!(),

            // comparator
            Eq        { left, right, destination } => self.eq(left, right, destination),
            NotEq     { left, right, destination } => self.not_eq(left, right, destination),
            Greater   { left, right, destination } => self.greater(left, right, destination),
            GreaterEq { left, right, destination } => self.greater_eq(left, right, destination),
            Less      { left, right, destination } => self.less(left, right, destination),
            LessEq    { left, right, destination } => self.less_eq(left, right, destination),

            // 16bit alu
            AddW { left, right, destination } => self.add16(left, right, destination),
            SubW { left, right, destination } => self.sub16(left, right, destination),
            AndW { left, right, destination } => self.and16(left, right, destination),
            OrW  { left, right, destination } => self.or16(left, right, destination),
            XorW { left, right, destination } => self.xor16(left, right, destination),

            // flow control
            Jmp       { location } => self.jmp(location),
            JmpCmp    { location, source } => self.cmp(source, location),
            JmpCmpNot { location, source } => self.cmp_not(source, location),

            // routine instructions
            Call { routine, range } => self.call(*routine, range),
            Ret => self.ret(),

            _ => unimplemented!("{:?}", statement),
        }
    }

    fn call(&mut self, routine: usize, range: &Range<u16>) {
        // push registers
        let reg8 = self.reg8.last().unwrap().clone();
        let reg16 = self.reg16.last().unwrap().clone();
        self.reg8.push(reg8);
        self.reg16.push(reg16);

        // push program counter
        self.program_counter.push(0);
        self.routine.push(routine);

        // initialize new stack frame
        let current_stack = self.memory.stack.clone();
        self.memory.stack.push(range.start as usize);
        for i in range.clone() {
            self.memory.stack[(i - range.start) as usize] = current_stack[i as usize];
        }
    }

    fn ret(&mut self) {
        self.routine.pop().unwrap();
        self.program_counter.pop().unwrap();
        self.memory.stack.pop();
        self.reg8.pop().unwrap();
        self.reg16.pop().unwrap();
    }

    fn cmp(&mut self, source: &Source<u8>, location: &Location) {
        if self.read(source) != 0 {
            self.jmp(location)
        }
    }

    fn cmp_not(&mut self, source: &Source<u8>, location: &Location) {
        if self.read(source) == 0 {
            self.jmp(location)
        }
    }

    fn jmp(&mut self, location: &Location) {
        match location {
            Location::Relative(rel) => {
                let mut pc_signed = self.program_counter() as isize;
                pc_signed += *rel as isize;
                *self.program_counter.last_mut().unwrap() = pc_signed as _;
            }
        }
    }

    fn and(&mut self, left: &Source<u8>, right: &Source<u8>, destination: &Destination) {
        let left = self.read(left);
        let right = self.read(right);
        self.ld(&Source::Literal(left & right), destination);
    }

    fn or(&mut self, left: &Source<u8>, right: &Source<u8>, destination: &Destination) {
        let left = self.read(left);
        let right = self.read(right);
        self.ld(&Source::Literal(left | right), destination);
    }

    fn xor(&mut self, left: &Source<u8>, right: &Source<u8>, destination: &Destination) {
        let left = self.read(left);
        let right = self.read(right);
        self.ld(&Source::Literal(left ^ right), destination);
    }

    fn mul(&mut self, left: &Source<u8>, right: &Source<u8>, destination: &Destination) {
        let left = self.read(left);
        let right = self.read(right);
        self.ld(&Source::Literal(left * right), destination);
    }

    fn div(&mut self, left: &Source<u8>, right: &Source<u8>, destination: &Destination) {
        let left = self.read(left);
        let right = self.read(right);
        self.ld(&Source::Literal(left / right), destination);
    }

    fn rem(&mut self, left: &Source<u8>, right: &Source<u8>, destination: &Destination) {
        let left = self.read(left);
        let right = self.read(right);
        self.ld(&Source::Literal(left % right), destination);
    }

    fn left_shift(&mut self, left: &Source<u8>, right: &Source<u8>, destination: &Destination) {
        let left = self.read(left);
        let right = self.read(right);
        self.ld(&Source::Literal(left << right), destination);
    }

    fn right_shift(&mut self, left: &Source<u8>, right: &Source<u8>, destination: &Destination) {
        let left = self.read(left);
        let right = self.read(right);
        self.ld(&Source::Literal(left >> right), destination);
    }

    fn eq(&mut self, left: &Source<u8>, right: &Source<u8>, destination: &Destination) {
        let left = self.read(left);
        let right = self.read(right);
        self.ld(&Source::Literal(if left == right { 1 } else { 0 }),
                destination);
    }

    fn not_eq(&mut self, left: &Source<u8>, right: &Source<u8>, destination: &Destination) {
        let left = self.read(left);
        let right = self.read(right);
        self.ld(&Source::Literal(if left != right { 1 } else { 0 }),
                destination);
    }

    fn greater(&mut self, left: &Source<u8>, right: &Source<u8>, destination: &Destination) {
        let left = self.read(left);
        let right = self.read(right);
        self.ld(&Source::Literal(if left > right { 1 } else { 0 }),
                destination);
    }

    fn greater_eq(&mut self, left: &Source<u8>, right: &Source<u8>, destination: &Destination) {
        let left = self.read(left);
        let right = self.read(right);
        self.ld(&Source::Literal(if left >= right { 1 } else { 0 }),
                destination);
    }

    fn less(&mut self, left: &Source<u8>, right: &Source<u8>, destination: &Destination) {
        let left = self.read(left);
        let right = self.read(right);
        self.ld(&Source::Literal(if left < right { 1 } else { 0 }),
                destination);
    }

    fn less_eq(&mut self, left: &Source<u8>, right: &Source<u8>, destination: &Destination) {
        let left = self.read(left);
        let right = self.read(right);
        self.ld(&Source::Literal(if left <= right { 1 } else { 0 }),
                destination);
    }

    fn add(&mut self, left: &Source<u8>, right: &Source<u8>, destination: &Destination) {
        let left = self.read(left);
        let right = self.read(right);
        //println!("{:?} = {} + {}", destination, left, right);
        self.ld(&Source::Literal(left.wrapping_add(right)), destination);
    }

    fn sub(&mut self, left: &Source<u8>, right: &Source<u8>, destination: &Destination) {
        let left = self.read(left);
        let right = self.read(right);
        self.ld(&Source::Literal(left.wrapping_sub(right)), destination);
    }

    fn and16(&mut self, left: &Source<u16>, right: &Source<u16>, destination: &Destination) {
        let left = self.read_u16(left);
        let right = self.read_u16(right);
        self.ld16(&Source::Literal(left & right), destination);
    }

    fn or16(&mut self, left: &Source<u16>, right: &Source<u16>, destination: &Destination) {
        let left = self.read_u16(left);
        let right = self.read_u16(right);
        self.ld16(&Source::Literal(left | right), destination);
    }

    fn xor16(&mut self, left: &Source<u16>, right: &Source<u16>, destination: &Destination) {
        let left = self.read_u16(left);
        let right = self.read_u16(right);
        self.ld16(&Source::Literal(left ^ right), destination);
    }

    fn add16(&mut self, left: &Source<u16>, right: &Source<u16>, destination: &Destination) {
        let left = self.read_u16(left);
        let right = self.read_u16(right);
        self.ld16(&Source::Literal(left.wrapping_add(right)), destination);
    }

    fn sub16(&mut self, left: &Source<u16>, right: &Source<u16>, destination: &Destination) {
        let left = self.read_u16(left);
        let right = self.read_u16(right);
        self.ld16(&Source::Literal(left.wrapping_sub(right)), destination);
    }

    fn inc(&mut self, source: &Source<u8>, destination: &Destination) {
        let data = self.read(source).wrapping_add(1);
        self.ld(&Source::Literal(data), destination);
    }

    fn dec(&mut self, source: &Source<u8>, destination: &Destination) {
        let data = self.read(source).wrapping_sub(1);
        self.ld(&Source::Literal(data), destination);
    }

    fn inc16(&mut self, source: &Source<u16>, destination: &Destination) {
        let data = self.read_u16(source).wrapping_add(1);
        self.ld16(&Source::Literal(data), destination);
    }

    fn dec16(&mut self, source: &Source<u16>, destination: &Destination) {
        let data = self.read_u16(source).wrapping_sub(1);
        self.ld16(&Source::Literal(data), destination);
    }

    fn ld(&mut self, source: &Source<u8>, destination: &Destination) {
        use Pointer::{Absolute, Const, Return, Stack, Static};
        let data = self.read(source);
        match destination {
            Destination::Pointer { base, offset } => {
                let offset = offset.as_ref().map(|o| self.read(o) as u16).unwrap_or(0) as u16;
                match base {
                    Absolute(addr) => self.memory.static_[(*addr + offset) as usize] = data,
                    Static(addr) => self.memory.static_[(*addr + offset) as usize] = data,
                    Return(addr) => self.memory.return_[(*addr + offset) as usize] = data,
                    // TODO don't panic, rather stop the VM and log the error
                    Const(_) => panic!("Attempted to write to ROM memory!"),
                    Stack(addr) => self.memory.stack[(*addr + offset) as usize] = data,
                }
            }
            Destination::Register(reg) => self.reg8.last_mut().unwrap().set(*reg, data),
        }
    }

    // FIXME code repetition with Self::ld (use traits instead)
    fn ld16(&mut self, source: &Source<u16>, destination: &Destination) {
        use Pointer::{Absolute, Const, Return, Stack, Static};
        // load data from source
        let data = self.read_u16(source);
        // store byte on the destination
        match destination {
            Destination::Pointer { base, offset } => {
                let offset = offset.as_ref().map(|o| self.read(o)).unwrap_or(0) as u16;
                match base {
                    Absolute(addr) => {
                        B::write_u16(&mut self.memory.static_[(*addr + offset) as usize..], data)
                    }
                    Static(addr) => {
                        B::write_u16(&mut self.memory.static_[(*addr + offset) as usize..], data)
                    }
                    Return(addr) => {
                        B::write_u16(&mut self.memory.return_[(*addr + offset) as usize..], data)
                    }
                    // TODO don't panic, rather stop the VM and log the error
                    Const(_) => panic!("Attempted to write to ROM memory!"),
                    Stack(addr) => {
                        B::write_u16(&mut self.memory.stack[(*addr + offset) as usize..], data)
                    }
                }
            }
            Destination::Register(reg) => self.reg16.last_mut().unwrap().set(*reg, data),
        }
    }

    fn read(&self, source: &Source<u8>) -> u8 {
        use Pointer::{Absolute, Const, Return, Stack, Static};
        match source {
            Source::Pointer { base, offset } => {
                let offset = offset.as_ref().map(|o| self.read(o)).unwrap_or(0) as u16;
                match base {
                    Absolute(addr) => self.memory.static_[(*addr + offset) as usize],
                    Static(addr) => self.memory.static_[(*addr + offset) as usize],
                    Return(addr) => self.memory.return_[(*addr + offset) as usize],
                    Const(addr) => self.ir.const_[(*addr + offset) as usize],
                    Stack(addr) => self.memory.stack[(*addr + offset) as usize],
                }
            }
            Source::Register(reg) => self.reg8.last().unwrap().get(*reg),
            Source::Literal(val) => *val,
        }
    }

    fn read_u16(&self, source: &Source<u16>) -> u16 {
        use Pointer::{Absolute, Const, Return, Stack, Static};
        match source {
            Source::Pointer { base: ptr, offset } => {
                let offset = offset.as_ref().map(|o| self.read(o)).unwrap_or(0) as u16;
                match ptr {
                    Absolute(addr) => {
                        B::read_u16(&self.memory.static_[(*addr + offset) as usize..])
                    }
                    Static(addr) => B::read_u16(&self.memory.static_[(*addr + offset) as usize..]),
                    Return(addr) => B::read_u16(&self.memory.return_[(*addr + offset) as usize..]),
                    Const(addr) => B::read_u16(&self.ir.const_[(*addr + offset) as usize..]),
                    Stack(addr) => B::read_u16(&self.memory.stack[(*addr + offset) as usize..]),
                }
            }
            Source::Register(reg) => self.reg16.last().unwrap().get(*reg),
            Source::Literal(val) => *val,
        }
    }
}
