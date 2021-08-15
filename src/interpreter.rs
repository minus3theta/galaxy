use std::{cell::RefCell, rc::Rc};

use anyhow::{bail, Context};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Expr {
    ELit(Value),
    EFun(PrimFunc),
    EVar(String),
    EAp(Thunk, Thunk),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    VInt(i64),
    VCons(Box<Value>, Box<Value>),
    VNil,
    VClosure(PrimFunc, Vec<Thunk>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PrimFunc {
    PCons,
    PCar,
    PCdr,
    PAdd,
    PMul,
    PDiv,
    PNeg,
    PEq,
    PLt,
    PIsnil,
    PT,
    PF,
    PI,
    PB,
    PC,
    PS,
}

impl PrimFunc {
    pub fn call(self, args: Vec<Thunk>) -> anyhow::Result<Value> {
        use PrimFunc::*;
        use Value::*;
        if self.arity() == args.len() {
            match self {
                PCons => todo!(),
                PCar => todo!(),
                PCdr => todo!(),
                PAdd => match (evaluate(&args[0])?, evaluate(&args[1])?) {
                    (VInt(i0), VInt(i1)) => Ok(VInt(i0 + i1)),
                    _ => bail!("add: type error"),
                },
                PMul => todo!(),
                PDiv => todo!(),
                PNeg => todo!(),
                PEq => todo!(),
                PLt => todo!(),
                PIsnil => todo!(),
                PT => todo!(),
                PF => todo!(),
                PI => todo!(),
                PB => todo!(),
                PC => todo!(),
                PS => todo!(),
            }
        } else {
            Ok(VClosure(self, args))
        }
    }

    fn arity(self) -> usize {
        use PrimFunc::*;
        match self {
            PCons => 3,
            PCar => 1,
            PCdr => 1,
            PAdd => 2,
            PMul => 2,
            PDiv => 2,
            PNeg => 1,
            PEq => 2,
            PLt => 2,
            PIsnil => 1,
            PT => 2,
            PF => 2,
            PI => 1,
            PB => 3,
            PC => 3,
            PS => 3,
        }
    }
}

impl Into<Value> for PrimFunc {
    fn into(self) -> Value {
        Value::VClosure(self, Vec::new())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThunkEnum {
    TExpr(Expr),
    TValue(Value),
}

pub type Thunk = Rc<RefCell<ThunkEnum>>;

impl Into<Thunk> for Expr {
    fn into(self) -> Thunk {
        Rc::new(RefCell::new(ThunkEnum::TExpr(self)))
    }
}

pub type Coord = (i64, i64);

pub type Picture = Vec<Coord>;

fn parse_token(token: &str) -> anyhow::Result<Expr> {
    use Expr::*;
    use PrimFunc::*;
    use Value::{VInt, VNil};
    let expr = match token {
        "cons" => EFun(PCons),
        "nil" => ELit(VNil),
        "car" => EFun(PCar),
        "cdr" => EFun(PCdr),
        "add" => EFun(PAdd),
        "mul" => EFun(PMul),
        "div" => EFun(PDiv),
        "neg" => EFun(PNeg),
        "eq" => EFun(PEq),
        "lt" => EFun(PLt),
        "isnil" => EFun(PIsnil),
        "t" => EFun(PT),
        "f" => EFun(PF),
        "i" => EFun(PI),
        "b" => EFun(PB),
        "c" => EFun(PC),
        "s" => EFun(PS),
        var if var.starts_with(":") => EVar(var.trim_start_matches(':').parse()?),
        int if int.chars().all(|c| c == '-' || c.is_ascii_digit()) => ELit(VInt(token.parse()?)),
        _ => bail!("unknown token: {}", token),
    };
    Ok(expr)
}

pub fn parse(input: &str) -> anyhow::Result<Thunk> {
    let mut stack = vec![];
    for token in input.split_ascii_whitespace().rev() {
        match token {
            "ap" => {
                let e0 = stack.pop().context("insufficient argument")?;
                let e1 = stack.pop().context("insufficient argument")?;
                stack.push(Expr::EAp(e0, e1).into());
            }
            _ => stack.push(parse_token(token)?.into()),
        }
    }
    stack.pop().context("empty expression")
}

pub fn evaluate(thunk: &Thunk) -> anyhow::Result<Value> {
    use ThunkEnum::*;
    let mut thunk_ref = thunk.borrow_mut();
    match &*thunk_ref {
        TExpr(e) => {
            let value = evaluate_expr(&e)?;
            *thunk_ref = TValue(value.clone());
            Ok(value)
        }
        TValue(v) => Ok(v.clone()),
    }
}

fn evaluate_expr(e: &Expr) -> anyhow::Result<Value> {
    use Expr::*;
    Ok(match e {
        ELit(x) => x.clone(),
        &EFun(f) => f.into(),
        EVar(_) => todo!(),
        EAp(e0, e1) => {
            evaluate(e0)?;
            match &*e0.borrow() {
                ThunkEnum::TExpr(_) => unreachable!(),
                ThunkEnum::TValue(v0) => match v0 {
                    Value::VClosure(fun, args) => {
                        let mut args = args.clone();
                        args.push(Rc::clone(e1));
                        fun.call(args)?
                    }
                    _ => bail!("not a function: {:?}", v0),
                },
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use Expr::*;
    use PrimFunc::*;
    use Value::*;

    #[test]
    fn parse_int() {
        assert_eq!(parse("-1").unwrap(), ELit(VInt(-1)).into());
    }

    #[test]
    fn parse_add() -> anyhow::Result<()> {
        let input = "ap ap add 1 2";
        let e = parse(input)?;
        assert_eq!(
            e,
            EAp(
                EAp(EFun(PAdd).into(), ELit(VInt(1)).into()).into(),
                ELit(VInt(2)).into()
            )
            .into()
        );
        Ok(())
    }

    #[test]
    fn parse_1029() -> anyhow::Result<()> {
        let input = "ap ap cons 7 ap ap cons 123229502148636 nil";
        parse(input).map(|_| ())
    }

    #[test]
    fn parse_1343() -> anyhow::Result<()> {
        let input = "ap ap b ap c ap ap b b ap ap b s ap ap b c ap c :1343 ap ap s ap ap b c ap ap b ap b c ap ap c ap ap b b ap ap b b isnil ap ap s ap ap b b cons ap ap c ap ap b c ap ap b ap b cons ap ap c ap ap b c ap ap b ap b :1141 ap c :1141 1 ap ap cons 0 ap ap cons nil nil i";
        parse(input).map(|_| ())
    }

    #[test]
    fn eval_add() -> anyhow::Result<()> {
        let input = "ap ap add 1 2";
        let e = parse(input)?;
        let v = evaluate(&e)?;
        assert_eq!(v, Value::VInt(3));
        Ok(())
    }
}
