fn main() {
    println!("Hello, world!");

    let x: i32 = -42;
    let y: u64 = 100;

    println!("Signed Integer x: {}", x);
    println!("Unsigned Integer y: {}", y);

    let e: i32 = 0x7FFFFFFF;
    // let f: i32 = 0x80000000;
    println!("Hexadecimal Integer e: {}", e);

    let pi: f64 = 3.14;
    println!("Floating Point Number pi: {}", pi);

    let check: bool = true;
    println!("Boolean check: {}", check);

    let letter: char = 'A';
    println!("Character leter: {}", letter);

    let string: &str = "Hello, Rust!";
    println!("String: {}", string);
    
    let string2: String = String::from("Hello, Rust!");
    println!("String2: {}", string2);

    let string3: String = "Hello, Rust!".to_string();
    println!("String3: {}", string3);

    let tuple: (i32, f64, char) = (42, 3.14, 'A');
    println!("Tuple: ({}, {}, {})", tuple.0, tuple.1, tuple.2);

    let array: [i32; 3] = [1, 2, 3];
    println!("Array: [{}, {}, {}]", array[0], array[1], array[2]);
    println!("Array: {:?}", array);

    let numbers_slice: &[i32] = &[0, 1, 2, 3, 4];
    println!("Slice: {:?}", numbers_slice);

    let string_slice: &[&str] = &["Hello", "Rust"];
    println!("String Slice: {:?}", string_slice);

    let string_slice2: &[String] = &["Hello".to_string(), "Rust".to_string()];
    println!("String Slice2: {:?}", string_slice2);

    let vector: Vec<i32> = vec![1, 2, 3];
    println!("Vector: [{}, {}, {}]", vector[0], vector[1], vector[2]);

    let mut mutable_var: i32 = 10;
    println!("Mutable Variable: {}", mutable_var);
    mutable_var += 5;
    println!("Updated Mutable Variable: {}", mutable_var);
    
    let constant_var: i32 = 100;
    println!("Constant Variable: {}", constant_var);
    
}
