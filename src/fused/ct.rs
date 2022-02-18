#![allow(dead_code)]

use super::{Bits, Const, Context, Global, Outcome, Register, Target};

// ===

pub trait Store {
    fn store(&self, context: &mut Context, value: Bits);
}

impl Store for Register {
    fn store(&self, context: &mut Context, value: Bits) {
        context.regs[self.0] = value;
    }
}

impl Store for Global {
    fn store(&self, context: &mut Context, value: Bits) {
        context.globals[self.0] = value;
    }
}

// ===

pub trait Load {
    fn load(&self, context: &Context) -> Bits;
}

impl Load for Register {
    fn load(&self, context: &Context) -> Bits {
        context.regs[self.0]
    }
}

impl Load for Global {
    fn load(&self, context: &Context) -> Bits {
        context.globals[self.0]
    }
}

impl Load for Const {
    fn load(&self, _: &Context) -> Bits {
        self.0
    }
}

// ===

#[derive(Copy, Clone)]
pub struct RawSink {
    index: usize,
}

impl From<Global> for RawSink {
    fn from(global: Global) -> Self {
        Self {
            index: global.into_usize(),
        }
    }
}

impl From<Register> for RawSink {
    fn from(register: Register) -> Self {
        Self {
            index: register.into_usize(),
        }
    }
}

// ===

#[derive(Copy, Clone)]
pub struct RawSource {
    index: u64,
}

impl From<Global> for RawSource {
    fn from(global: Global) -> Self {
        Self {
            index: global.into_usize() as u64,
        }
    }
}

impl From<Register> for RawSource {
    fn from(register: Register) -> Self {
        Self {
            index: register.into_usize() as u64,
        }
    }
}

impl From<Const> for RawSource {
    fn from(constant: Const) -> Self {
        Self {
            index: constant.into_bits(),
        }
    }
}

// ===

impl From<RawSink> for Global {
    fn from(sink: RawSink) -> Self {
        Self(sink.index)
    }
}

impl From<RawSink> for Register {
    fn from(sink: RawSink) -> Self {
        Self(sink.index)
    }
}

impl From<RawSource> for Global {
    fn from(source: RawSource) -> Self {
        Self(source.index as usize)
    }
}

impl From<RawSource> for Register {
    fn from(source: RawSource) -> Self {
        Self(source.index as usize)
    }
}

impl From<RawSource> for Const {
    fn from(source: RawSource) -> Self {
        Self(source.index)
    }
}

// ===

#[derive(Copy, Clone)]
pub struct InstData {
    pub sink: RawSink,
    pub src0: RawSource,
    pub src1: RawSource,
}

impl<Sink, Src0, Src1> From<(Sink, Src0, Src1)> for InstData
where
    Sink: Into<RawSink>,
    Src0: Into<RawSource>,
    Src1: Into<RawSource>,
{
    fn from((sink, src0, src1): (Sink, Src0, Src1)) -> Self {
        Self {
            sink: sink.into(),
            src0: src0.into(),
            src1: src1.into(),
        }
    }
}

impl InstData {
    pub fn into_raw_parts<Sink, Src0, Src1>(self) -> (Sink, Src0, Src1)
    where
        Sink: From<RawSink>,
        Src0: From<RawSource>,
        Src1: From<RawSource>,
    {
        (
            Sink::from(self.sink),
            Src0::from(self.src0),
            Src1::from(self.src1),
        )
    }
}

// ===

#[derive(Copy, Clone)]
pub struct Inst {
    handler: fn(&mut Context, InstData) -> Outcome,
    data: InstData,
}

pub trait Result: Store + Into<RawSink> + From<RawSink> {}
impl<T> Result for T where T: Store + Into<RawSink> + From<RawSink> {}

pub trait Param: Load + Into<RawSource> + From<RawSource> {}
impl<T> Param for T where T: Load + Into<RawSource> + From<RawSource> {}

impl Inst {
    pub fn execute(&self, context: &mut Context) -> Outcome {
        (self.handler)(context, self.data)
    }

    pub fn add<R, P0, P1>(result: R, lhs: P0, rhs: P1) -> Self
    where
        R: Result,
        P0: Param,
        P1: Param,
    {
        let inst = AddInst { result, lhs, rhs };
        Self {
            handler: move |context, data| {
                <AddInst<R, P0, P1> as FromData>::from_data(data).execute(context)
            },
            data: IntoData::into_data(inst),
        }
    }

    pub fn sub<R, P0, P1>(result: R, lhs: P0, rhs: P1) -> Self
    where
        R: Result,
        P0: Param,
        P1: Param,
    {
        let inst = SubInst { result, lhs, rhs };
        Self {
            handler: move |context, data| {
                <SubInst<R, P0, P1> as FromData>::from_data(data).execute(context)
            },
            data: IntoData::into_data(inst),
        }
    }

    pub fn branch(target: Target) -> Self {
        let inst = BranchInst { target };
        Self {
            handler: move |context, data| {
                <BranchInst as FromData>::from_data(data).execute(context)
            },
            data: IntoData::into_data(inst),
        }
    }

    pub fn branch_eqz<C>(target: Target, condition: C) -> Self
    where
        C: Param,
    {
        let inst = BranchEqzInst { target, condition };
        Self {
            handler: move |context, data| {
                <BranchEqzInst<C> as FromData>::from_data(data).execute(context)
            },
            data: IntoData::into_data(inst),
        }
    }

    pub fn ret<R>(result: R) -> Self
    where
        R: Param,
    {
        let inst = ReturnInst { result };
        Self {
            handler: move |context, data| {
                <ReturnInst<R> as FromData>::from_data(data).execute(context)
            },
            data: IntoData::into_data(inst),
        }
    }
}

// ===

pub trait IntoData {
    fn into_data(self) -> InstData;
}

pub trait FromData {
    fn from_data(data: InstData) -> Self;
}

pub trait Execute {
    fn execute(self, context: &mut Context) -> Outcome;
}

// ===

#[derive(Copy, Clone)]
pub struct AddInst<R, P0, P1> {
    result: R,
    lhs: P0,
    rhs: P1,
}

impl<R, P0, P1> AddInst<R, P0, P1> {
    pub fn new(result: R, lhs: P0, rhs: P1) -> Self {
        Self { result, lhs, rhs }
    }
}

impl<R, P0, P1> IntoData for AddInst<R, P0, P1>
where
    R: Into<RawSink>,
    P0: Into<RawSource>,
    P1: Into<RawSource>,
{
    fn into_data(self) -> InstData {
        InstData::from((self.result, self.lhs, self.rhs))
    }
}

impl<R, P0, P1> FromData for AddInst<R, P0, P1>
where
    R: From<RawSink>,
    P0: From<RawSource>,
    P1: From<RawSource>,
{
    fn from_data(data: InstData) -> Self {
        let (result, lhs, rhs) = data.into_raw_parts();
        Self { result, lhs, rhs }
    }
}

impl<R, P0, P1> Execute for AddInst<R, P0, P1>
where
    R: Store,
    P0: Load,
    P1: Load,
{
    fn execute(self, context: &mut Context) -> Outcome {
        let lhs = self.lhs.load(context);
        let rhs = self.rhs.load(context);
        self.result.store(context, lhs.wrapping_add(rhs));
        context.next_inst()
    }
}

// ===

#[derive(Copy, Clone)]
pub struct SubInst<R, P0, P1> {
    result: R,
    lhs: P0,
    rhs: P1,
}

impl<R, P0, P1> SubInst<R, P0, P1> {
    pub fn new(result: R, lhs: P0, rhs: P1) -> Self {
        Self { result, lhs, rhs }
    }
}

impl<R, P0, P1> IntoData for SubInst<R, P0, P1>
where
    R: Into<RawSink>,
    P0: Into<RawSource>,
    P1: Into<RawSource>,
{
    fn into_data(self) -> InstData {
        InstData::from((self.result, self.lhs, self.rhs))
    }
}

impl<R, P0, P1> FromData for SubInst<R, P0, P1>
where
    R: From<RawSink>,
    P0: From<RawSource>,
    P1: From<RawSource>,
{
    fn from_data(data: InstData) -> Self {
        let (result, lhs, rhs) = data.into_raw_parts();
        Self { result, lhs, rhs }
    }
}

impl<R, P0, P1> Execute for SubInst<R, P0, P1>
where
    R: Store,
    P0: Load,
    P1: Load,
{
    fn execute(self, context: &mut Context) -> Outcome {
        let lhs = self.lhs.load(context);
        let rhs = self.rhs.load(context);
        self.result.store(context, lhs.wrapping_sub(rhs));
        context.next_inst()
    }
}

// ===

#[derive(Copy, Clone)]
pub struct BranchInst {
    target: Target,
}

impl BranchInst {
    pub fn new(target: Target) -> Self {
        Self { target }
    }
}

impl IntoData for BranchInst {
    fn into_data(self) -> InstData {
        InstData {
            sink: RawSink { index: self.target },
            src0: RawSource { index: 0 },
            src1: RawSource { index: 0 },
        }
    }
}

impl FromData for BranchInst {
    fn from_data(data: InstData) -> Self {
        let target = data.sink.index;
        Self { target }
    }
}

impl Execute for BranchInst {
    fn execute(self, context: &mut Context) -> Outcome {
        context.branch_to(self.target)
    }
}

// ===

#[derive(Copy, Clone)]
pub struct BranchEqzInst<C> {
    target: Target,
    condition: C,
}

impl<C> BranchEqzInst<C> {
    pub fn new(target: Target, condition: C) -> Self {
        Self { target, condition }
    }
}

impl<C> IntoData for BranchEqzInst<C>
where
    C: Into<RawSource>,
{
    fn into_data(self) -> InstData {
        InstData {
            sink: RawSink { index: self.target },
            src0: self.condition.into(),
            src1: RawSource { index: 0 },
        }
    }
}

impl<C> FromData for BranchEqzInst<C>
where
    C: From<RawSource>,
{
    fn from_data(data: InstData) -> Self {
        let target = data.sink.index;
        let condition = C::from(data.src0);
        Self { target, condition }
    }
}

impl<C> Execute for BranchEqzInst<C>
where
    C: Load,
{
    fn execute(self, context: &mut Context) -> Outcome {
        let condition = self.condition.load(context);
        if condition == 0 {
            context.branch_to(self.target)
        } else {
            context.next_inst()
        }
    }
}

// ===

#[derive(Copy, Clone)]
pub struct ReturnInst<R> {
    result: R,
}

impl<C> ReturnInst<C> {
    pub fn new(result: C) -> Self {
        Self { result }
    }
}

impl<R> IntoData for ReturnInst<R>
where
    R: Into<RawSource>,
{
    fn into_data(self) -> InstData {
        InstData {
            sink: RawSink { index: 0 },
            src0: self.result.into(),
            src1: RawSource { index: 0 },
        }
    }
}

impl<R> FromData for ReturnInst<R>
where
    R: From<RawSource>,
{
    fn from_data(data: InstData) -> Self {
        let result = R::from(data.src0);
        Self { result }
    }
}

impl<R> Execute for ReturnInst<R>
where
    R: Load,
{
    fn execute(self, context: &mut Context) -> Outcome {
        let result = self.result.load(context);
        context.set_reg(Register(0), result);
        Outcome::Return
    }
}

// ===

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

// ===

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
