use byteorder::ByteOrder;
use ggbc::ir::{Destination, Ir, Pointer, Source, Statement};
use registers::Registers;
use stack::{Stack, StackFrame};
use std::{fs::read, io::Cursor};

pub mod registers;
pub mod stack;

pub use byteorder;

pub struct VM<B: ByteOrder = byteorder::NativeEndian> {
    running: bool,
    /// Intermediate representation being run.
    ir: Ir,
    /// Index of the routine within Self::ir being run.
    routine: usize,
    /// Index of the instruction to run next (program counter).
    pc: usize,
    /// Memory space corresponding to the Absolute memory space.
    memory: Box<[u8; 0x10000]>,
    /// Static memory space.
    static_: Box<[u8; 0x10000]>,
    /// Stack,
    stack: Stack,
    /// 8-bit registers.
    reg8: Registers<u8>,
    /// 16-bit registers.
    reg16: Registers<u16>,
    _phantom: std::marker::PhantomData<B>,
}

impl<B: ByteOrder> VM<B> {
    /// Create a new VM to run the IR statements.
    pub fn new(ir: Ir) -> Self {
        let routine = ir.main;
        // initialize stack with an empty stack frame
        let mut stack = Stack::new();
        stack.push(StackFrame::new());
        Self { running: true,
               ir,
               routine,
               pc: 0,
               // initialize absolute memory with zeroes.
               memory: Box::new([0; 0x10000]),
               static_: Box::new([0; 0x10000]),
               stack,
               reg8: Registers::new(),
               reg16: Registers::new(),
               _phantom: std::marker::PhantomData }
    }

    pub fn running(&self) -> bool {
        self.running
    }

    /// Updates state of the VM:
    /// 1. Fetches the next statement (current PC).
    /// 2. Executes statement.
    /// 3. Advances PC.
    pub fn update(&mut self) {
        if self.running {
            let routine = &self.ir.routines[self.routine];
            let statement = &routine.statements[self.pc] as *const _;
            // FIXME refactor to avoid unsafe :(
            self.execute(unsafe { &(*statement) })
        }
    }

    fn execute(&mut self, statement: &Statement) {
        use Statement::*;

        match statement {
            Nop => {}
            Stop => self.running = false,

            Ld { source,
                 destination, } => self.ld(source, destination),
            Ld16 { source,
                   destination, } => self.ld16(source, destination),

            _ => unimplemented!(),
        }
    }

    fn current_stack_frame(&self) -> &StackFrame {
        self.stack.last().unwrap()
    }

    fn current_stack_frame_mut(&mut self) -> &mut StackFrame {
        self.stack.last_mut().unwrap()
    }

    fn inc(&mut self, source: &Source<u8>, destination: &Destination) {
        let data = self.read(source) + 1;
        self.ld(&Source::Literal(data), destination);
    }

    fn inc16(&mut self, source: &Source<u16>, destination: &Destination) {
        let data = self.read_u16(source) + 1;
        self.ld16(&Source::Literal(data), destination);
    }

    fn ld(&mut self, source: &Source<u8>, destination: &Destination) {
        // load data from source
        let data = self.read(source);
        // store byte on the destination
        match destination {
            Destination::Pointer(ptr) => match ptr {
                Pointer::Absolute(addr) => self.memory[*addr as usize] = data,
                Pointer::Static(addr) => self.static_[*addr as usize] = data,
                // TODO don't panic, rather stop the VM and log the error
                Pointer::Const(_) => panic!("Attempted to write to ROM memory!"),
                Pointer::Stack(addr) => self.current_stack_frame_mut()[*addr as usize] = data,
            },
            Destination::Register(reg) => self.reg8.set(*reg, data),
        }
    }

    // FIXME code repetition with Self::ld
    fn ld16(&mut self, source: &Source<u16>, destination: &Destination) {
        // load data from source
        let data = self.read_u16(source);
        // store byte on the destination
        match destination {
            Destination::Pointer(ptr) => match ptr {
                Pointer::Absolute(addr) => B::write_u16(&mut self.memory[*addr as usize..], data),
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
            Source::Pointer(ptr) => match ptr {
                Pointer::Absolute(addr) => self.memory[*addr as usize],
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
            Source::Pointer(ptr) => match ptr {
                Pointer::Absolute(addr) => B::read_u16(&self.memory[*addr as usize..]),
                Pointer::Static(addr) => B::read_u16(&self.static_[*addr as usize..]),
                Pointer::Const(addr) => B::read_u16(&self.ir.const_[*addr as usize..]),
                Pointer::Stack(addr) => B::read_u16(&self.current_stack_frame()[*addr as usize..]),
            },
            Source::Register(reg) => self.reg16.get(*reg),
            Source::Literal(val) => *val,
        }
    }
}
