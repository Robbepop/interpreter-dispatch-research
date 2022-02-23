mod closure_block;
mod closure_loop;
mod closure_tail;
mod closure_tree;
// mod closure_tree;
mod enum_tree;
mod enum_tree_2;
mod fused;
mod switch;
mod switch_tail;
mod switch_tail_2;

pub type Register = usize;
pub type Bits = u64;
pub type Target = usize;

use std::time::{Duration, Instant};

pub fn benchmark<F, R>(f: F) -> (Duration, R)
where
    F: FnOnce() -> R,
{
    let before = Instant::now();
    let result = f();
    let duration = before.elapsed();
    println!("duration = {:?}", duration);
    (duration, result)
}

/// The outcome of an instruction execution.
#[derive(Copy, Clone)]
pub enum Outcome {
    /// Continue with the next instruction pointed to by the `pc`.
    Continue,
    /// Return function execution.
    Return,
}

/// A simple execution context with a program counter and some registers.
pub struct Context {
    pc: usize,
    regs: Vec<Bits>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            pc: 0,
            regs: vec![0x00; 16],
        }
    }
}

impl Context {
    /// Sets the register `reg` to the `new_value`.
    pub fn set_reg(&mut self, reg: Register, new_value: Bits) {
        debug_assert!(reg < self.regs.len());
        unsafe {
            *self.regs.get_unchecked_mut(reg) = new_value;
        }
    }

    /// Returns the current value of `reg`.
    pub fn get_reg(&self, reg: Register) -> Bits {
        debug_assert!(reg < self.regs.len());
        unsafe { *self.regs.get_unchecked(reg) }
    }

    /// Sets the `pc` to point to the `new_pc`.
    pub fn branch_to(&mut self, new_pc: usize) -> Outcome {
        self.pc = new_pc;
        Outcome::Continue
    }

    /// Advance the `pc` to the next instruction.
    pub fn next_inst(&mut self) -> Outcome {
        self.pc += 1;
        Outcome::Continue
    }
}

mod handler {
    use super::{Bits, Context, Outcome, Register};

    pub fn add(context: &mut Context, result: Register, lhs: Register, rhs: Register) -> Outcome {
        let lhs = context.get_reg(lhs);
        let rhs = context.get_reg(rhs);
        context.set_reg(result, lhs.wrapping_add(rhs));
        context.next_inst()
    }

    pub fn add_imm(context: &mut Context, result: Register, src: Register, imm: Bits) -> Outcome {
        let lhs = context.get_reg(src);
        let rhs = imm;
        context.set_reg(result, lhs.wrapping_add(rhs));
        context.next_inst()
    }

    pub fn sub(context: &mut Context, result: Register, lhs: Register, rhs: Register) -> Outcome {
        let lhs = context.get_reg(lhs);
        let rhs = context.get_reg(rhs);
        context.set_reg(result, lhs.wrapping_sub(rhs));
        context.next_inst()
    }

    pub fn sub_imm(context: &mut Context, result: Register, src: Register, imm: Bits) -> Outcome {
        let lhs = context.get_reg(src);
        let rhs = imm;
        context.set_reg(result, lhs.wrapping_sub(rhs));
        context.next_inst()
    }

    pub fn mul(context: &mut Context, result: Register, lhs: Register, rhs: Register) -> Outcome {
        let lhs = context.get_reg(lhs);
        let rhs = context.get_reg(rhs);
        context.set_reg(result, lhs.wrapping_mul(rhs));
        context.next_inst()
    }

    pub fn mul_imm(context: &mut Context, result: Register, src: Register, imm: Bits) -> Outcome {
        let lhs = context.get_reg(src);
        let rhs = imm;
        context.set_reg(result, lhs.wrapping_mul(rhs));
        context.next_inst()
    }

    pub fn branch(context: &mut Context, target: Register) -> Outcome {
        context.branch_to(target as usize)
    }

    pub fn branch_eqz(context: &mut Context, target: Register, condition: Register) -> Outcome {
        let condition = context.get_reg(condition);
        if condition == 0 {
            context.branch_to(target as usize)
        } else {
            context.next_inst()
        }
    }

    pub fn ret(context: &mut Context, result: Register) -> Outcome {
        let result = context.get_reg(result);
        context.set_reg(0, result);
        Outcome::Return
    }
}
