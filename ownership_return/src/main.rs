fn main() {
    // return tuple
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("'{}'의 길이는 {}입니다.", s2, len);

    let s1 = gives_ownership();
    println!("{}", s1);

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    println!("{}", s3);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn gives_ownership() -> String {
    
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
