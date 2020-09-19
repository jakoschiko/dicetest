use crate::frontend::RunCode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode {
    Debug(RunCode),
    Once,
    Repeatedly,
}
