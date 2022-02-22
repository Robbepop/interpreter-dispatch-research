#![allow(dead_code)]

#[cfg(test)]
use crate::benchmark;

use super::{handler, switch::Inst, Context, Outcome};

pub struct ExecContext<'i, 'c> {
    insts: &'i [Inst],
    context: &'c mut Context,
}

impl<'i, 'c> ExecContext<'i, 'c> {
    pub fn tail_execute_next(&mut self) -> Outcome {
        let inst = &self.insts[self.context.pc];
        inst.tail_execute(self)
    }
}

impl Inst {
    pub fn tail_execute(&self, context: &mut ExecContext) -> Outcome {
        match self {
            Inst::Add { result, lhs, rhs } => {
                handler::add(context.context, *result, *lhs, *rhs);
                context.tail_execute_next()
            }
            Inst::AddImm { result, src, imm } => {
                handler::add_imm(context.context, *result, *src, *imm);
                context.tail_execute_next()
            }
            Inst::Sub { result, lhs, rhs } => {
                handler::sub(context.context, *result, *lhs, *rhs);
                context.tail_execute_next()
            }
            Inst::SubImm { result, src, imm } => {
                handler::sub_imm(context.context, *result, *src, *imm);
                context.tail_execute_next()
            }
            Inst::Mul { result, lhs, rhs } => {
                handler::mul(context.context, *result, *lhs, *rhs);
                context.tail_execute_next()
            }
            Inst::MulImm { result, src, imm } => {
                handler::mul_imm(context.context, *result, *src, *imm);
                context.tail_execute_next()
            }
            Inst::Branch { target } => {
                handler::branch(context.context, *target);
                context.tail_execute_next()
            }
            Inst::BranchEqz { target, condition } => {
                handler::branch_eqz(context.context, *target, *condition);
                context.tail_execute_next()
            }
            Inst::Return { result } => handler::ret(context.context, *result),
        }
    }
}

/// Executes the list of instruction using the given [`Context`].
fn execute(insts: &[Inst], context: &mut Context) {
    let mut exec_context = ExecContext { insts, context };
    exec_context.tail_execute_next();
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
