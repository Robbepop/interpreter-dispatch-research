#![allow(dead_code)]

#[cfg(test)]
use crate::benchmark;

use super::{
    ct::{AddInst, BranchEqzInst, BranchInst, Execute, ReturnInst, SubInst},
    rt2::{
        AddInst as DynamicAddInst, BranchEqzInst as DynamicBranchEqzInst,
        BranchInst as DynamicBranchInst, Inst as DynamicInst, ReturnInst as DynamicReturnInst,
        Source, SubInst as DynamicSubInst,
    },
    Const, Context, Outcome, Register,
};
use derive_more::From;

#[derive(Copy, Clone, From)]
pub enum Inst {
    AddRr(AddInst<Register, Register, Register>),
    AddRc(AddInst<Register, Register, Const>),
    AddCr(AddInst<Register, Const, Register>),
    AddCc(AddInst<Register, Const, Const>),

    SubRr(SubInst<Register, Register, Register>),
    SubRc(SubInst<Register, Register, Const>),
    SubCr(SubInst<Register, Const, Register>),
    SubCc(SubInst<Register, Const, Const>),

    Branch(BranchInst),

    BranchEqzR(BranchEqzInst<Register>),
    BranchEqzC(BranchEqzInst<Const>),

    ReturnR(ReturnInst<Register>),
    ReturnC(ReturnInst<Const>),
}

impl Execute for Inst {
    fn execute(self, context: &mut Context) -> Outcome {
        match self {
            Inst::AddRr(inst) => inst.execute(context),
            Inst::AddRc(inst) => inst.execute(context),
            Inst::AddCr(inst) => inst.execute(context),
            Inst::AddCc(inst) => inst.execute(context),

            Inst::SubRr(inst) => inst.execute(context),
            Inst::SubRc(inst) => inst.execute(context),
            Inst::SubCr(inst) => inst.execute(context),
            Inst::SubCc(inst) => inst.execute(context),

            Inst::Branch(inst) => inst.execute(context),

            Inst::BranchEqzR(inst) => inst.execute(context),
            Inst::BranchEqzC(inst) => inst.execute(context),

            Inst::ReturnR(inst) => inst.execute(context),
            Inst::ReturnC(inst) => inst.execute(context),
        }
    }
}

pub trait Compile {
    fn compile(self) -> Inst;
}

impl Compile for DynamicInst {
    fn compile(self) -> Inst {
        match self {
            DynamicInst::Add(inst) => inst.compile(),
            DynamicInst::Sub(inst) => inst.compile(),
            DynamicInst::Branch(inst) => inst.compile(),
            DynamicInst::BranchEqz(inst) => inst.compile(),
            DynamicInst::Return(inst) => inst.compile(),
            _ => todo!(),
        }
    }
}

impl Compile for DynamicAddInst {
    fn compile(self) -> Inst {
        match (self.lhs, self.rhs) {
            (Source::Const(src0), Source::Const(src1)) => {
                Inst::from(AddInst::new(self.result, src0, src1))
            }
            (Source::Const(src0), Source::Register(src1)) => {
                Inst::from(AddInst::new(self.result, src0, src1))
            }
            (Source::Register(src0), Source::Const(src1)) => {
                Inst::from(AddInst::new(self.result, src0, src1))
            }
            (Source::Register(src0), Source::Register(src1)) => {
                Inst::from(AddInst::new(self.result, src0, src1))
            }
        }
    }
}

impl Compile for DynamicSubInst {
    fn compile(self) -> Inst {
        match (self.lhs, self.rhs) {
            (Source::Const(src0), Source::Const(src1)) => {
                Inst::from(SubInst::new(self.result, src0, src1))
            }
            (Source::Const(src0), Source::Register(src1)) => {
                Inst::from(SubInst::new(self.result, src0, src1))
            }
            (Source::Register(src0), Source::Const(src1)) => {
                Inst::from(SubInst::new(self.result, src0, src1))
            }
            (Source::Register(src0), Source::Register(src1)) => {
                Inst::from(SubInst::new(self.result, src0, src1))
            }
        }
    }
}

impl Compile for DynamicBranchInst {
    fn compile(self) -> Inst {
        Inst::from(BranchInst::new(self.target))
    }
}

impl Compile for DynamicBranchEqzInst {
    fn compile(self) -> Inst {
        match self.condition {
            Source::Const(condition) => Inst::from(BranchEqzInst::new(self.target, condition)),
            Source::Register(condition) => Inst::from(BranchEqzInst::new(self.target, condition)),
        }
    }
}

impl Compile for DynamicReturnInst {
    fn compile(self) -> Inst {
        match self.result {
            Source::Const(result) => Inst::from(ReturnInst::new(result)),
            Source::Register(result) => Inst::from(ReturnInst::new(result)),
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
    let insts = [
        // Store `repetitions` into r0.
        // Note: r0 is our loop counter register.
        DynamicInst::add(Register(0), Register(0), Const(repetitions)),
        // Branch to the end if r0 is zero.
        DynamicInst::branch_eqz(4, Register(0)),
        // Decrease r0 by 1.
        DynamicInst::sub(Register(0), Register(0), Const(1)),
        // Jump back to the loop header.
        DynamicInst::branch(1),
        // Return value and end function execution.
        DynamicInst::ret(Register(0)),
    ]
    .map(DynamicInst::compile);
    let mut context = Context::default();
    benchmark(|| execute(&insts, &mut context));
}
