use std::sync::{Arc, Mutex};

use crate::memory::{heap::HeapMemory, stack::StackMemory};

pub struct VM {
    pub heap: Arc<Mutex<HeapMemory>>,
    pub stack: Arc<Mutex<StackMemory>>,
}

impl VM {
    pub fn new(heap: HeapMemory, stack: StackMemory) -> Self {
        Self {
            heap: Arc::new(Mutex::new(heap)),
            stack: Arc::new(Mutex::new(stack)),
        }
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new(HeapMemory::default(), StackMemory::default())
    }
}
