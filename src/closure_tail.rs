#![allow(dead_code)]

use super::{handler, Bits, Context, Outcome, Register, Target};

pub struct ExecContext<'i, 'c> {
    insts: &'i [Inst],
    context: &'c mut Context,
}

impl<'i, 'c> ExecContext<'i, 'c> {
    pub fn execute_next(&mut self) -> Outcome {
        let inst = &self.insts[self.context.pc];
        inst.execute(self)
    }
}

/// A closure based instruction.
pub struct Inst {
    /// The closure stores everything required for the instruction execution.
    handler: Box<dyn Fn(&mut ExecContext) -> Outcome>,
}

impl Inst {
    /// Executes the given instruction using the given [`Context`].
    pub fn execute(&self, context: &mut ExecContext) -> Outcome {
        (self.handler)(context)
    }

    /// Creates a new [`Inst`] from the given closure.
    fn new<T>(handler: T) -> Self
    where
        T: Fn(&mut ExecContext) -> Outcome + 'static,
    {
        Self {
            handler: Box::new(handler),
        }
    }

    /// Adds the constant `imm` and the contents of `src` and stores the result into `result`.
    pub fn add_imm(result: Register, src: Register, imm: Bits) -> Self {
        Self::new(move |context| {
            handler::add_imm(context.context, result, src, imm);
            context.execute_next()
        })
    }

    /// Subtracts the constant `imm` from the contents of `src` and stores the result into `result`.
    pub fn sub_imm(result: Register, src: Register, imm: Bits) -> Self {
        Self::new(move |context| {
            handler::sub_imm(context.context, result, src, imm);
            context.execute_next()
        })
    }

    /// Branches to the instruction indexed by `target`.
    pub fn branch(target: Target) -> Self {
        Self::new(move |context| {
            handler::branch(context.context, target);
            context.execute_next()
        })
    }

    /// Branches to the instruction indexed by `target` if the contents of `condition` are zero.
    pub fn branch_eqz(target: Target, condition: Register) -> Self {
        Self::new(move |context| {
            handler::branch_eqz(context.context, target, condition);
            context.execute_next()
        })
    }

    /// Returns execution of the function and returns the result in `result`.
    pub fn ret(result: Register) -> Self {
        Self::new(move |context| handler::ret(context.context, result))
    }
}

/// Executes the list of instruction using the given [`Context`].
fn execute(insts: &[Inst], context: &mut Context) {
    let mut context = ExecContext { insts, context };
    context.execute_next();
}

#[test]
fn counter_loop() {
    let repetitions = 100_000_000;
    let insts = vec![
        // Store `repetitions` into r0.
        // Note: r0 is our loop counter register.
        Inst::add_imm(0, 0, repetitions),
        // Branch to the end if r0 is zero.
        Inst::branch_eqz(4, 0),
        // Decrease r0 by 1.
        Inst::sub_imm(0, 0, 1),
        // Jump back to the loop header.
        Inst::branch(1),
        // Return value and end function execution.
        Inst::ret(0),
    ];
    let mut context = Context::default();
    execute(&insts, &mut context);
}
