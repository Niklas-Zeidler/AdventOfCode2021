use std::fs;
use std::convert::TryInto;
fn task1() {
    let mut gamma: i32 = 0;
    let mut epsilon: i32 = 0;
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    let size : i32 = 12;
    let mut avg = vec![0; size.try_into().unwrap()];
    let two : i32 = 2;
    for line in contents.lines(){
        if !line.is_empty() {
            for (i,c) in line.chars().enumerate(){
                match c {
                    '1' => avg[i] += 1,
                    '0' => avg[i] -= 1,
                    _ => {}
                }
            }
        }
    }
    
    for (i,bit) in avg.iter().rev().enumerate() {
        let mut sign : i32 = 0;
        if *bit != 0 {
            sign = bit / i32::abs(*bit);
        }
        println!("i is: {} and sign is: {}",i,sign);        
        
        if sign == 1 { 
           gamma =  two.pow(i.try_into().unwrap()) + gamma;
        }
        if sign == -1 {
            epsilon = two.pow(i.try_into().unwrap()) + epsilon;
        }
        
        
    }

    
    println!("Epsilon is {}",epsilon);
    println!("Gamma is {}",gamma);
    
    println!("Result from part 1 is: {}", epsilon*gamma);
}

fn task2() {
}


fn main(){
    task1();
//    task2();
}
