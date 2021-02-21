fn main() {
    // tuple
    let tup: (i32, f64, u8) = (6, 6.22, 22); // type annotation

    println!("x의 값: {}, y의 값: {}, z의 값: {}", tup.0, tup.1, tup.2);

    let (x, y, z) = tup; // destruct

    println!("x의 값: {}, y의 값: {}, z의 값: {}", x, y, z);

    // array
    let days = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
    println!("Today is {}", days[6]);

    let array: [i32; 3] = [1, 2, 3]; // i32: data type; 3: length
    println!("{}", array.len());

    let array_one_value = [3; 4];
    println!("{}", array_one_value.len());
}
