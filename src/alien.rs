use std::iter::{repeat, Peekable};

use anyhow::{bail, Context};
use reqwest::Client;

use crate::{
    interpreter::{cons, Value},
    log,
};

pub async fn send(data: Value) -> anyhow::Result<Value> {
    let client = Client::new();
    let body = modulate(&data)?;
    log(&format!("Sent:     {}", &body));
    let res = client
        .post("https://api.pegovka.space/aliens/send")
        .body(body)
        .send()
        .await?;
    let res = res.text().await?;
    log(&format!("Received: {}", &res));
    let res = demodulate(&res)?;
    Ok(res)
}

fn modulate(data: &Value) -> anyhow::Result<String> {
    let mut buf = String::new();
    modulate_internal(data, &mut buf)?;
    Ok(buf)
}

fn modulate_internal(data: &Value, buf: &mut String) -> anyhow::Result<()> {
    use Value::*;
    match data {
        &VInt(i) => {
            buf.push_str(if i >= 0 { "01" } else { "10" });
            modulate_nat(i.abs(), buf);
        }
        VCons(car, cdr) => {
            buf.push_str("11");
            modulate_internal(car.as_ref(), buf)?;
            modulate_internal(cdr.as_ref(), buf)?;
        }
        VNil => buf.push_str("00"),
        VClosure(_, _) => bail!("closure is not modulatable"),
    }
    Ok(())
}

fn modulate_nat(nat: i64, buf: &mut String) {
    let len = 64 - nat.leading_zeros() as usize;
    let reserved_len = (len + 4 - 1) / 4;
    buf.extend(repeat('1').take(reserved_len));
    buf.push('0');
    buf.extend(repeat('0').take(reserved_len * 4 - len));
    if nat > 0 {
        buf.push_str(&format!("{:b}", nat));
    }
}

fn demodulate(data: &str) -> anyhow::Result<Value> {
    use Token::*;
    use Value::*;
    let tokens = tokenize(data)?;
    let mut stack = Vec::new();
    for token in tokens.into_iter().rev() {
        match token {
            TkNil => stack.push(VNil),
            TkInt(i) => stack.push(VInt(i)),
            TkCons => {
                let car = stack.pop().context("car does not exist")?;
                let cdr = stack.pop().context("cdr does not exist")?;
                stack.push(cons(car, cdr));
            }
        }
    }
    stack.pop().context("empty data")
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Token {
    TkNil,
    TkInt(i64),
    TkCons,
}

fn tokenize(data: &str) -> anyhow::Result<Vec<Token>> {
    use Token::*;
    let mut tokens = Vec::new();
    let mut iter = data.chars().peekable();
    while iter.peek().is_some() {
        let token = match (iter.next(), iter.next()) {
            (Some('0'), Some('0')) => TkNil,
            (Some('0'), Some('1')) => TkInt(demod_nat(&mut iter)?),
            (Some('1'), Some('0')) => TkInt(-demod_nat(&mut iter)?),
            (Some('1'), Some('1')) => TkCons,
            _ => bail!("syntax error"),
        };
        tokens.push(token);
    }
    Ok(tokens)
}

fn demod_nat(iter: &mut Peekable<impl Iterator<Item = char>>) -> anyhow::Result<i64> {
    let mut reserved_len = 0;
    while let Some('1') = iter.next() {
        reserved_len += 1;
    }
    if reserved_len == 0 {
        return Ok(0);
    }
    let bin: String = iter.take(reserved_len * 4).collect();
    Ok(i64::from_str_radix(&bin, 2)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mod_nat_0() {
        let mut buf = String::new();
        modulate_nat(0, &mut buf);
        assert_eq!(buf, "0");
    }

    #[test]
    fn mod_nat_16() {
        let mut buf = String::new();
        modulate_nat(16, &mut buf);
        assert_eq!(buf, "11000010000");
    }

    #[test]
    fn tokenize_16() -> anyhow::Result<()> {
        assert_eq!(tokenize("0111000010000")?, vec![Token::TkInt(16)]);
        Ok(())
    }

    #[test]
    fn tokenize_0() -> anyhow::Result<()> {
        assert_eq!(tokenize("010")?, vec![Token::TkInt(0)]);
        Ok(())
    }

    #[test]
    fn demod_cons_0_nil() -> anyhow::Result<()> {
        use Value::*;
        assert_eq!(demodulate("1101000")?, cons(VInt(0), VNil));
        Ok(())
    }
}
