#![allow(dead_code)]

#[cfg(test)]
use crate::benchmark;

use super::{handler, Bits, Context, Outcome, Register};

/// A closure based expression.
pub struct Expr {
    handler: Box<dyn Fn(&mut Context) -> Option<Bits>>,
}

impl Expr {
    /// Executes the given expression using the given [`Context`].
    pub fn execute(&self, context: &mut Context) -> Option<Bits> {
        (self.handler)(context)
    }

    /// Creates a new [`Inst`] from the given closure.
    fn new<T>(handler: T) -> Self
    where
        T: Fn(&mut Context) -> Option<Bits> + 'static,
    {
        Self {
            handler: Box::new(handler),
        }
    }

    /// Subtracts the constant `imm` from the contents of `src` and return the result.
    pub fn sub_imm(result: Register, src: Register, imm: Bits) -> Self {
        Self::new(move |context| {
            let lhs = context.get_reg(src);
            let rhs = imm;
            let eval = lhs.wrapping_sub(rhs);
            context.set_reg(result, eval);
            Some(eval)
        })
    }
}

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
    pub fn add_imm(result: Register, src: Register, imm: Bits) -> Self {
        Self::new(move |context| handler::add_imm(context, result, src, imm))
    }

    /// Subtracts the constant `imm` from the contents of `src` and stores the result into `result`.
    pub fn sub_imm(result: Register, src: Register, imm: Bits) -> Self {
        Self::new(move |context| handler::sub_imm(context, result, src, imm))
    }

    /// Branches to the instruction indexed by `target` if the contents of `condition` are zero.
    pub fn branch_eqz(condition: Expr) -> Self {
        Self::new(move |context| {
            let condition = condition.execute(context).unwrap();
            if condition == 0 {
                Outcome::Return
            } else {
                Outcome::Continue
            }
        })
    }

    /// Executes all the instructions in the basic block one after another.
    pub fn basic_block(insts: Vec<Inst>) -> Self {
        Self::new(move |context| {
            for inst in &insts {
                match inst.execute(context) {
                    Outcome::Continue => (),
                    Outcome::Return => return Outcome::Return,
                }
            }
            Outcome::Continue
        })
    }

    /// Loops the body until it returns.
    pub fn loop_block(body: Inst) -> Self {
        Self::new(move |context| loop {
            match body.execute(context) {
                Outcome::Continue => (),
                Outcome::Return => return Outcome::Return,
            }
        })
    }

    /// Returns execution of the function and returns the result in `result`.
    pub fn ret(result: Register) -> Self {
        Self::new(move |context| handler::ret(context, result))
    }
}

#[test]
fn counter_loop() {
    let repetitions = 100_000_000;
    let inst = Inst::basic_block(vec![
        Inst::add_imm(0, 0, repetitions),
        Inst::loop_block(Inst::branch_eqz(Expr::sub_imm(0, 0, 1))),
    ]);
    // let inst = vec![
    //     // Store `repetitions` into r0.
    //     // Note: r0 is our loop counter register.
    //     Inst::add_imm(0, 0, repetitions),
    //     // Branch to the end if r0 is zero.
    //     Inst::branch_eqz(4, 0),
    //     // Decrease r0 by 1.
    //     Inst::sub_imm(0, 0, 1),
    //     // Jump back to the loop header.
    //     Inst::branch(1),
    //     // Return value and end function execution.
    //     Inst::ret(0),
    // ];
    let mut context = Context::default();
    benchmark(|| inst.execute(&mut context));
}
