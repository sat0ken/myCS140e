// FIXME: Make me compile! Diff budget: 2 lines.
//extern crate a;
use a::MyTrait;

// Do not change this module.
mod a {
    pub trait MyTrait {
        fn foo(&self) {  }
    }

    pub struct MyType;

    impl MyTrait for MyType {  }
}

// Do not modify this function.
fn main() {
    let x = a::MyType;
    x.foo();
}
