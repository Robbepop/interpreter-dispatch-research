#![allow(dead_code)]

#[cfg(test)]
use crate::benchmark;

use super::{Bits, Context, Outcome};

#[derive(Copy, Clone)]
pub struct Global(u32);

#[derive(Copy, Clone)]
pub struct Label(usize);

#[derive(Copy, Clone)]
pub struct Register(usize);

#[derive(Copy, Clone)]
pub struct Immediate(Bits);

pub enum Expr {
    Immediate {
        immediate: Immediate,
    },
    LocalGet {
        register: Register,
    },
    LocalTee {
        register: Register,
        new_value: Box<Expr>,
    },

    AddRr {
        lhs: Register,
        rhs: Register,
    },
    AddRi {
        lhs: Register,
        rhs: Immediate,
    },
    AddRe {
        lhs: Register,
        rhs: Box<Expr>,
    },
    AddIe {
        lhs: Immediate,
        rhs: Box<Expr>,
    },
    AddEe {
        lhs_rhs: Box<[Expr; 2]>,
    },

    SubRr {
        lhs: Register,
        rhs: Register,
    },
    SubRi {
        lhs: Register,
        rhs: Immediate,
    },
    SubRe {
        lhs: Register,
        rhs: Box<Expr>,
    },
    SubIe {
        lhs: Immediate,
        rhs: Box<Expr>,
    },
    SubEe {
        lhs_rhs: Box<[Expr; 2]>,
    },

    MulRr {
        lhs: Register,
        rhs: Register,
    },
    MulRi {
        lhs: Register,
        rhs: Immediate,
    },
    MulRe {
        lhs: Register,
        rhs: Box<Expr>,
    },
    MulIe {
        lhs: Immediate,
        rhs: Box<Expr>,
    },
    MulEe {
        lhs_rhs: Box<[Expr; 2]>,
    },
}

impl Expr {
    pub fn evaluate(&self, context: &mut Context) -> Bits {
        match self {
            Expr::Immediate { immediate } => immediate.0,

            Expr::LocalGet { register } => context.get_reg(register.0),
            Expr::LocalTee {
                register,
                new_value,
            } => {
                let new_value = new_value.evaluate(context);
                context.set_reg(register.0, new_value);
                new_value
            }

            Expr::AddRr { lhs, rhs } => {
                let lhs = context.get_reg(lhs.0);
                let rhs = context.get_reg(rhs.0);
                lhs.wrapping_add(rhs)
            }
            Expr::AddRi { lhs, rhs } => {
                let lhs = context.get_reg(lhs.0);
                let rhs = rhs.0;
                lhs.wrapping_add(rhs)
            }
            Expr::AddRe { lhs, rhs } => {
                let lhs = context.get_reg(lhs.0);
                let rhs = rhs.evaluate(context);
                lhs.wrapping_add(rhs)
            }
            Expr::AddIe { lhs, rhs } => {
                let lhs = lhs.0;
                let rhs = rhs.evaluate(context);
                lhs.wrapping_add(rhs)
            }
            Expr::AddEe { lhs_rhs } => {
                let lhs = lhs_rhs[0].evaluate(context);
                let rhs = lhs_rhs[1].evaluate(context);
                lhs.wrapping_add(rhs)
            }

            Expr::SubRr { lhs, rhs } => {
                let lhs = context.get_reg(lhs.0);
                let rhs = context.get_reg(rhs.0);
                lhs.wrapping_sub(rhs)
            }
            Expr::SubRi { lhs, rhs } => {
                let lhs = context.get_reg(lhs.0);
                let rhs = rhs.0;
                lhs.wrapping_sub(rhs)
            }
            Expr::SubRe { lhs, rhs } => {
                let lhs = context.get_reg(lhs.0);
                let rhs = rhs.evaluate(context);
                lhs.wrapping_sub(rhs)
            }
            Expr::SubIe { lhs, rhs } => {
                let lhs = lhs.0;
                let rhs = rhs.evaluate(context);
                lhs.wrapping_sub(rhs)
            }
            Expr::SubEe { lhs_rhs } => {
                let lhs = lhs_rhs[0].evaluate(context);
                let rhs = lhs_rhs[1].evaluate(context);
                lhs.wrapping_sub(rhs)
            }

            Expr::MulRr { lhs, rhs } => {
                let lhs = context.get_reg(lhs.0);
                let rhs = context.get_reg(rhs.0);
                lhs.wrapping_mul(rhs)
            }
            Expr::MulRi { lhs, rhs } => {
                let lhs = context.get_reg(lhs.0);
                let rhs = rhs.0;
                lhs.wrapping_mul(rhs)
            }
            Expr::MulRe { lhs, rhs } => {
                let lhs = context.get_reg(lhs.0);
                let rhs = rhs.evaluate(context);
                lhs.wrapping_mul(rhs)
            }
            Expr::MulIe { lhs, rhs } => {
                let lhs = lhs.0;
                let rhs = rhs.evaluate(context);
                lhs.wrapping_mul(rhs)
            }
            Expr::MulEe { lhs_rhs } => {
                let lhs = lhs_rhs[0].evaluate(context);
                let rhs = lhs_rhs[1].evaluate(context);
                lhs.wrapping_mul(rhs)
            }
        }
    }
}

pub enum Inst {
    LocalSet { register: Register, expr: Expr },
    GlobalSet { global: Global, expr: Expr },
    Branch { label: Label },
    BranchIf { label: Label, condition: Expr },
    Return { result: Expr },
}

impl Inst {
    pub fn execute(&self, context: &mut Context) -> Outcome {
        match self {
            Inst::LocalSet { register, expr } => {
                let new_value = expr.evaluate(context);
                context.set_reg(register.0, new_value);
                context.next_inst()
            }
            Inst::GlobalSet { global, expr } => todo!(),
            Inst::Branch { label } => context.branch_to(label.0),
            Inst::BranchIf { label, condition } => {
                let condition = condition.evaluate(context);
                if condition == 0 {
                    context.branch_to(label.0)
                } else {
                    context.next_inst()
                }
            }
            Inst::Return { result } => {
                let new_value = result.evaluate(context);
                context.set_reg(0, new_value);
                Outcome::Return
            }
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
        Inst::LocalSet {
            register: Register(0),
            expr: Expr::Immediate {
                immediate: Immediate(repetitions),
            },
        },
        // Branch to the end if r0 is zero.
        // Decrease r0 by 1 in loop.
        Inst::BranchIf {
            label: Label(3),
            condition: Expr::LocalTee {
                register: Register(0),
                new_value: Box::new(Expr::SubRi {
                    lhs: Register(0),
                    rhs: Immediate(1),
                }),
            },
        },
        // Jump back to the loop header.
        Inst::Branch { label: Label(1) },
        // Return value and end function execution.
        Inst::Return {
            result: Expr::LocalGet {
                register: Register(0),
            },
        },
    ];
    let mut context = Context::default();
    benchmark(|| execute(&insts, &mut context));
}
