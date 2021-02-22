fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1: {}, s2: {}", s1, s2);

    let s1 = String::from("hello");
    let s2 = s1;
    // println!("s1: {}", s1); // error[E0382]: borrow of moved value: `s1`
    println!("s2: {}", s2);

    let x = 5;
    let y = x;
    println!("x: {}", x);
    println!("y: {}", y);

    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);
    
    // invalid
    let scope = "rust"; // valid
    println!("{}", scope);// valid
} // invalid
