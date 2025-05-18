fn main() {
    println!("no var");
    
    let immutable = "rock"; // immutable vars
    println!("{}", immutable);
    
    
    let mut mutable = "paper";
    println!("{}", mutable);

    mutable = "scissors";
    println!("{}", mutable);

    print!("No line break,");
    print!(" it keeps going.");
    println!("");

    eprintln!("Error: stuff happened");
    
    eprint!("Error: no line breaks");
    eprint!(" it just keeps on going.");
    println!("");
   
    let has_funk: bool = true; // scalar types are ints, bools, etc
    println!("{}", has_funk);

    let is_bool = true;  // Rust has type inference at compile time, but you can also specify types
    println!("{}", is_bool);

    let card_tuple = ('A', "Spades"); // compound types are arrays, tuples, etc
    let int_array = [1, 2, 3];
    // println!("{}", card_tuple);

    let signed_int_32: i32 = -4;  // i = signed
    println!("{}", signed_int_32);

    let unsigned_int_64: u64 = 9999999; // u = unsigned
    println!("{}", unsigned_int_64);

    let small_int = 9; // rust infers the type as i32 for all integers
    println!("{}", small_int);

    let big_int: u64 = 5000000000; // you must assign types to big ints
    println!("{}", big_int);

    let small_float = -12.4;  //small floats are always signed
    println!("{}", small_float);

    let small_float: f32 = -7.9;  // up to 7 digits
    println!("{}", small_float);

    let long_float: f64 = -294.96729611;  // up to 15 digits
    println!("{}", long_float);

    let inferred_float = 3.141592653589793; // rust always infers floats as f64
    println!("{}", inferred_float);

    let amogus: char = 'ඞ';
    println!("{}", amogus);

    let inferred_char = 'B';
    println!("{}", inferred_char);

    let position_tuple: (char, i32) = ('B', 8);
    let triple_tuple: (char, char, i32) = ('A', 'B', 3);

    let inferred_tuple = ('B', 8);
    print!("{}", inferred_tuple.0);
    print!(", ");
    println!("{}", inferred_tuple.1);

    // Destructuring means assign mutliple variables to values at once
    let (column, row) = ('P', 40);
    print!("{}", column);
    print!(", ");
    println!("{}", row);

    let array: [u32; 4] = [0, 1, 2, 3]; // arrays are always fixed length, all of the same type.
    println!("{}", array[0]);

    let inferred_array = [99, 98, 87];
    println!("{}", inferred_array[0]);

    // destructure array
    let [a1, a2, a3] = inferred_array;
    println!("{}", a3);

    // if 
    let is_true = true;
    if is_true {
        println!("{}", "Yep, its true");
    }

    let temp = 25;
    if temp < 25 {
        println!("{}", "Turn on the heater");
    } else if temp == 25 {
        println!("{}", "Temp is perfect.");
    } else {
        println!("{}", "Turn on the AC.");
    }

    // assign var to an if
    let is_number = true;
    let number = if is_number {4} else {5};
    println!("{}", number);
}
