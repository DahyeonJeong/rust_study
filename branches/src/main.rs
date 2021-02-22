fn main() {
    let number = 3;
    
    if number < 5 {
        println!("조건이 일치합니다.");
    } else {
        println!("조건이 일치하지 않습니다.");
    }

    let number = 1;
    
    if number < 0 {
        println!("음수입니다.");
    } else if number == 0 {
        println!("0입니다.");
    } else {
        println!("양수입니다.");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("{}", number)
}
