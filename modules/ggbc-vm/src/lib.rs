use byteorder::ByteOrder;
use ggbc::ir::{Destination, Ir, Location, Pointer, Source, Statement};
use registers::Registers;
use stack::{Stack, StackFrame};
use std::{fs::read, io::Cursor};

pub mod memory;
pub mod registers;
pub mod stack;

use crate::memory::{Absolute, Static};
pub use byteorder;

pub struct VM<B: ByteOrder = byteorder::NativeEndian> {
    running: bool,
    /// Intermediate representation being run.
    ir: Ir,
    /// Index of the routine within Self::ir being run.
    routine: Stack<usize>,
    /// Index of the instruction to run next (program counter).
    pc: Stack<usize>,
    /// Memory space corresponding to the Absolute memory space.
    absolute: Absolute,
    /// Static memory space.
    static_: Static,
    /// Stack,
    stack: Stack<StackFrame>,
    /// 8-bit registers.
    reg8: Registers<u8>,
    /// 16-bit registers.
    reg16: Registers<u16>,
    _phantom: std::marker::PhantomData<B>,
}

impl<B: ByteOrder> VM<B> {
    /// Create a new VM to run the IR statements.
    pub fn new(ir: Ir) -> Self {
        // init current-routine stack
        let mut routine = Stack::new();
        routine.push(ir.main);

        // initialize stack with an empty stack frame
        let mut stack = Stack::new();
        stack.push(StackFrame::new());

        // init PC
        let mut pc = Stack::new();
        pc.push(0);

        Self { running: true,
               ir,
               routine,
               pc,
               // initialize absolute memory with zeroes.
               absolute: Box::new([0; 0x10000]),
               static_: Box::new([0; 0x10000]),
               stack,
               reg8: Registers::new(),
               reg16: Registers::new(),
               _phantom: std::marker::PhantomData }
    }

    pub fn running(&self) -> bool {
        self.running
    }

    /// Program counter.
    pub fn pc(&self) -> usize {
        self.pc.last().copied().unwrap()
    }

    /// Return absolute memory space.
    pub fn absolute(&self) -> &Absolute {
        &self.absolute
    }

    /// Return static memory space.
    pub fn statik(&self) -> &Static {
        &self.static_
    }

    /// Updates state of the VM:
    /// 1. Fetches the next statement (current PC).
    /// 2. Executes statement.
    /// 3. Advances PC.
    pub fn update(&mut self) {
        if self.running {
            let routine_index = self.routine.last().unwrap();
            let routine = &self.ir.routines[*routine_index];

            let statement = &routine.statements[self.pc()] as *const _;
            // FIXME refactor to avoid unsafe :(
            self.execute(unsafe { &(*statement) });
            *self.pc.last_mut().unwrap() += 1;
        }
    }

    fn execute(&mut self, statement: &Statement) {
        use Statement::*;

        match statement {
            Nop(_) => {}
            Stop => self.running = false,

            // store and load
            Ld { source,
                 destination, } => self.ld(source, destination),
            Ld16 { source,
                   destination, } => self.ld16(source, destination),

            // unary arithmetic
            Inc { source,
                  destination, } => self.inc(source, destination),
            Dec { source,
                  destination, } => self.dec(source, destination),

            Inc16 { source,
                    destination, } => self.inc16(source, destination),
            Dec16 { source,
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

            Add16 { left,
                    right,
                    destination, } => self.add16(left, right, destination),
            Sub16 { left,
                    right,
                    destination, } => self.sub16(left, right, destination),
            And16 { left,
                    right,
                    destination, } => self.and16(left, right, destination),
            Or16 { left,
                   right,
                   destination, } => self.or16(left, right, destination),
            Xor16 { left,
                    right,
                    destination, } => self.xor16(left, right, destination),

            // flow control
            Jmp { location } => self.jmp(location),
            Cmp { location, source } => self.cmp(source, location),
            CmpNot { location, source } => self.cmp_not(source, location),

            // routine and stack frame control
            Call { routine, .. } => self.call(*routine),
            Ret => self.ret(),
            Push => self.push(),
            Pop => self.pop(),

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
    }

    fn ret(&mut self) {
        self.routine.pop().unwrap();
        self.pc.pop().unwrap();
    }

    fn push(&mut self) {
        self.stack.push(StackFrame::new());
    }

    fn pop(&mut self) {
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

    fn add(&mut self, left: &Source<u8>, right: &Source<u8>, destination: &Destination) {
        let left = self.read(left);
        let right = self.read(right);
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
        // load data from source
        let data = self.read(source);
        // store byte on the destination
        match destination {
            Destination::Pointer(ptr) => match ptr {
                Pointer::Absolute(addr) => self.absolute[*addr as usize] = data,
                Pointer::Static(addr) => self.static_[*addr as usize] = data,
                // TODO don't panic, rather stop the VM and log the error
                Pointer::Const(_) => panic!("Attempted to write to ROM memory!"),
                Pointer::Stack(addr) => self.current_stack_frame_mut()[*addr as usize] = data,
            },
            Destination::Register(reg) => self.reg8.set(*reg, data),
        }
    }

    // FIXME code repetition with Self::ld (use traits instead)
    fn ld16(&mut self, source: &Source<u16>, destination: &Destination) {
        // load data from source
        let data = self.read_u16(source);
        // store byte on the destination
        match destination {
            Destination::Pointer(ptr) => match ptr {
                Pointer::Absolute(addr) => B::write_u16(&mut self.absolute[*addr as usize..], data),
                Pointer::Static(addr) => B::write_u16(&mut self.static_[*addr as usize..], data),
                // TODO don't panic, rather stop the VM and log the error
                Pointer::Const(_) => panic!("Attempted to write to ROM memory!"),
                Pointer::Stack(addr) => {
                    B::write_u16(&mut self.current_stack_frame_mut()[*addr as usize..], data)
                }
            },
            Destination::Register(reg) => self.reg16.set(*reg, data),
        }
    }

    fn read(&self, source: &Source<u8>) -> u8 {
        match source {
            Source::Pointer { base, offset } => match base {
                Pointer::Absolute(addr) => self.absolute[*addr as usize],
                Pointer::Static(addr) => self.static_[*addr as usize],
                Pointer::Const(addr) => self.ir.const_[*addr as usize],
                Pointer::Stack(addr) => self.current_stack_frame()[*addr as usize],
            },
            Source::Register(reg) => self.reg8.get(*reg),
            Source::Literal(val) => *val,
        }
    }

    fn read_u16(&self, source: &Source<u16>) -> u16 {
        match source {
            Source::Pointer { base: ptr, offset } => match ptr {
                Pointer::Absolute(addr) => B::read_u16(&self.absolute[*addr as usize..]),
                Pointer::Static(addr) => B::read_u16(&self.static_[*addr as usize..]),
                Pointer::Const(addr) => B::read_u16(&self.ir.const_[*addr as usize..]),
                Pointer::Stack(addr) => B::read_u16(&self.current_stack_frame()[*addr as usize..]),
            },
            Source::Register(reg) => self.reg16.get(*reg),
            Source::Literal(val) => *val,
        }
    }
}
