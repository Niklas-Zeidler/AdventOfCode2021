use std::fs;
// use ndarray::prelude::*;
fn task1() {
    let amount_of_days = 80;
    // let input_text = fs::read_to_string("sample.txt")
    let input_text = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    let numbers = input_text.split(",").collect::<Vec<&str>>();
    let mut fish_vec = vec![0;numbers.len()];
    for (i,number) in numbers.iter().enumerate(){
        fish_vec[i] = i32::from_str_radix(number, 10).unwrap();
    } 
    let mut fish_vec_new = vec![0;1];
    fish_vec_new[0] = 6;
    fish_vec = fish_vec_new;
    for _day in 0..amount_of_days{
        let mut new_fish = 0;
        for fish in fish_vec.iter_mut(){
            if *fish == 0{
                *fish = 6;
                new_fish += 1;
            }
            else{
                *fish -=  1;
            }    
        }
        fish_vec.extend(vec![8;new_fish]);
        // println!("day: {},amount of fish:{}",day,fish_vec.len());
    }
    println!("task 1 : day: {},amount of fish: {}",amount_of_days,fish_vec.len());
}
fn task2(){
    let amount_of_days = 256;
    // let amount_of_days = 80;
    // let input_text = fs::read_to_string("sample.txt")
    let input_text = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    let numbers = input_text.split(",").collect::<Vec<&str>>();
    let mut fish_vec = vec![0;1];
    let mut amount_of_fish = vec![0;amount_of_days];
    // for day in 0..amount_of_days{
    //     let mut new_fish = 0;
    //     for fish in fish_vec.iter_mut(){
    //         if *fish == 0{
    //             *fish = 6;
    //             new_fish += 1;
    //         }
    //         else{
    //             *fish -=  1;
    //         }    
    //     }
    //     fish_vec.extend(vec![8;new_fish]);
    //     amount_of_fish[day] = fish_vec.len();
    //     // println!("day: {},amount of fish:{}",day,fish_vec.len());
    // }
    let mut input_vec = vec![0;numbers.len()];
    for (i,number) in numbers.iter().enumerate(){
        input_vec[i] = i32::from_str_radix(number, 10).unwrap();
    } 
    let possible_values = vec![0,1,2,3,4,5,6,7,8];
    let mut count_per_value = vec![0;9];
    for (i,value) in possible_values.iter().enumerate(){
        count_per_value[i] = input_vec.iter().filter(|&n| *n == *value).count();
    }
    for _day in 0..256{
        let mut old_counts = count_per_value.clone();
        for i in 0..8{
            if i == 0{
                count_per_value[8] = old_counts[0];
            }
            count_per_value[i] = old_counts[i+1];
            if i == 6 {
                count_per_value[i] = old_counts[0] + count_per_value[i];
            }
        }
        // println!("{:?}",count_per_value);
    }

    
    let total_sum : usize = count_per_value.iter().sum();
    println!("task 2 : day: {},amount of fish total:{}",amount_of_days,total_sum);
}
fn main(){
    task1();
    task2();
}