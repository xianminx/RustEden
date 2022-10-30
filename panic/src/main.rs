#![allow(unused)]
fn main() {
    use std::panic;

    // like java exception handling, rust catches panic and turns it into a Err.
    let result = panic::catch_unwind(|| {
        panic!("oh no!");
    });

    // OUTPUT: Err(Any { .. })
    println!("{:?}", result);

    if let Err(err) = result {
        panic::resume_unwind(err);
    }
}