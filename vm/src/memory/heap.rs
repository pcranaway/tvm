use std::{
    collections::{hash_map::Entry::Vacant, HashMap},
    sync::Arc,
};

use super::{value::Value, Address, Memory};

pub struct HeapMemory {
    pub data: HashMap<Address, Arc<Box<Value>>>,
}

impl HeapMemory {
    pub fn new(data: HashMap<Address, Arc<Box<Value>>>) -> Self {
        Self { data }
    }
}

impl Default for HeapMemory {
    fn default() -> Self {
        Self::new(HashMap::new())
    }
}

impl Memory<Arc<Box<Value>>> for HeapMemory {
    fn set(&mut self, address: Address, value: Arc<Box<Value>>) -> Result<(), &'static str> {
        if let Vacant(e) = self.data.entry(address) {
            e.insert(value);

            Ok(())
        } else {
            Err("memory address is taken")
        }
    }

    fn put(&mut self, value: Arc<Box<Value>>) -> Result<Address, &'static str> {
        let address= if self.data.is_empty() {
            0
        } else {
            self.data.len() as Address + 1
        };

        match self.set(address, value) {
            Ok(_) => Ok(address),
            Err(_) => Err("system breakdown"),
        }
    }

    fn get(&self, address: Address) -> Result<Arc<Box<Value>>, &'static str> {
        match self.data.get(&address) {
            None => Err(""),
            Some(v) => Ok(v.clone()),
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use crate::{
        memory::{value::Value, Memory},
        vm::VM,
    };

    #[test]
    fn test_heap_memory() {
        let data = Value::Int(20);

        let vm = VM::default();

        let heap = vm.heap;
        let mut lock = heap.lock().unwrap();

        let address = match lock.put(Arc::new(Box::new(data.clone()))) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };

        let value = match lock.get(address) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };

        assert_eq!(data, **value);
    }
}