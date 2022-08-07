use super::{heap::HeapAddress, value::Value};

pub enum StackEntry {
    Pointer(HeapAddress),
    Primitive(Value),
}

pub type StackAddress = u64;

pub struct StackMemory {
    pub data: Vec<StackEntry>,
}

impl StackMemory {
    pub fn new(data: Vec<StackEntry>) -> Self {
        Self { data }
    }

    pub fn put(&mut self, entry: StackEntry) -> StackAddress {
        self.data.push(entry);

        self.data.len() as StackAddress - 1
    }

    pub fn get(&self, address: StackAddress) -> Option<&StackEntry> {
        self.data.get(address as usize)
    }
}

impl Default for StackMemory {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}
