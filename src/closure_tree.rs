#![allow(dead_code)]

#[cfg(test)]
use crate::benchmark;

use super::{handler, Bits, Context, Outcome};

#[derive(Copy, Clone)]
pub struct Register(usize);

/// A closure based expression.
pub struct Expr {
    handler: Box<dyn Fn(&mut Context) -> Bits>,
}

pub enum Input {
    Register(Register),
    Immediate(Bits),
    Expr(Expr),
}

impl From<Register> for Input {
    fn from(register: Register) -> Self {
        Self::Register(register)
    }
}

impl From<Bits> for Input {
    fn from(bits: Bits) -> Self {
        Self::Immediate(bits)
    }
}

impl From<Expr> for Input {
    fn from(expr: Expr) -> Self {
        Self::Expr(expr)
    }
}

impl Expr {
    /// Executes the given expression using the given [`Context`].
    pub fn execute(&self, context: &mut Context) -> Bits {
        (self.handler)(context)
    }

    /// Creates a new [`Inst`] from the given closure.
    fn new<T>(handler: T) -> Self
    where
        T: Fn(&mut Context) -> Bits + 'static,
    {
        Self {
            handler: Box::new(handler),
        }
    }

    pub fn add<P0, P1>(result: Register, lhs: P0, rhs: P1) -> Self
    where
        P0: Eval + 'static,
        P1: Eval + 'static,
    {
        Self::new(move |context| {
            let lhs = lhs.eval(context);
            let rhs = rhs.eval(context);
            let new_value = lhs.wrapping_add(rhs);
            context.set_reg(result.0, new_value);
            new_value
        })
    }

    pub fn sub<P0, P1>(result: Register, lhs: P0, rhs: P1) -> Self
    where
        P0: Eval + 'static,
        P1: Eval + 'static,
    {
        Self::new(move |context| {
            let lhs = lhs.eval(context);
            let rhs = rhs.eval(context);
            let new_value = lhs.wrapping_sub(rhs);
            context.set_reg(result.0, new_value);
            new_value
        })
    }

    pub fn mul<P0, P1>(result: Register, lhs: P0, rhs: P1) -> Self
    where
        P0: Eval + 'static,
        P1: Eval + 'static,
    {
        Self::new(move |context| {
            let lhs = lhs.eval(context);
            let rhs = rhs.eval(context);
            let new_value = lhs.wrapping_mul(rhs);
            context.set_reg(result.0, new_value);
            new_value
        })
    }
}

/// A closure based instruction.
pub struct Inst {
    /// The closure stores everything required for the instruction execution.
    handler: Box<dyn Fn(&mut Context) -> Outcome>,
}

pub trait Eval {
    fn eval(&self, context: &mut Context) -> Bits;
}

impl Eval for Register {
    fn eval(&self, context: &mut Context) -> Bits {
        context.get_reg(self.0)
    }
}

impl Eval for Bits {
    fn eval(&self, context: &mut Context) -> Bits {
        *self
    }
}

impl Eval for Expr {
    fn eval(&self, context: &mut Context) -> Bits {
        self.execute(context)
    }
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

    pub fn exec(expr: Expr) -> Self {
        Self::new(move |context| {
            expr.eval(context);
            Outcome::Continue
        })
    }

    pub fn local_set<I>(result: Register, input: I) -> Self
    where
        I: Eval + 'static,
    {
        Self::new(move |context| {
            let new_value = input.eval(context);
            context.set_reg(result.0, new_value);
            Outcome::Continue
        })
    }

    /// Branches to the instruction indexed by `target` if the contents of `condition` are zero.
    pub fn branch_eqz(condition: Expr) -> Self {
        Self::new(move |context| {
            let condition = condition.execute(context);
            if condition == 0 {
                Outcome::Return
            } else {
                Outcome::Continue
            }
        })
    }

    /// Executes all the instructions in the basic block one after another.
    pub fn basic_block<I>(insts: I) -> Self
    where
        I: IntoIterator<Item = Inst>,
    {
        let insts = insts.into_iter().collect::<Vec<_>>().into_boxed_slice();
        Self::new(move |context| {
            for inst in &insts[..] {
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
        Self::new(move |context| handler::ret(context, result.0))
    }
}

#[test]
fn counter_loop() {
    let repetitions = 100_000_000;
    let inst = Inst::basic_block(vec![
        Inst::exec(Expr::add(Register(0), Register(0), repetitions)),
        Inst::loop_block(Inst::branch_eqz(Expr::sub(Register(0), Register(0), 1))),
    ]);
    let mut context = Context::default();
    benchmark(|| inst.execute(&mut context));
}
