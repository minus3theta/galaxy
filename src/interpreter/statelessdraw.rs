use super::ProtocolGenerator;

const STATELESSDRAW: &str = r"statelessdraw = ap ap c ap ap b b ap ap b ap b ap cons 0 ap ap c ap ap b b cons ap ap c cons nil ap ap c ap ap b cons ap ap c cons nil nil";

pub enum StatelessdrawProtocol {}

impl ProtocolGenerator for StatelessdrawProtocol {
    const DEFINITION: &'static str = STATELESSDRAW;
    const PROTOCOL_NAME: &'static str = "statelessdraw";
}
