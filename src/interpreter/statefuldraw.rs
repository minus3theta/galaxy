use super::Protocol;

const STATEFULDRAW: &str = "statefuldraw = ap ap b ap b ap ap s ap ap b ap b ap cons 0 ap ap c ap ap b b cons ap ap c cons nil ap ap c cons nil ap c cons";

pub enum StatefuldrawProtocol {}

impl Protocol for StatefuldrawProtocol {
    const DEFINITION: &'static str = STATEFULDRAW;
    const PROTOCOL_NAME: &'static str = "statefuldraw";
}
