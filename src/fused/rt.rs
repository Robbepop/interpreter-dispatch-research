#![allow(dead_code)]

use super::{Bits, Const, Context, Global, Outcome, Register, Target};

#[derive(Copy, Clone)]
pub enum Source {
    Const(Const),
    Register(Register),
    Global(Global),
}

impl From<Const> for Source {
    fn from(constant: Const) -> Self {
        Self::Const(constant)
    }
}

impl From<Register> for Source {
    fn from(register: Register) -> Self {
        Self::Register(register)
    }
}

impl From<Global> for Source {
    fn from(global: Global) -> Self {
        Self::Global(global)
    }
}

impl Source {
    pub fn load(&self, context: &Context) -> Bits {
        match self {
            Source::Const(constant) => constant.into_bits(),
            Source::Register(register) => context.get_reg(*register),
            Source::Global(global) => context.get_global(*global),
        }
    }
}

#[derive(Copy, Clone)]
pub enum Sink {
    Register(Register),
    Global(Global),
}

impl From<Register> for Sink {
    fn from(register: Register) -> Self {
        Self::Register(register)
    }
}

impl From<Global> for Sink {
    fn from(global: Global) -> Self {
        Self::Global(global)
    }
}

impl Sink {
    fn store(&self, context: &mut Context, value: Bits) {
        match self {
            Sink::Register(register) => context.set_reg(*register, value),
            Sink::Global(global) => context.set_global(*global, value),
        }
    }
}

pub trait Execute {
    fn execute(&self, context: &mut Context) -> Outcome;
}

#[derive(Copy, Clone)]
pub enum Inst {
    Add(AddInst),
    Sub(SubInst),
    Mul(MulInst),
    Eq(EqInst),
    Ne(NeInst),
    Branch(BranchInst),
    BranchEqz(BranchEqzInst),
    Return(ReturnInst),
}

impl Inst {
    pub fn add<R, P0, P1>(result: R, lhs: P0, rhs: P1) -> Self
    where
        R: Into<Sink>,
        P0: Into<Source>,
        P1: Into<Source>,
    {
        Self::Add(AddInst {
            result: result.into(),
            lhs: lhs.into(),
            rhs: rhs.into(),
        })
    }

    pub fn sub<R, P0, P1>(result: R, lhs: P0, rhs: P1) -> Self
    where
        R: Into<Sink>,
        P0: Into<Source>,
        P1: Into<Source>,
    {
        Self::Sub(SubInst {
            result: result.into(),
            lhs: lhs.into(),
            rhs: rhs.into(),
        })
    }

    pub fn mul<R, P0, P1>(result: R, lhs: P0, rhs: P1) -> Self
    where
        R: Into<Sink>,
        P0: Into<Source>,
        P1: Into<Source>,
    {
        Self::Mul(MulInst {
            result: result.into(),
            lhs: lhs.into(),
            rhs: rhs.into(),
        })
    }

    pub fn branch(target: Target) -> Self {
        Self::Branch(BranchInst { target })
    }

    pub fn branch_eqz<C>(target: Target, condition: C) -> Self
    where
        C: Into<Source>,
    {
        Self::BranchEqz(BranchEqzInst {
            target,
            condition: condition.into(),
        })
    }

    pub fn ret<R>(result: R) -> Self
    where
        R: Into<Source>,
    {
        Self::Return(ReturnInst {
            result: result.into(),
        })
    }
}

impl Execute for Inst {
    fn execute(&self, context: &mut Context) -> Outcome {
        match self {
            Inst::Add(inst) => inst.execute(context),
            Inst::Sub(inst) => inst.execute(context),
            Inst::Mul(inst) => inst.execute(context),
            Inst::Eq(inst) => inst.execute(context),
            Inst::Ne(inst) => inst.execute(context),
            Inst::Branch(inst) => inst.execute(context),
            Inst::BranchEqz(inst) => inst.execute(context),
            Inst::Return(inst) => inst.execute(context),
        }
    }
}

macro_rules! impl_cmp_insts {
    ( $( $inst_name:ident($op_name:ident) ),* $(,)? ) => {
        $(
            #[derive(Copy, Clone)]
            pub struct $inst_name {
                pub result: Sink,
                pub lhs: Source,
                pub rhs: Source,
            }

            impl Execute for $inst_name {
                fn execute(&self, context: &mut Context) -> Outcome {
                    let lhs = self.lhs.load(context);
                    let rhs = self.rhs.load(context);
                    self.result.store(context, lhs.$op_name(&rhs) as u64);
                    context.next_inst()
                }
            }
        )*
    };
}
impl_cmp_insts! {
    EqInst(eq),
    NeInst(ne),
}

#[derive(Copy, Clone)]
pub struct AddInst {
    pub result: Sink,
    pub lhs: Source,
    pub rhs: Source,
}

impl Execute for AddInst {
    fn execute(&self, context: &mut Context) -> Outcome {
        let lhs = self.lhs.load(context);
        let rhs = self.rhs.load(context);
        self.result.store(context, lhs.wrapping_add(rhs));
        context.next_inst()
    }
}

#[derive(Copy, Clone)]
pub struct SubInst {
    pub result: Sink,
    pub lhs: Source,
    pub rhs: Source,
}

impl Execute for SubInst {
    fn execute(&self, context: &mut Context) -> Outcome {
        let lhs = self.lhs.load(context);
        let rhs = self.rhs.load(context);
        self.result.store(context, lhs.wrapping_sub(rhs));
        context.next_inst()
    }
}

#[derive(Copy, Clone)]
pub struct MulInst {
    pub result: Sink,
    pub lhs: Source,
    pub rhs: Source,
}

impl Execute for MulInst {
    fn execute(&self, context: &mut Context) -> Outcome {
        let lhs = self.lhs.load(context);
        let rhs = self.rhs.load(context);
        self.result.store(context, lhs.wrapping_mul(rhs));
        context.next_inst()
    }
}

#[derive(Copy, Clone)]
pub struct BranchInst {
    pub target: Target,
}

impl Execute for BranchInst {
    fn execute(&self, context: &mut Context) -> Outcome {
        context.branch_to(self.target)
    }
}

#[derive(Copy, Clone)]
pub struct BranchEqzInst {
    pub target: Target,
    pub condition: Source,
}

impl Execute for BranchEqzInst {
    fn execute(&self, context: &mut Context) -> Outcome {
        let condition = self.condition.load(context);
        if condition == 0 {
            context.branch_to(self.target)
        } else {
            context.next_inst()
        }
    }
}

#[derive(Copy, Clone)]
pub struct ReturnInst {
    pub result: Source,
}

impl Execute for ReturnInst {
    fn execute(&self, context: &mut Context) -> Outcome {
        let result = self.result.load(context);
        context.set_reg(Register(0), result);
        Outcome::Return
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
        Inst::add(Register(0), Register(0), Const(repetitions)),
        // Branch to the end if r0 is zero.
        Inst::branch_eqz(4, Register(0)),
        // Decrease r0 by 1.
        Inst::sub(Register(0), Register(0), Const(1)),
        // Jump back to the loop header.
        Inst::branch(1),
        // Return value and end function execution.
        Inst::ret(Register(0)),
    ];
    let mut context = Context::default();
    execute(&insts, &mut context);
}
