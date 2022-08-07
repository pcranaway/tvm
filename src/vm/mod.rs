use std::sync::{Arc, Mutex};

use crate::memory::heap::HeapMemory;

pub struct VM {
    pub heap: Arc<Mutex<HeapMemory>>,
}

impl VM {
    pub fn new(heap: HeapMemory) -> Self {
        Self {
            heap: Arc::new(Mutex::new(heap)),
        }
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new(HeapMemory::default())
    }
}
