#[derive(Debug)]
struct TestStruct {
    foo: u8,
}

fn main() {
    let test_struct = TestStruct { foo: 12 };

    dbg!(test_struct);

    println!("new main who dis");

    println!("Hello, world!");
}
