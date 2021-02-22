fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result); // The result is 20

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
    }
}
