#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    String(String),
    Int(i32),
    Nil(),
}
