fn main() {
    println!("Hello, world!");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // byte 타입으로 변경

    for (i, &item) in bytes.iter().enumerate() { // iter(): 반복자, enumerate(): 튜플(인덱스, 참조) 리턴
        if item == b' ' {
            return i;
        }
    }
}