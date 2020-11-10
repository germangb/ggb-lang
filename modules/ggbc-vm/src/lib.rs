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
    ir::{Destination, Ir, Location, Offset, Pointer, Source, Statement},
};
use registers::Registers;
use stack::{Stack, StackFrame};

pub mod registers;
pub mod stack;

#[derive(Default)]
pub struct Opts {}

pub struct Memory {
    /// Static memory space.
    pub static_: Box<[u8; 0x10000]>,
}

pub struct VM<'a, B: ByteOrder> {
    #[warn(unused)]
    opts: Opts,
    running: bool,
    ir: &'a Ir,
    // Index of the routine within Self::ir being run.
    routine: Stack<usize>,
    // Index of the instruction to run next (program counter).
    pc: Stack<usize>,
    memory: Memory,
    // Stack,
    stack: Stack<StackFrame>,
    // 8-bit registers.
    reg8: Registers<u8>,
    // 16-bit registers.
    reg16: Registers<u16>,
    _phantom: std::marker::PhantomData<B>,
}

impl<'a, B: ByteOrder> VM<'a, B> {
    /// Create a new VM to run the IR statements.
    pub fn new(ir: &'a Ir, opts: Opts) -> Self {
        Self { opts,
               running: true,
               ir,
               routine: Stack::new(),
               pc: vec![0],
               memory: Memory { static_: Box::new([0; 0x10000]) },
               stack: vec![StackFrame::new()],
               reg8: Registers::new(),
               reg16: Registers::new(),
               _phantom: std::marker::PhantomData }
    }

    /// Program counter.
    pub fn pc(&self) -> usize {
        self.pc.last().copied().unwrap()
    }

    /// Return static memory space.
    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    fn update(&mut self) {
        //println!("{:04x}", self.pc.last().unwrap());
        if self.running {
            let routine = self.routine
                              .last()
                              .map(|i| &self.ir.routines[*i])
                              .unwrap_or(&self.ir.routines[self.ir.handlers.main]);

            let statement = &routine.statements[self.pc()] as *const _;
            // FIXME refactor to avoid unsafe :(
            self.execute(unsafe { &(*statement) });
            *self.pc.last_mut().unwrap() += 1;
        }
    }

    pub fn run(mut self) -> Memory {
        while self.running {
            self.update()
        }
        self.memory
    }

    fn execute(&mut self, statement: &Statement) {
        use Statement::{
            Add, AddW, And, AndW, Call, Dec, DecW, Div, Eq, Greater, GreaterEq, Inc, IncW, Jmp,
            JmpCmp, JmpCmpNot, Ld, LdW, LeftShift, Less, LessEq, Mul, Nop, NotEq, Or, OrW, Ret,
            RightShift, Stop, Sub, SubW, Xor, XorW,
        };

        match statement {
            Nop(_) => {}
            Stop => self.running = false,

            // store and load
            Ld { source,
                 destination, } => self.ld(source, destination),
            LdW { source,
                  destination, } => self.ld16(source, destination),
            // unary arithmetic
            Inc { source,
                  destination, } => self.inc(source, destination),
            Dec { source,
                  destination, } => self.dec(source, destination),

            IncW { source,
                   destination, } => self.inc16(source, destination),
            DecW { source,
                   destination, } => self.dec16(source, destination),
            // binary arithmetic
            Add { left,
                  right,
                  destination, } => self.add(left, right, destination),
            Sub { left,
                  right,
                  destination, } => self.sub(left, right, destination),
            And { left,
                  right,
                  destination, } => self.and(left, right, destination),
            Or { left,
                 right,
                 destination, } => self.or(left, right, destination),
            Xor { left,
                  right,
                  destination, } => self.xor(left, right, destination),
            Mul { left,
                  right,
                  destination, } => self.mul(left, right, destination),
            Div { left,
                  right,
                  destination, } => self.div(left, right, destination),
            LeftShift { left,
                        right,
                        destination, } => self.left_shift(left, right, destination),
            RightShift { left,
                         right,
                         destination, } => self.right_shift(left, right, destination),
            // comparators
            Eq { left,
                 right,
                 destination, } => self.eq(left, right, destination),
            NotEq { left,
                    right,
                    destination, } => self.not_eq(left, right, destination),
            Greater { left,
                      right,
                      destination, } => self.greater(left, right, destination),
            GreaterEq { left,
                        right,
                        destination, } => self.greater_eq(left, right, destination),
            Less { left,
                   right,
                   destination, } => self.less(left, right, destination),
            LessEq { left,
                     right,
                     destination, } => self.less_eq(left, right, destination),
            // 16bit alu
            AddW { left,
                   right,
                   destination, } => self.add16(left, right, destination),
            SubW { left,
                   right,
                   destination, } => self.sub16(left, right, destination),
            AndW { left,
                   right,
                   destination, } => self.and16(left, right, destination),
            OrW { left,
                  right,
                  destination, } => self.or16(left, right, destination),
            XorW { left,
                   right,
                   destination, } => self.xor16(left, right, destination),
            // flow control
            Jmp { location } => self.jmp(location),
            JmpCmp { location, source } => self.cmp(source, location),
            JmpCmpNot { location, source } => self.cmp_not(source, location),
            // routine and stack frame control
            Call { routine, .. } => self.call(*routine),
            Ret => self.ret(),

            _ => unimplemented!("{:?}", statement),
        }
    }

    fn current_stack_frame(&self) -> &StackFrame {
        self.stack.last().unwrap()
    }

    fn current_stack_frame_mut(&mut self) -> &mut StackFrame {
        self.stack.last_mut().unwrap()
    }

    fn call(&mut self, routine: usize) {
        self.routine.push(routine);
        self.pc.push(0);
        self.stack.push(StackFrame::new());
    }

    fn ret(&mut self) {
        self.routine.pop().unwrap();
        self.pc.pop().unwrap();
        self.stack.pop().unwrap();
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
                let mut pc_signed = self.pc() as isize;
                pc_signed += *rel as isize;
                *self.pc.last_mut().unwrap() = pc_signed as _;
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

    fn offset(&self, offset: &Offset) -> u16 {
        match offset {
            Offset::U8(o) => self.read(o) as u16,
            Offset::U16(o) => self.read_u16(o),
        }
    }

    fn ld(&mut self, source: &Source<u8>, destination: &Destination) {
        // load data from source
        let data = self.read(source);
        // store byte on the destination
        match destination {
            Destination::Pointer { base, offset } => {
                let offset = offset.as_ref().map(|o| self.offset(o) as u16).unwrap_or(0);
                match base {
                    Pointer::Absolute(addr) => {
                        self.memory.static_[(*addr + offset) as usize] = data
                    }
                    Pointer::Static(addr) => self.memory.static_[(*addr + offset) as usize] = data,
                    // TODO don't panic, rather stop the VM and log the error
                    Pointer::Const(_) => panic!("Attempted to write to ROM memory!"),
                    Pointer::Stack(addr) => {
                        self.current_stack_frame_mut()[(*addr + offset) as usize] = data
                    }
                }
            }
            Destination::Register(reg) => self.reg8.set(*reg, data),
        }
    }

    // FIXME code repetition with Self::ld (use traits instead)
    fn ld16(&mut self, source: &Source<u16>, destination: &Destination) {
        // load data from source
        let data = self.read_u16(source);
        // store byte on the destination
        match destination {
            Destination::Pointer { base, offset } => {
                let offset = offset.as_ref().map(|o| self.offset(o)).unwrap_or(0);
                match base {
                    Pointer::Absolute(addr) => {
                        B::write_u16(&mut self.memory.static_[(*addr + offset) as usize..], data)
                    }
                    Pointer::Static(addr) => {
                        B::write_u16(&mut self.memory.static_[(*addr + offset) as usize..], data)
                    }
                    // TODO don't panic, rather stop the VM and log the error
                    Pointer::Const(_) => panic!("Attempted to write to ROM memory!"),
                    Pointer::Stack(addr) => B::write_u16(&mut self.current_stack_frame_mut()
                                                             [(*addr + offset) as usize..],
                                                         data),
                }
            }
            Destination::Register(reg) => self.reg16.set(*reg, data),
        }
    }

    fn read(&self, source: &Source<u8>) -> u8 {
        match source {
            Source::Pointer { base, offset } => {
                let offset = offset.as_ref().map(|o| self.offset(o)).unwrap_or(0);
                match base {
                    Pointer::Absolute(addr) => self.memory.static_[(*addr + offset) as usize],
                    Pointer::Static(addr) => self.memory.static_[(*addr + offset) as usize],
                    Pointer::Const(addr) => self.ir.const_[(*addr + offset) as usize],
                    Pointer::Stack(addr) => self.current_stack_frame()[(*addr + offset) as usize],
                }
            }
            Source::Register(reg) => self.reg8.get(*reg),
            Source::Literal(val) => *val,
        }
    }

    fn read_u16(&self, source: &Source<u16>) -> u16 {
        match source {
            Source::Pointer { base: ptr, offset } => {
                let offset = offset.as_ref().map(|o| self.offset(o)).unwrap_or(0);
                match ptr {
                    Pointer::Absolute(addr) => {
                        B::read_u16(&self.memory.static_[(*addr + offset) as usize..])
                    }
                    Pointer::Static(addr) => {
                        B::read_u16(&self.memory.static_[(*addr + offset) as usize..])
                    }
                    Pointer::Const(addr) => {
                        B::read_u16(&self.ir.const_[(*addr + offset) as usize..])
                    }
                    Pointer::Stack(addr) => {
                        B::read_u16(&self.current_stack_frame()[(*addr + offset) as usize..])
                    }
                }
            }
            Source::Register(reg) => self.reg16.get(*reg),
            Source::Literal(val) => *val,
        }
    }
}
