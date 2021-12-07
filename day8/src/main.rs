use std::fs;
fn task1(){
    let input_text = fs::read_to_string("sample.txt")
    // let input_text = fs::read_to_string("input.txt")
    .expect("Something went wrong reading the file");
    let numbers = input_text.split(",").collect::<Vec<&str>>();
    let mut input_vec = vec![0;numbers.len()];
    for (i,number) in numbers.iter().enumerate(){
        input_vec[i] = i32::from_str_radix(number, 10).unwrap();

    }
}
fn task2(){
    let input_text = fs::read_to_string("sample.txt")
    // let input_text = fs::read_to_string("input.txt")
    .expect("Something went wrong reading the file");
    let numbers = input_text.split(",").collect::<Vec<&str>>();
    let mut input_vec = vec![0;numbers.len()];
    for (i,number) in numbers.iter().enumerate(){
        input_vec[i] = i32::from_str_radix(number, 10).unwrap();

    }
}
fn main() {
    task1();
    task2();
}
