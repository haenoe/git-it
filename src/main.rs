#[derive(Debug)]
struct TestStruct {
    foo: u8,
}

fn main() {
    let test_struct = TestStruct { foo: 12 };

    dbg!(test_struct);

    println!("oh a new feature b");

    println!("Hello, world!");
}
