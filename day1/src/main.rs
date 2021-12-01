use std::fs;
fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    let mut c: i32 = 0;
    let mut count : i32 = 0;
    let mut current: i32 = 0;
    for line in contents.lines(){
        if !line.is_empty(){
            current = line.parse().unwrap();
            if (c != 0) && (current + a + b) > (a + b + c) {
                count = count + 1;
            }
        }
        c = b;
        b = a;
        a = current;
    }
    println!("{}",count);
}



fn maini2() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    let mut prior: i32 = 0;
    let mut current: i32 = 0;
    let mut count : i32 = -1;
    let mut k = 1;
    for line in contents.lines(){
        k = k + 1;
        
        if line.is_empty(){
            prior = 0;
        }
        else{
        current = line.parse().unwrap();
        if current > prior {
            count = count + 1;
        }
        prior = current;
        }
    }
    println!("{}",count);
}

