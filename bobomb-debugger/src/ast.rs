use std::fmt;
use std::collections::HashMap;
use anyhow::*;

pub trait Environment {
    fn get(&self, name: &str) -> Option<i32>;
    fn set(&mut self, name: &str, val: i32);
}

impl Environment for HashMap<String,i32> {
    fn get(&self, name: &str) -> Option<i32> {
        self.get(name).map(|i| *i)
    }

    fn set(&mut self, name: &str, val: i32) {
        self.insert(name.to_string(), val);
    }
}

#[derive(Clone, Debug)]
pub enum Cmd {
    Examine(Option<Box<Expression>>, Option<Format>),
    Print(Option<Box<Expression>>, Option<Format>),
    Break(Option<Box<Expression>>),
    Clear(i32),
    SetVar(String, Box<Expression>),
    PrintVar(Option<String>),
    Display(Option<Box<Cmd>>),
    Undisplay(i32),
    PrintStack,
    Status,
    Attach,
    Continue,
    Step,
}

impl Cmd {
    pub fn name(&self) -> &'static str {
        match *self {
            Cmd::Examine(_,_) => "examine",
            Cmd::Print(_,_) => "print",
            Cmd::Break(_) => "break",
            Cmd::Clear(_) => "clear",
            Cmd::SetVar(_,_) => "set",
            Cmd::PrintVar(_) => "print",
            Cmd::Display(_) => "display",
            Cmd::Undisplay(_) => "undisplay",
            Cmd::PrintStack => "stack",
            Cmd::Status => "status",
            Cmd::Attach => "attach",
            Cmd::Continue => "continue",
            Cmd::Step => "step",
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Display {
    Hex,
    Decimal,
    Instruction,
}

#[derive(Copy, Clone, Debug)]
pub struct Format {
    pub display: Option<Display>,
    pub count: Option<i32>,
}

#[derive(Clone, Debug)]
pub enum Expression {
    Number(i32),
    Variable(String),
    Op(Box<Expression>, BinaryOp, Box<Expression>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Expression::Variable(s) => write!(f, "{}", &s),
            Expression::Number(n) => write!(f, "{}", n),
            Expression::Op(ref l, op, ref r) => write!(f, "{} {} {}", l, op, r),
        }
    }
}

impl Expression {
    pub fn reduce<E: Environment>(&self, env: &E) -> Result<i32> {
        match &*self {
            // TODO
            Expression::Variable(v) => {
                match env.get(&v) {
                    Some(n) => Ok(n),
                    None => bail!("undefined variable {}", v),
                }
            }
            Expression::Number(n) => Ok(*n),
            Expression::Op(ref lhs, op, ref rhs) => {
                let a = lhs.reduce(env)?;
                let b = rhs.reduce(env)?;

                // TODO Wrapping operations?
                let result = match op {
                    BinaryOp::Or => a | b,
                    BinaryOp::Mul => a * b,
                    BinaryOp::Div => a / b,
                    BinaryOp::Add => a + b,
                    BinaryOp::Sub => a - b,
                    BinaryOp::And => a & b,
                    BinaryOp::Xor => a ^ b,
                    BinaryOp::LShift => a << b,
                    BinaryOp::RShift => a >> b,
                };

                Ok(result)
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum BinaryOp {
    // These are in precedent order
    Mul,
    Div,
    // Modulus
    Add,
    Sub,
    LShift,
    RShift,
    And,
    Xor,
    Or,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BinaryOp::Or => write!(f, "|"),
            BinaryOp::Mul => write!(f, "*"),
            BinaryOp::Div => write!(f, "/"),
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Sub => write!(f, "-"),
            BinaryOp::And => write!(f, "&"),
            BinaryOp::Xor => write!(f, "^"),
            BinaryOp::LShift => write!(f, "<<"),
            BinaryOp::RShift => write!(f, ">>"),
        }
    }
}
