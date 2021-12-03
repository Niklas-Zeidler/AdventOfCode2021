use std::fs;
fn task1() {
    let mut gamma: i32 = 0;
    let contents = fs::read_to_string("../input.txt")
        .expect("Something went wrong reading the file");
    let mut avg = vec![0; 12];
    let two : i32 = 2;
    let amount_of_bits : usize = 12;
    for line in contents.lines(){
        if !line.is_empty(){
            let current : i32 = line.parse().ok().unwrap();
            for k in 0..amount_of_bits {
                let power: u32 = k.try_into().unwrap();
                if (two.pow(power) & current) != 0 {
                    avg[k] += 1;
                }
                else{
                    avg[k] -= 1;
                }
            }
        }
    }
    let epsilon = (-1) * !gamma;
    println!("Result from part 1 is: {}", epsilon);
}

fn task2() {
}


fn main(){
    task1();
//    task2();
}
