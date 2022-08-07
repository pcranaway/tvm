use std::{collections::{HashMap, hash_map::Entry::Vacant}, sync::Arc};

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
        let address = self.data.len() as u64 + 1;

        match self.set(address, value) {
            Ok(_) => Ok(address),
            Err(_) => Err("system breakdown")
        }
    }


    fn get(&self, address: Address) -> Result<Arc<Box<Value>>, &'static str> {
        match self.data.get(&address) {
            None => Err(""),
            Some(v) => Ok(v.clone()),
        }
    }

}
