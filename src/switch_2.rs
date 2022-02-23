#![allow(dead_code)]

#[cfg(test)]
use crate::benchmark;

use super::{handler, Bits, Context, Outcome, Register, Target};

#[derive(Copy, Clone)]
pub enum Inst {
    /// Adds the contents of `lhs` and `rhs` and stores the result into `result`.
    Add {
        result: Register,
        lhs: Register,
        rhs: Register,
    },
    /// Adds the constant `imm` and the contents of `src` and stores the result into `result`.
    AddImm {
        result: Register,
        src: Register,
        imm: Bits,
    },
    /// Adds the constant `imm` and the contents of `src` and stores the result into `result`.
    AddImm0 {
        imm: Bits,
    },
    /// Subtracts the contents of `rhs` from `lhs` and stores the result into `result`.
    Sub {
        result: Register,
        lhs: Register,
        rhs: Register,
    },
    /// Subtracts the constant `imm` from the contents of `src` and stores the result into `result`.
    SubImm {
        result: Register,
        src: Register,
        imm: Bits,
    },
    /// Subtracts the constant `imm` from the contents of `src` and stores the result into `result`.
    SubImm0 {
        imm: Bits,
    },
    /// Multiplies the contents of `lhs` and `rhs` and stores the result into `result`.
    Mul {
        result: Register,
        lhs: Register,
        rhs: Register,
    },
    /// Multiplies the constant `imm` and the contents of `src` and stores the result into `result`.
    MulImm {
        result: Register,
        src: Register,
        imm: Bits,
    },
    /// Branches to the instruction indexed by `target`.
    Branch { target: Target },
    /// Branches to the instruction indexed by `target` if the contents of `condition` are zero.
    BranchEqz { target: Target, condition: Register },
    BranchEqz0 { target: Target },
    /// Returns execution of the function and returns the result in `result`.
    Return { result: Register },
}

impl Inst {
    pub fn execute(&self, context: &mut Context, reg0: &mut Bits) -> Outcome {
        match self {
            Inst::Add { result, lhs, rhs } => handler::add(context, *result, *lhs, *rhs),
            Inst::AddImm { result, src, imm } => handler::add_imm(context, *result, *src, *imm),
            Inst::AddImm0 { imm } => {
                *reg0 = reg0.wrapping_add(*imm);
                context.next_inst()
            }
            Inst::Sub { result, lhs, rhs } => handler::sub(context, *result, *lhs, *rhs),
            Inst::SubImm { result, src, imm } => handler::sub_imm(context, *result, *src, *imm),
            Inst::SubImm0 { imm } => {
                *reg0 = reg0.wrapping_sub(*imm);
                context.next_inst()
            }
            Inst::Mul { result, lhs, rhs } => handler::mul(context, *result, *lhs, *rhs),
            Inst::MulImm { result, src, imm } => handler::mul_imm(context, *result, *src, *imm),
            Inst::Branch { target } => handler::branch(context, *target),
            Inst::BranchEqz { target, condition } => {
                handler::branch_eqz(context, *target, *condition)
            }
            Inst::BranchEqz0 { target } => {
                if *reg0 == 0 {
                    context.branch_to(*target)
                } else {
                    context.next_inst()
                }
            }
            Inst::Return { result } => handler::ret(context, *result),
        }
    }
}

/// Executes the list of instruction using the given [`Context`].
fn execute(insts: &[Inst], context: &mut Context) {
    let mut reg0 = 0;
    loop {
        let pc = context.pc;
        // let inst = &insts[pc];
        let inst = unsafe { insts.get_unchecked(pc) };
        match inst.execute(context, &mut reg0) {
            Outcome::Continue => continue,
            Outcome::Return => return,
        }
    }
}

#[test]
fn counter_loop() {
    let repetitions = 100_000_000;
    let insts = vec![
        // Store `repetitions` into r0.
        // Note: r0 is our loop counter register.
        Inst::AddImm0 {
            imm: repetitions,
        },
        // Branch to the end if r0 is zero.
        Inst::BranchEqz0 {
            target: 4,
        },
        // Decrease r0 by 1.
        Inst::SubImm0 {
            imm: 1,
        },
        // Jump back to the loop header.
        Inst::Branch { target: 1 },
        // Return value and end function execution.
        Inst::Return { result: 0 },
    ];
    let mut context = Context::default();
    benchmark(|| execute(&insts, &mut context));
}
