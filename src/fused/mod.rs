mod ct;
mod ct2;
mod ct3;
mod rt;
mod rt2;

use crate::{Outcome, Target};

pub type Bits = u64;

pub struct Context {
    pc: usize,
    regs: Vec<Bits>,
    globals: Vec<Bits>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            pc: 0,
            regs: vec![0x00; 16],
            globals: vec![0x00; 16],
        }
    }
}

impl Context {
    pub fn next_inst(&mut self) -> Outcome {
        self.pc += 1;
        Outcome::Continue
    }

    pub fn branch_to(&mut self, target: Target) -> Outcome {
        self.pc = target;
        Outcome::Continue
    }

    pub fn set_reg(&mut self, reg: Register, new_value: Bits) {
        let reg = reg.into_usize();
        debug_assert!(reg < self.regs.len());
        unsafe {
            *self.regs.get_unchecked_mut(reg) = new_value;
        }
    }

    pub fn get_reg(&self, reg: Register) -> Bits {
        let reg = reg.into_usize();
        debug_assert!(reg < self.regs.len());
        unsafe { *self.regs.get_unchecked(reg) }
    }

    pub fn set_global(&mut self, global: Global, new_value: Bits) {
        let global = global.into_usize();
        debug_assert!(global < self.globals.len());
        unsafe {
            *self.globals.get_unchecked_mut(global) = new_value;
        }
    }

    pub fn get_global(&self, global: Global) -> Bits {
        let global = global.into_usize();
        debug_assert!(global > self.globals.len());
        unsafe { *self.globals.get_unchecked(global) }
    }
}

#[derive(Copy, Clone)]
pub struct Register(usize);
impl Register {
    pub fn into_usize(self) -> usize {
        self.0
    }
}

#[derive(Copy, Clone)]
pub struct Global(usize);
impl Global {
    pub fn into_usize(self) -> usize {
        self.0
    }
}

#[derive(Copy, Clone)]
pub struct Const(Bits);
impl Const {
    pub fn into_bits(self) -> Bits {
        self.0
    }
}
