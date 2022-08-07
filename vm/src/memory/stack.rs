use super::{heap::HeapAddress, value::Value};

pub enum StackEntry {
    Pointer(HeapAddress),
    Primitive(Value),
}

pub struct StackMemory {
    pub data: Vec<StackEntry>,
}

impl StackMemory {
    pub fn new(data: Vec<StackEntry>) -> Self {
        Self { data }
    }
}

impl Default for StackMemory {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}
