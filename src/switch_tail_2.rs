#![allow(dead_code)]

#[cfg(test)]
use crate::benchmark;

use super::{handler, Register, Target, Context, Outcome, Bits};

#[derive(Copy, Clone)]
pub enum Inst {
    /// Adds the constant `imm` and the contents of `src` and stores the result into `result`.
    AddImm {
        result: Register,
        src: Register,
        imm: Bits,
    },
    AddImm0 {
        imm: Bits,
    },
    /// Subtracts the constant `imm` from the contents of `src` and stores the result into `result`.
    SubImm {
        result: Register,
        src: Register,
        imm: Bits,
    },
    SubImm0 {
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

pub struct ExecContext<'i, 'c> {
    insts: &'i [Inst],
    context: &'c mut Context,
}

impl<'i, 'c> ExecContext<'i, 'c> {
    pub fn tail_execute_next_2(&mut self, reg0: Bits) -> Outcome {
        let inst = unsafe { self.insts.get_unchecked(self.context.pc) };
        inst.tail_execute_2(self, reg0)
    }
}

impl Inst {
    pub fn tail_execute_2(&self, context: &mut ExecContext, reg0: Bits) -> Outcome {
        match self {
            Inst::AddImm { result, src, imm } => {
                handler::add_imm(context.context, *result, *src, *imm);
                context.tail_execute_next_2(reg0)
            }
            Inst::AddImm0 { imm } => {
                let result = reg0.wrapping_add(*imm);
                context.context.pc += 1;
                context.tail_execute_next_2(result)
            }
            Inst::SubImm { result, src, imm } => {
                handler::sub_imm(context.context, *result, *src, *imm);
                context.tail_execute_next_2(reg0)
            }
            Inst::SubImm0 { imm } => {
                let result = reg0.wrapping_sub(*imm);
                context.context.pc += 1;
                context.tail_execute_next_2(result)
            }
            Inst::Branch { target } => {
                handler::branch(context.context, *target);
                context.tail_execute_next_2(reg0)
            }
            Inst::BranchEqz { target, condition } => {
                handler::branch_eqz(context.context, *target, *condition);
                context.tail_execute_next_2(reg0)
            }
            Inst::BranchEqz0 { target } => {
                if reg0 == 0 {
                    context.context.pc = *target as usize;
                } else {
                    context.context.pc += 1;
                }
                context.tail_execute_next_2(reg0)
            }
            Inst::Return { result } => handler::ret(context.context, *result),
        }
    }
}

/// Executes the list of instruction using the given [`Context`].
fn execute(insts: &[Inst], context: &mut Context) {
    let mut exec_context = ExecContext { insts, context };
    exec_context.tail_execute_next_2(0);
}

#[test]
fn counter_loop() {
    let repetitions = 100_000_000;
    let insts = [
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
