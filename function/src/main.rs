fn main() {
    print_x(6);

    print_xy(6, 22);

    println!("{}", return_six());
}

fn print_x (x: i32) {
    println!("x의 값: {}", x);
}

fn print_xy (x: i32, y: i32) {
    println!("x의 값: {}", x);
    println!("y의 값: {}", y);
}

fn return_six() -> i32 {
    6
}
