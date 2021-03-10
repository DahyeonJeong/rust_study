fn main() {
    // string slice
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{} {}", hello, world);

    let slice1 = &s[0..2];
    let slice2 = &s[..2];

    println!("slice1: {}\nslice2: {}", slice1, slice2);

    let slice1 = &s[3..s.len()];
    let slice2 = &s[3..];

    println!("slice1: {}\nslice2: {}", slice1, slice2);

    let slice1 = &s[0..s.len()];
    let slice2 = &s[..];

    println!("slice1: {}\nslice2: {}", slice1, slice2);

    // return the length of first word
    let s = String::from("apple is red");

    let s_len = len_first_word(&s);
    
    println!("{}", s_len);

    // return the first word
    let first = first_word(&s);

    println!("{}", first);

    // error[E0502]
    /*
    let mut err = String::from("B is banana");

    let word = first_word(&err);

    err.clear(); // error
    println!("the first word is: {}", word);
    */

    let s_literal = "apple is red";

    let first = first_word(&s_literal[..]);

    println!("{}", first);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    println!("{}, {}", slice[0], slice[1]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn len_first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // byte 타입으로 변경

    for (i, &item) in bytes.iter().enumerate() { // iter(): 반복자, enumerate(): 튜플(인덱스, 참조) 리턴
        if item == b' ' {
            return i;
        }
    }

    s.len()
}