fn main() {
    let mut attempts = 0;

    print!("Starting");
    loop {
        print!(".");
        attempts += 1;
        if attempts >= 3 {
            println!("");
            break;
        }
    }

    let mut bananas = 4;
    print!("eating bananas");
    while bananas > 0 {
        print!(".");
        bananas -= 1;
    }
    println!("");

    print!("eating rice");
    for rice in 0..5 { // exclusing ranges, does not include the end value
        print!(".");
    }
    println!("");

    print!("counting: ");
    for count in 0..=4 { // inclusive ranges include the end value
        print!("{}", count);
        print!(", " );
    }
    println!("");
}
