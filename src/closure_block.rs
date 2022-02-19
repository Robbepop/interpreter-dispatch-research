#![allow(dead_code)]

use super::{handler, Bits, Context, Outcome, Register, Target};

/// A closure based instruction.
pub struct Inst {
    /// The closure stores everything required for the instruction execution.
    handler: Box<dyn Fn(&mut Context) -> Outcome>,
}

impl Inst {
    /// Executes the given instruction using the given [`Context`].
    pub fn execute(&self, context: &mut Context) -> Outcome {
        (self.handler)(context)
    }

    /// Creates a new [`Inst`] from the given closure.
    fn new<T>(handler: T) -> Self
    where
        T: Fn(&mut Context) -> Outcome + 'static,
    {
        Self {
            handler: Box::new(handler),
        }
    }

    /// Adds the constant `imm` and the contents of `src` and stores the result into `result`.
    pub fn add_imm<N>(result: Register, src: Register, imm: Bits, next: N) -> Self
    where
        N: Into<Option<Self>>,
    {
        match next.into() {
            Some(next) => Self::new(move |context| {
                handler::add_imm(context, result, src, imm);
                next.execute(context)
            }),
            None => Self::new(move |context| handler::add_imm(context, result, src, imm)),
        }
    }

    /// Subtracts the contents of `rhs` from `lhs` and stores the result into `result`.
    pub fn sub<N>(result: Register, lhs: Register, rhs: Register, next: N) -> Self
    where
        N: Into<Option<Self>>,
    {
        match next.into() {
            Some(next) => Self::new(move |context| {
                handler::sub(context, result, lhs, rhs);
                next.execute(context)
            }),
            None => Self::new(move |context| handler::sub(context, result, lhs, rhs)),
        }
    }

    /// Subtracts the constant `imm` from the contents of `src` and stores the result into `result`.
    pub fn sub_imm<N>(result: Register, src: Register, imm: Bits, next: N) -> Self
    where
        N: Into<Option<Self>>,
    {
        match next.into() {
            Some(next) => Self::new(move |context| {
                handler::sub_imm(context, result, src, imm);
                next.execute(context)
            }),
            None => Self::new(move |context| handler::sub_imm(context, result, src, imm)),
        }
    }

    /// Multiplies the contents of `lhs` and `rhs` and stores the result into `result`.
    pub fn mul<N>(result: Register, lhs: Register, rhs: Register, next: N) -> Self
    where
        N: Into<Option<Self>>,
    {
        match next.into() {
            Some(next) => Self::new(move |context| {
                handler::mul(context, result, lhs, rhs);
                next.execute(context)
            }),
            None => Self::new(move |context| handler::mul(context, result, lhs, rhs)),
        }
    }

    /// Branches to the instruction indexed by `target`.
    pub fn branch(target: Target) -> Self {
        Self::new(move |context| handler::branch(context, target))
    }

    /// Branches to the instruction indexed by `target` if the contents of `condition` are zero.
    pub fn branch_eqz<N>(target: Target, condition: Register, next: N) -> Self
    where
        N: Into<Option<Self>>,
    {
        match next.into() {
            Some(next) => Self::new(move |context| {
                let condition = context.get_reg(condition);
                if condition == 0 {
                    context.branch_to(target as usize)
                } else {
                    next.execute(context)
                }
            }),
            None => Self::new(move |context| handler::branch_eqz(context, target, condition)),
        }
    }

    /// Returns execution of the function and returns the result in `result`.
    pub fn ret(result: Register) -> Self {
        Self::new(move |context| handler::ret(context, result))
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
        Inst::add_imm(0, 0, repetitions, None),
        // Branch to the end if r0 is zero.
        //
        // Then: Decrease r0 by 1.
        // Then: Jump back to the loop header.
        Inst::branch_eqz(2, 0, Inst::sub_imm(0, 0, 1, Inst::branch(1))),
        // Return value and end function execution.
        Inst::ret(0),
    ];
    let mut context = Context::default();
    execute(&insts, &mut context);
}

#[test]
fn more_comps() {
    let repetitions = 100_000_000;
    let insts = vec![
        // Store `repetitions` into r0.
        // Note: r0 is our loop counter register.
        Inst::add_imm(0, 0, repetitions, None),
        // Branch to the end if r0 is zero.
        //
        // Then: Decrease r0 by 1.
        // Then: Jump back to the loop header.
        Inst::branch_eqz(
            2,
            0,
            Inst::mul(
                1,
                1,
                0,
                Inst::sub(1, 1, 0, Inst::sub_imm(0, 0, 1, Inst::branch(1))),
            ),
        ),
        // Return value and end function execution.
        Inst::ret(0),
    ];
    let mut context = Context::default();
    execute(&insts, &mut context);
}
