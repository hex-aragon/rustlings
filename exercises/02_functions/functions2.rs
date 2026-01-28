// TODO: Add the missing type of the argument `num` after the colon `:`.
fn call_me(num:u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }

    for j in 0..num {
        println!("Another ring! Call number {}", j+1);
    }
}

fn main() {
    call_me(3);
}
