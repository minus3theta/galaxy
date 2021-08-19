use std::{cell::RefCell, collections::HashMap, convert::TryInto, rc::Rc};

use anyhow::{bail, Context};

use crate::alien::send;

mod galaxy;
pub use galaxy::GalaxyProtocol;

mod statelessdraw;
pub use statelessdraw::StatelessdrawProtocol;

mod statefuldraw;
pub use statefuldraw::StatefuldrawProtocol;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Expr {
    ELit(Value),
    EFun(PrimFunc),
    EVar(String),
    EAp(Thunk, Thunk),
}

pub type Env = HashMap<String, Thunk>;

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
    pub fn call(self, args: Vec<Thunk>, env: &Env) -> anyhow::Result<Value> {
        use Expr::EAp;
        use PrimFunc::*;
        use Value::*;
        if self.arity() == args.len() {
            match self {
                PCons => {
                    let e0 = Rc::clone(&args[0]);
                    let e1 = Rc::clone(&args[1]);
                    let e2 = Rc::clone(&args[2]);
                    let expr = EAp(EAp(e2, e0).into(), e1);
                    evaluate(&expr.into(), env)
                }
                PCar => {
                    let e0 = Rc::clone(&args[0]);
                    let v: Value = PT.into();
                    let expr = EAp(e0, v.into());
                    evaluate(&expr.into(), env)
                }
                PCdr => {
                    let e0 = Rc::clone(&args[0]);
                    let v: Value = PF.into();
                    let expr = EAp(e0, v.into());
                    evaluate(&expr.into(), env)
                }
                PAdd => match (evaluate(&args[0], env)?, evaluate(&args[1], env)?) {
                    (VInt(i0), VInt(i1)) => Ok(VInt(i0 + i1)),
                    _ => bail!("add: type error"),
                },
                PMul => match (evaluate(&args[0], env)?, evaluate(&args[1], env)?) {
                    (VInt(i0), VInt(i1)) => Ok(VInt(i0 * i1)),
                    _ => bail!("mul: type error"),
                },
                PDiv => match (evaluate(&args[0], env)?, evaluate(&args[1], env)?) {
                    (VInt(i0), VInt(i1)) => Ok(VInt(i0 / i1)),
                    _ => bail!("div: type error"),
                },
                PNeg => match evaluate(&args[0], env)? {
                    VInt(i0) => Ok(VInt(-i0)),
                    _ => bail!("neg: type error"),
                },
                PEq => match (evaluate(&args[0], env)?, evaluate(&args[1], env)?) {
                    (VInt(i0), VInt(i1)) => {
                        if i0 == i1 {
                            Ok(PT.into())
                        } else {
                            Ok(PF.into())
                        }
                    }
                    _ => bail!("eq: type error"),
                },
                PLt => match (evaluate(&args[0], env)?, evaluate(&args[1], env)?) {
                    (VInt(i0), VInt(i1)) => {
                        if i0 < i1 {
                            Ok(PT.into())
                        } else {
                            Ok(PF.into())
                        }
                    }
                    _ => bail!("lt: type error"),
                },
                PIsnil => match evaluate(&args[0], env)? {
                    VNil => Ok(PT.into()),
                    _ => Ok(PF.into()),
                },
                PT => Ok(evaluate(&args[0], env)?),
                PF => Ok(evaluate(&args[1], env)?),
                PI => Ok(evaluate(&args[0], env)?),
                PB => {
                    let e0 = Rc::clone(&args[0]);
                    let e1 = Rc::clone(&args[1]);
                    let e2 = Rc::clone(&args[2]);
                    let expr = EAp(e0, EAp(e1, e2).into());
                    evaluate(&expr.into(), env)
                }
                PC => {
                    let e0 = Rc::clone(&args[0]);
                    let e1 = Rc::clone(&args[1]);
                    let e2 = Rc::clone(&args[2]);
                    let expr = EAp(EAp(e0, e2).into(), e1);
                    evaluate(&expr.into(), env)
                }
                PS => {
                    let e0 = Rc::clone(&args[0]);
                    let e1 = Rc::clone(&args[1]);
                    let e2 = Rc::clone(&args[2]);
                    let expr = EAp(EAp(e0, Rc::clone(&e2)).into(), EAp(e1, e2).into());
                    evaluate(&expr.into(), env)
                }
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

impl From<PrimFunc> for Value {
    fn from(pf: PrimFunc) -> Self {
        Value::VClosure(pf, Vec::new())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThunkEnum {
    TExpr(Expr),
    TValue(Value),
}

pub type Thunk = Rc<RefCell<ThunkEnum>>;

impl From<Expr> for Thunk {
    fn from(e: Expr) -> Self {
        Rc::new(RefCell::new(ThunkEnum::TExpr(e)))
    }
}

impl From<Value> for Thunk {
    fn from(v: Value) -> Self {
        Rc::new(RefCell::new(ThunkEnum::TValue(v)))
    }
}

pub type Coord = (i64, i64);

impl From<Coord> for Value {
    fn from(c: Coord) -> Self {
        use Value::VInt;
        let (x, y) = c;
        cons(VInt(x), VInt(y))
    }
}

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
        int if int.chars().all(|c| c == '-' || c.is_ascii_digit()) => ELit(VInt(token.parse()?)),
        var => EVar(var.to_owned()),
    };
    Ok(expr)
}

fn parse(input: &str) -> anyhow::Result<Thunk> {
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

fn evaluate(thunk: &Thunk, env: &Env) -> anyhow::Result<Value> {
    use ThunkEnum::*;
    let value = match &*thunk.borrow() {
        TExpr(e) => evaluate_expr(e, env)?,
        TValue(v) => return Ok(v.clone()),
    };
    let mut thunk_ref = thunk.borrow_mut();
    match &*thunk_ref {
        TExpr(_) => {
            *thunk_ref = TValue(value.clone());
        }
        _ => (),
    }
    Ok(value)
}

fn evaluate_expr(e: &Expr, env: &Env) -> anyhow::Result<Value> {
    use Expr::*;
    use Value::{VClosure, VCons};
    Ok(match e {
        ELit(x) => x.clone(),
        &EFun(f) => f.into(),
        EVar(var) => {
            let expr = env.get(var).context("unbound variable")?;
            evaluate(expr, env)?
        }
        EAp(e0, e1) => {
            evaluate(e0, env)?;
            match &*e0.borrow() {
                ThunkEnum::TExpr(_) => unreachable!(),
                ThunkEnum::TValue(v0) => match v0 {
                    VClosure(fun, args) => {
                        let mut args = args.clone();
                        args.push(Rc::clone(e1));
                        fun.call(args, env)?
                    }
                    VCons(car, cdr) => evaluate_expr(
                        &EAp(
                            EAp(Rc::clone(e1), (**car).clone().into()).into(),
                            (**cdr).clone().into(),
                        ),
                        env,
                    )?,
                    _ => bail!("not a function: {:?}", v0),
                },
            }
        }
    })
}

fn force_evaluate(thunk: &Thunk, env: &Env) -> anyhow::Result<Value> {
    use PrimFunc::PCons;
    use ThunkEnum::*;
    use Value::{VClosure, VCons};
    evaluate(thunk, env)?;
    Ok(match &*thunk.borrow() {
        TExpr(_) => unreachable!(),
        TValue(value) => match value {
            VClosure(fun, args) => match (fun, args.len()) {
                (PCons, 2) => {
                    let v0 = force_evaluate(&args[0], env)?;
                    let v1 = force_evaluate(&args[1], env)?;
                    VCons(Box::new(v0), Box::new(v1))
                }
                _ => bail!("incompatible closure: {:?}", fun),
            },
            _ => value.clone(),
        },
    })
}

fn parse_definition(input: &str) -> anyhow::Result<Env> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split(" = ");
            let key = iter.next().context("syntax error")?.to_owned();
            let expr = iter.next().context("syntax error")?;
            let expr = parse(expr)?;
            Ok((key, expr))
        })
        .collect()
}

pub fn cons(x: Value, y: Value) -> Value {
    Value::VCons(Box::new(x), Box::new(y))
}

fn interact(
    protocol: &Thunk,
    state: &Thunk,
    vector: Value,
    env: &Env,
) -> anyhow::Result<(Thunk, Vec<Picture>)> {
    use Expr::EAp;
    let expr = EAp(
        EAp(Rc::clone(&protocol), Rc::clone(&state)).into(),
        vector.into(),
    )
    .into();
    let ret = force_evaluate(&expr, env)?;
    let (flag, new_state, data) = ret.try_into()?;
    if flag == 0 {
        Ok((new_state, data.try_into()?))
    } else {
        interact(protocol, &new_state, send(data)?, env)
    }
}

impl std::convert::TryFrom<Value> for (i64, Thunk, Value) {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        use Value::{VCons, VInt, VNil};
        let (flag, state, data) = match value {
            VCons(flag, tail) => match (*flag, *tail) {
                (VInt(flag), VCons(state, tail)) => match *tail {
                    VCons(data, nil) if *nil == VNil => (flag, *state, *data),
                    other => bail!(
                        "type error: expected: cons(value, nil), actual: {:?}",
                        other
                    ),
                },
                other => bail!("type error: expected: (int, cons), actual: {:?}", other),
            },
            _ => bail!("type error: expected: cons, actual: {:?}", value),
        };
        Ok((flag, state.into(), data))
    }
}

impl std::convert::TryFrom<Value> for Vec<Picture> {
    type Error = anyhow::Error;

    fn try_from(mut value: Value) -> Result<Self, Self::Error> {
        use Value::{VCons, VInt};
        let mut pictures = Vec::new();
        while let VCons(p_head, p_tail) = value {
            let mut p_head = *p_head;
            let mut picture = Vec::new();
            while let VCons(d_head, d_tail) = p_head {
                if let VCons(x, y) = *d_head {
                    if let (VInt(x), VInt(y)) = (*x, *y) {
                        picture.push((x, y));
                    } else {
                        bail!("not a vector");
                    }
                } else {
                    bail!("not a list");
                }
                p_head = *d_tail;
            }

            pictures.push(picture);
            value = *p_tail;
        }
        Ok(pictures)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Protocol {
    state: Thunk,
    protocol: Thunk,
    env: Env,
}

impl Protocol {
    pub fn new<G: ProtocolGenerator>() -> anyhow::Result<Self> {
        let (protocol, env) = G::get_protocol()?;
        Ok(Self {
            state: Value::VNil.into(),
            protocol,
            env,
        })
    }

    pub fn click(&mut self, x: i64, y: i64) -> anyhow::Result<Vec<Picture>> {
        let (state, pictures) = interact(&self.protocol, &self.state, (x, y).into(), &self.env)?;
        self.state = state;
        Ok(pictures)
    }
}

pub trait ProtocolGenerator {
    const DEFINITION: &'static str;
    const PROTOCOL_NAME: &'static str;
    fn get_protocol() -> anyhow::Result<(Thunk, Env)> {
        let env = parse_definition(Self::DEFINITION)?;
        Ok((parse(Self::PROTOCOL_NAME)?, env))
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;

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
        let v = evaluate(&e, &Default::default())?;
        assert_eq!(v, Value::VInt(3));
        Ok(())
    }

    #[test]
    fn eval_cons1() -> anyhow::Result<()> {
        let input = "ap ap cons 1 2";
        let e = parse(input)?;
        let v = force_evaluate(&e, &Default::default())?;
        assert_eq!(v, cons(VInt(1), VInt(2)));
        Ok(())
    }

    #[test]
    fn eval_cons2() -> anyhow::Result<()> {
        let input = "ap ap cons 1 ap car ap ap cons 2 3";
        let e = parse(input)?;
        let v = force_evaluate(&e, &Default::default())?;
        assert_eq!(v, cons(VInt(1), VInt(2)));
        Ok(())
    }

    #[test]
    fn parse_galaxy() -> anyhow::Result<()> {
        let file = "galaxy.txt";
        let mut file = std::fs::File::open(file)?;
        let mut def = String::new();
        file.read_to_string(&mut def)?;
        let env = parse_definition(&def)?;

        let input = parse("galaxy")?;
        evaluate(&input, &env).map(|_| ())
    }

    #[test]
    fn eval_galaxy() -> anyhow::Result<()> {
        let (protocol, env) = self::galaxy::GalaxyProtocol::get_protocol()?;
        let input = EAp(
            EAp(protocol, VNil.into()).into(),
            cons(VInt(0), VInt(0)).into(),
        )
        .into();
        let (flag, _, _) = force_evaluate(&input, &env)?.try_into()?;
        assert_eq!(flag, 0);
        Ok(())
    }

    #[test]
    fn s_combinator() -> anyhow::Result<()> {
        let input = "ap ap ap s mul ap add 1 6";
        let e = parse(input)?;
        let v = evaluate(&e, &Default::default())?;
        assert_eq!(v, VInt(42));
        Ok(())
    }

    #[test]
    fn use_env() -> anyhow::Result<()> {
        let input = "ap ap mul x x";
        let e = parse(input)?;
        let env = parse_definition("x = ap ap add 1 2")?;
        let v = evaluate(&e, &env)?;
        assert_eq!(v, VInt(9));
        Ok(())
    }

    #[test]
    fn fact() -> anyhow::Result<()> {
        // fact(x) = eq(x, 0)(1, mul(x, fact(add(-1, x))))
        let input = "ap fact 5";
        let e = parse(input)?;
        let env = "fact = ap ap s ap ap c ap eq 0 1 ap ap s mul ap ap b fact ap add -1";
        let env = parse_definition(env)?;
        let v = evaluate(&e, &env)?;
        assert_eq!(v, VInt(120));
        Ok(())
    }

    #[test]
    fn to_pictures() -> anyhow::Result<()> {
        let input = cons(cons(cons(VInt(1), VInt(2)), VNil), VNil);
        let pics: Vec<Picture> = input.try_into()?;
        assert_eq!(pics, vec![vec![(1, 2)]]);
        Ok(())
    }
}
