#![allow(dead_code)]

use super::{
    ct::{AddInst, BranchEqzInst, BranchInst, Execute, ReturnInst, SubInst},
    rt::{
        AddInst as DynamicAddInst, BranchEqzInst as DynamicBranchEqzInst,
        BranchInst as DynamicBranchInst, Inst as DynamicInst, ReturnInst as DynamicReturnInst,
        Sink, Source, SubInst as DynamicSubInst,
    },
    Const, Context, Global, Outcome, Register,
};
use derive_more::From;

#[derive(Copy, Clone, From)]
pub enum Inst {
    AddRrr(AddInst<Register, Register, Register>),
    AddRrg(AddInst<Register, Register, Global>),
    AddRrc(AddInst<Register, Register, Const>),
    AddRgr(AddInst<Register, Global, Register>),
    AddRgg(AddInst<Register, Global, Global>),
    AddRgc(AddInst<Register, Global, Const>),
    AddRcr(AddInst<Register, Const, Register>),
    AddRcg(AddInst<Register, Const, Global>),
    AddRcc(AddInst<Register, Const, Const>),
    AddGrr(AddInst<Global, Register, Register>),
    AddGrg(AddInst<Global, Register, Global>),
    AddGrc(AddInst<Global, Register, Const>),
    AddGgr(AddInst<Global, Global, Register>),
    AddGgg(AddInst<Global, Global, Global>),
    AddGgc(AddInst<Global, Global, Const>),
    AddGcr(AddInst<Global, Const, Register>),
    AddGcg(AddInst<Global, Const, Global>),
    AddGcc(AddInst<Global, Const, Const>),

    SubRrr(SubInst<Register, Register, Register>),
    SubRrg(SubInst<Register, Register, Global>),
    SubRrc(SubInst<Register, Register, Const>),
    SubRgr(SubInst<Register, Global, Register>),
    SubRgg(SubInst<Register, Global, Global>),
    SubRgc(SubInst<Register, Global, Const>),
    SubRcr(SubInst<Register, Const, Register>),
    SubRcg(SubInst<Register, Const, Global>),
    SubRcc(SubInst<Register, Const, Const>),
    SubGrr(SubInst<Global, Register, Register>),
    SubGrg(SubInst<Global, Register, Global>),
    SubGrc(SubInst<Global, Register, Const>),
    SubGgr(SubInst<Global, Global, Register>),
    SubGgg(SubInst<Global, Global, Global>),
    SubGgc(SubInst<Global, Global, Const>),
    SubGcr(SubInst<Global, Const, Register>),
    SubGcg(SubInst<Global, Const, Global>),
    SubGcc(SubInst<Global, Const, Const>),

    Branch(BranchInst),

    BranchEqzR(BranchEqzInst<Register>),
    BranchEqzC(BranchEqzInst<Const>),
    BranchEqzG(BranchEqzInst<Global>),

    ReturnR(ReturnInst<Register>),
    ReturnC(ReturnInst<Const>),
    ReturnG(ReturnInst<Global>),
}

impl Execute for Inst {
    fn execute(self, context: &mut Context) -> Outcome {
        match self {
            Inst::AddRrr(inst) => inst.execute(context),
            Inst::AddRrg(inst) => inst.execute(context),
            Inst::AddRrc(inst) => inst.execute(context),
            Inst::AddRgr(inst) => inst.execute(context),
            Inst::AddRgg(inst) => inst.execute(context),
            Inst::AddRgc(inst) => inst.execute(context),
            Inst::AddRcr(inst) => inst.execute(context),
            Inst::AddRcg(inst) => inst.execute(context),
            Inst::AddRcc(inst) => inst.execute(context),
            Inst::AddGrr(inst) => inst.execute(context),
            Inst::AddGrg(inst) => inst.execute(context),
            Inst::AddGrc(inst) => inst.execute(context),
            Inst::AddGgr(inst) => inst.execute(context),
            Inst::AddGgg(inst) => inst.execute(context),
            Inst::AddGgc(inst) => inst.execute(context),
            Inst::AddGcr(inst) => inst.execute(context),
            Inst::AddGcg(inst) => inst.execute(context),
            Inst::AddGcc(inst) => inst.execute(context),

            Inst::SubRrr(inst) => inst.execute(context),
            Inst::SubRrg(inst) => inst.execute(context),
            Inst::SubRrc(inst) => inst.execute(context),
            Inst::SubRgr(inst) => inst.execute(context),
            Inst::SubRgg(inst) => inst.execute(context),
            Inst::SubRgc(inst) => inst.execute(context),
            Inst::SubRcr(inst) => inst.execute(context),
            Inst::SubRcg(inst) => inst.execute(context),
            Inst::SubRcc(inst) => inst.execute(context),
            Inst::SubGrr(inst) => inst.execute(context),
            Inst::SubGrg(inst) => inst.execute(context),
            Inst::SubGrc(inst) => inst.execute(context),
            Inst::SubGgr(inst) => inst.execute(context),
            Inst::SubGgg(inst) => inst.execute(context),
            Inst::SubGgc(inst) => inst.execute(context),
            Inst::SubGcr(inst) => inst.execute(context),
            Inst::SubGcg(inst) => inst.execute(context),
            Inst::SubGcc(inst) => inst.execute(context),

            Inst::Branch(inst) => inst.execute(context),

            Inst::BranchEqzR(inst) => inst.execute(context),
            Inst::BranchEqzC(inst) => inst.execute(context),
            Inst::BranchEqzG(inst) => inst.execute(context),

            Inst::ReturnR(inst) => inst.execute(context),
            Inst::ReturnC(inst) => inst.execute(context),
            Inst::ReturnG(inst) => inst.execute(context),
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
        }
    }
}

impl Compile for DynamicAddInst {
    fn compile(self) -> Inst {
        match (self.result, self.lhs, self.rhs) {
            (Sink::Register(sink), Source::Const(src0), Source::Const(src1)) => {
                Inst::from(AddInst::new(sink, src0, src1))
            }
            (Sink::Register(sink), Source::Const(src0), Source::Register(src1)) => {
                Inst::from(AddInst::new(sink, src0, src1))
            }
            (Sink::Register(sink), Source::Const(src0), Source::Global(src1)) => {
                Inst::from(AddInst::new(sink, src0, src1))
            }
            (Sink::Register(sink), Source::Register(src0), Source::Const(src1)) => {
                Inst::from(AddInst::new(sink, src0, src1))
            }
            (Sink::Register(sink), Source::Register(src0), Source::Register(src1)) => {
                Inst::from(AddInst::new(sink, src0, src1))
            }
            (Sink::Register(sink), Source::Register(src0), Source::Global(src1)) => {
                Inst::from(AddInst::new(sink, src0, src1))
            }
            (Sink::Register(sink), Source::Global(src0), Source::Const(src1)) => {
                Inst::from(AddInst::new(sink, src0, src1))
            }
            (Sink::Register(sink), Source::Global(src0), Source::Register(src1)) => {
                Inst::from(AddInst::new(sink, src0, src1))
            }
            (Sink::Register(sink), Source::Global(src0), Source::Global(src1)) => {
                Inst::from(AddInst::new(sink, src0, src1))
            }
            (Sink::Global(sink), Source::Const(src0), Source::Const(src1)) => {
                Inst::from(AddInst::new(sink, src0, src1))
            }
            (Sink::Global(sink), Source::Const(src0), Source::Register(src1)) => {
                Inst::from(AddInst::new(sink, src0, src1))
            }
            (Sink::Global(sink), Source::Const(src0), Source::Global(src1)) => {
                Inst::from(AddInst::new(sink, src0, src1))
            }
            (Sink::Global(sink), Source::Register(src0), Source::Const(src1)) => {
                Inst::from(AddInst::new(sink, src0, src1))
            }
            (Sink::Global(sink), Source::Register(src0), Source::Register(src1)) => {
                Inst::from(AddInst::new(sink, src0, src1))
            }
            (Sink::Global(sink), Source::Register(src0), Source::Global(src1)) => {
                Inst::from(AddInst::new(sink, src0, src1))
            }
            (Sink::Global(sink), Source::Global(src0), Source::Const(src1)) => {
                Inst::from(AddInst::new(sink, src0, src1))
            }
            (Sink::Global(sink), Source::Global(src0), Source::Register(src1)) => {
                Inst::from(AddInst::new(sink, src0, src1))
            }
            (Sink::Global(sink), Source::Global(src0), Source::Global(src1)) => {
                Inst::from(AddInst::new(sink, src0, src1))
            }
        }
    }
}

impl Compile for DynamicSubInst {
    fn compile(self) -> Inst {
        match (self.result, self.lhs, self.rhs) {
            (Sink::Register(sink), Source::Const(src0), Source::Const(src1)) => {
                Inst::from(SubInst::new(sink, src0, src1))
            }
            (Sink::Register(sink), Source::Const(src0), Source::Register(src1)) => {
                Inst::from(SubInst::new(sink, src0, src1))
            }
            (Sink::Register(sink), Source::Const(src0), Source::Global(src1)) => {
                Inst::from(SubInst::new(sink, src0, src1))
            }
            (Sink::Register(sink), Source::Register(src0), Source::Const(src1)) => {
                Inst::from(SubInst::new(sink, src0, src1))
            }
            (Sink::Register(sink), Source::Register(src0), Source::Register(src1)) => {
                Inst::from(SubInst::new(sink, src0, src1))
            }
            (Sink::Register(sink), Source::Register(src0), Source::Global(src1)) => {
                Inst::from(SubInst::new(sink, src0, src1))
            }
            (Sink::Register(sink), Source::Global(src0), Source::Const(src1)) => {
                Inst::from(SubInst::new(sink, src0, src1))
            }
            (Sink::Register(sink), Source::Global(src0), Source::Register(src1)) => {
                Inst::from(SubInst::new(sink, src0, src1))
            }
            (Sink::Register(sink), Source::Global(src0), Source::Global(src1)) => {
                Inst::from(SubInst::new(sink, src0, src1))
            }
            (Sink::Global(sink), Source::Const(src0), Source::Const(src1)) => {
                Inst::from(SubInst::new(sink, src0, src1))
            }
            (Sink::Global(sink), Source::Const(src0), Source::Register(src1)) => {
                Inst::from(SubInst::new(sink, src0, src1))
            }
            (Sink::Global(sink), Source::Const(src0), Source::Global(src1)) => {
                Inst::from(SubInst::new(sink, src0, src1))
            }
            (Sink::Global(sink), Source::Register(src0), Source::Const(src1)) => {
                Inst::from(SubInst::new(sink, src0, src1))
            }
            (Sink::Global(sink), Source::Register(src0), Source::Register(src1)) => {
                Inst::from(SubInst::new(sink, src0, src1))
            }
            (Sink::Global(sink), Source::Register(src0), Source::Global(src1)) => {
                Inst::from(SubInst::new(sink, src0, src1))
            }
            (Sink::Global(sink), Source::Global(src0), Source::Const(src1)) => {
                Inst::from(SubInst::new(sink, src0, src1))
            }
            (Sink::Global(sink), Source::Global(src0), Source::Register(src1)) => {
                Inst::from(SubInst::new(sink, src0, src1))
            }
            (Sink::Global(sink), Source::Global(src0), Source::Global(src1)) => {
                Inst::from(SubInst::new(sink, src0, src1))
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
            Source::Global(condition) => Inst::from(BranchEqzInst::new(self.target, condition)),
        }
    }
}

impl Compile for DynamicReturnInst {
    fn compile(self) -> Inst {
        match self.result {
            Source::Const(result) => Inst::from(ReturnInst::new(result)),
            Source::Register(result) => Inst::from(ReturnInst::new(result)),
            Source::Global(result) => Inst::from(ReturnInst::new(result)),
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
    execute(&insts, &mut context);
}
