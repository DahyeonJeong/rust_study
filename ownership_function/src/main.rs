fn main() {
    let owner = String::from("string");
    move_to(owner);
    // let test = owner; // error[E0382]: use of moved value: `owner`
}

fn move_to(s: String) {
    println!("{}", s);
}
