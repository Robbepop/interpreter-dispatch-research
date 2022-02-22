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
    /// Returns execution of the function and returns the result in `result`.
    Return { result: Register },
}

impl Inst {
    pub fn execute(&self, context: &mut Context) -> Outcome {
        match self {
            Inst::Add { result, lhs, rhs } => handler::add(context, *result, *lhs, *rhs),
            Inst::AddImm { result, src, imm } => handler::add_imm(context, *result, *src, *imm),
            Inst::Sub { result, lhs, rhs } => handler::sub(context, *result, *lhs, *rhs),
            Inst::SubImm { result, src, imm } => handler::sub_imm(context, *result, *src, *imm),
            Inst::Mul { result, lhs, rhs } => handler::mul(context, *result, *lhs, *rhs),
            Inst::MulImm { result, src, imm } => handler::mul_imm(context, *result, *src, *imm),
            Inst::Branch { target } => handler::branch(context, *target),
            Inst::BranchEqz { target, condition } => {
                handler::branch_eqz(context, *target, *condition)
            }
            Inst::Return { result } => handler::ret(context, *result),
        }
    }
}

/// Executes the list of instruction using the given [`Context`].
fn execute(insts: &[Inst], context: &mut Context) {
    loop {
        let pc = context.pc;
        let inst = &insts[pc];
        match inst.execute(context) {
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
        Inst::AddImm {
            result: 0,
            src: 0,
            imm: repetitions,
        },
        // Branch to the end if r0 is zero.
        Inst::BranchEqz {
            target: 4,
            condition: 0,
        },
        // Decrease r0 by 1.
        Inst::SubImm {
            result: 0,
            src: 0,
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

#[test]
fn more_comps() {
    let repetitions = 100_000_000;
    let insts = vec![
        // Store `repetitions` into r0.
        // Note: r0 is our loop counter register.
        Inst::AddImm {
            result: 0,
            src: 0,
            imm: repetitions,
        },
        // Store `1` into r1.
        // Note: r1 is our accumulator register.
        Inst::AddImm {
            result: 1,
            src: 1,
            imm: 1,
        },
        // Branch to the end if r0 is zero.
        Inst::BranchEqz {
            target: 7,
            condition: 0,
        },
        // Multiply r1 with r0.
        Inst::Mul {
            result: 1,
            lhs: 1,
            rhs: 0,
        },
        // Subtract r0 from r1.
        Inst::Sub {
            result: 1,
            lhs: 1,
            rhs: 0,
        },
        // Decrease r0 by 1.
        Inst::SubImm {
            result: 0,
            src: 0,
            imm: 1,
        },
        // Jump back to the loop header.
        Inst::Branch { target: 2 },
        // Return value and end function execution.
        Inst::Return { result: 1 },
    ];
    let mut context = Context::default();
    benchmark(|| execute(&insts, &mut context));
}
