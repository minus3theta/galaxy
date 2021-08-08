use anyhow::Context;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Expr {
    EInt(i64),
    ECons,
    ENil,
    ECar,
    ECdr,
    EAdd,
    EMul,
    EDiv,
    ENeg,
    EEq,
    ELt,
    EIsnil,
    ET,
    EF,
    EI,
    EB,
    EC,
    ES,
    EVar(i32),
    EAp(Box<Expr>, Box<Expr>),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    VInt(i64),
    VCons(Box<Value>, Box<Value>),
    VNil,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Thunk {
    TExpr(Expr),
    TValue(Value),
}

pub type Coord = (i64, i64);

pub type Picture = Vec<Coord>;

fn parse_token(token: &str) -> anyhow::Result<Expr> {
    use Expr::*;
    let expr = match token {
        "cons" => ECons,
        "nil" => ENil,
        "car" => ECar,
        "cdr" => ECdr,
        "add" => EAdd,
        "mul" => EMul,
        "div" => EDiv,
        "neg" => ENeg,
        "eq" => EEq,
        "lt" => ELt,
        "isnil" => EIsnil,
        "t" => ET,
        "f" => EF,
        "i" => EI,
        "b" => EB,
        "c" => EC,
        "s" => ES,
        var if var.starts_with(":") => EVar(var.trim_start_matches(':').parse()?),
        int if int.chars().all(|c| c == '-' || c.is_ascii_digit()) => EInt(token.parse()?),
        _ => None.context("unknown token")?,
    };
    Ok(expr)
}

pub fn parse(input: &str) -> anyhow::Result<Expr> {
    let mut stack = vec![];
    for token in input.split_ascii_whitespace().rev() {
        match token {
            "ap" => {
                let e1 = stack.pop().context("insufficient argument")?;
                let e0 = stack.pop().context("insufficient argument")?;
                stack.push(Expr::EAp(Box::new(e0), Box::new(e1)));
            }
            _ => stack.push(parse_token(token)?),
        }
    }
    stack.pop().context("empty expression")
}

pub fn evaluate(e: &Expr) -> Value {
    use Expr::*;
    use Value::*;
    match e {
        &EInt(x) => VInt(x),
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_int() {
        assert_eq!(parse("-1").unwrap(), Expr::EInt(-1));
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
}
