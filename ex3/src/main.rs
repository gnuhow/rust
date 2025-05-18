fn main() {
    print_words();
    let q: i32 = multiply(2,3);
    println!("{}", q);

    let r: i32 = add(2,3);
    println!("{}", r);

    borrow_complex_object() 
    borrow_mutable_complex_object()
}

fn print_words() {
    println!("zebra abracadabra");
}

fn multiply(a: i32, b:i32) -> i32 {
    a * b // last expression is returned automagically, if you don't have a ;
}

fn add(a: i32, b:i32) -> i32 {
    return a + b;
}

// When you copy complex objects, the original is lost
fn bad_copy_complex_object() {
    let text1 = String::from("example");
    // let text2 = text1;

    println!("{}", text1);
}

// use & to safely borrow a complex object
fn borrow_complex_object() {
    let text1 = String::from("example");
    let text2 = &text1;

    println!("{}", text1);
    println!("{}", text2);
}

fn borrow_mutable_complex_object() {
    let mut text1 = String::from("example");
    let mut text2 = &mut text1;

    println!("{text1}, {text2}");
}
