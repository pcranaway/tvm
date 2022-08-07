type Address = u64;

pub mod value;

pub trait Memory<V> {
    fn set(&mut self, address: Address, value: V) -> Result<(), &'static str>;
    fn put(&mut self, value: V) -> Result<&Address, &'static str>;

    fn get(&self, address: Address) -> Result<V, &'static str>;

}

pub mod heap;
