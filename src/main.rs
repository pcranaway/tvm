use vm::VM;

pub mod memory;
pub mod vm;

fn main() {
    let vm = VM::default();

    {
        let heap = vm.heap.clone();
        let lock = heap.lock().unwrap();
    }
}
