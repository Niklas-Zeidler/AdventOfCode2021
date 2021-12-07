use std::fs;
fn task1() {
    // let input_text = fs::read_to_string("sample.txt")
    let input_text = fs::read_to_string("input.txt")
    .expect("Something went wrong reading the file");
    let numbers = input_text.split(",").collect::<Vec<&str>>();
    let mut input_vec = vec![0;numbers.len()];
    for (i,number) in numbers.iter().enumerate(){
        input_vec[i] = i32::from_str_radix(number, 10).unwrap();
    }
    let mut is_falling = true;
    let mut position :i32 = 2;
    let mut fuel_cost = 1000*input_vec.len() as i32;
    let mut fuel_cost_old : i32;
    let mut fuel_cost_per_steps : std::vec::Vec<usize>= vec![0;30000];
    fuel_cost_per_steps[1] = 1;
    while is_falling {
        fuel_cost_old = fuel_cost;
        fuel_cost = 0;
        for element in input_vec.iter(){
            let fuel_cost_element = element - position;
            fuel_cost += fuel_cost_element.abs();
        }
        if fuel_cost_old < fuel_cost{
            position -= 1;
            fuel_cost = fuel_cost_old;
            is_falling = false;
        }
        else{position += 1;}
    }
    println!("task 1: position is : {} fuel cost being: {}",position,fuel_cost);
}
fn task2() {
    // let input_text = fs::read_to_string("sample.txt")
    let input_text = fs::read_to_string("input.txt")
    .expect("Something went wrong reading the file");
    let numbers = input_text.split(",").collect::<Vec<&str>>();
    let mut input_vec = vec![0;numbers.len()];
    for (i,number) in numbers.iter().enumerate(){
        input_vec[i] = i32::from_str_radix(number, 10).unwrap();

    }
    let mut fuel_cost_per_steps : std::vec::Vec<usize>= vec![0;30000];
    fuel_cost_per_steps[1] = 1;
    let max_value = input_vec.iter().max().unwrap().abs() as usize;
    println!("max value : {}",max_value);
    for steps in 2..(max_value as usize + 1) {
        let vec: Vec<usize> = (0..steps+1).collect();
        fuel_cost_per_steps[steps] = vec.iter().sum();
    }
    let mut fuel_cost_min = 1000000000;
    let mut position_min : usize = 0;
    let mut fuel_cost : usize;
    for position in 0..max_value as usize {
        fuel_cost = 0;
        for element in input_vec.iter(){
            let steps = element - position as i32;
            fuel_cost += fuel_cost_per_steps[steps.abs() as usize];
        }
        if fuel_cost_min > fuel_cost{
            fuel_cost_min = fuel_cost;
            position_min = position as usize;
        }
    }
    println!("task 2: position is : {} fuel cost being: {}",position_min,fuel_cost_min);
}
fn main(){
    task1();
    task2();
}