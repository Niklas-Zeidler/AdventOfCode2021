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
    // check with sample input
    let contents = fs::read_to_string("sample.txt")
        .expect("Something went wrong reading the file");

    let mut array = vec![0;contents.len()];
    let size : usize = 5;
    for (i, line) in contents.lines().enumerate(){
        let temp = u32::from_str_radix(line, 2).unwrap();
        array[i] = temp as u32;
    }
    let  best_epsilon : u32 = search_for_closest_element(array, size,true);
    // let  best_gamma : u32 = search_for_closest_element(array.to_vec(), size,false);
    // println!("oxygen generator rating: {}", best_epsilon);
    // println!("co2 scrubber rating : {}", best_gamma);
    // println!("solution task2 : {}", best_gamma * best_epsilon);

    // 
    // let  best_epsilon : u32 = search_for_closest_element(array.to_vec(), epsilon, size,true);
    // let  best_gamma : u32 = search_for_closest_element(array,gamma,size,false);

    // let gamma = 784;
    // let epsilon = 3311;
    // let contents = fs::read_to_string("input.txt")
    //     .expect("Something went wrong reading the file");

    // let mut array = vec![0;contents.len()];
    // let size : usize = 12;
    // for (i, line) in contents.lines().enumerate(){
    //     let temp = u32::from_str_radix(line, 2).unwrap();
    //     array[i] = temp as u32;
    // }
    // let  best_epsilon : u32 = search_for_closest_element(array.to_vec(), epsilon, size,true);
    // let  best_gamma : u32 = search_for_closest_element(array,gamma,size,false);
    
    // println!("oxygen generator rating: {}", best_epsilon);
    // println!("co2 scrubber rating : {}", best_gamma);
    // println!("solution task2 : {}", best_gamma * best_epsilon);

    
}

fn search_for_closest_element(mut array : Vec<u32>,bit_range:usize,modifier : bool)->u32{
    let two : u32 = 2;
    let mut more_than_one_number = true;
    let mut comparing_value :u32;
    while more_than_one_number {
        for shift in (0..bit_range).rev() {
            let bit_selector =  two.pow(shift as u32);
            let mut sum = 0;

            for remaining in array.iter(){
                sum = (remaining & bit_selector) + sum;
                println!("{}",remaining);
            }
            comparing_value = 0;
            if modifier {
                if ((sum as f32) / (array.len() as f32 * bit_selector as f32)) >= 0.5 { comparing_value = bit_selector;}
            }
            else{
                if ((sum as f32) / (array.len() as f32 * bit_selector as f32)) > 0.5 { comparing_value = bit_selector;}
            }
            let old_array = array.to_vec();
            array.retain(|&x| !((x & bit_selector) > 0)^((comparing_value & bit_selector) > 0));
            // if shift < two.pow(3).try_into().unwrap()  {println!("{:?}", array);}
            // println!("shift : {}, comparing value: {}, length : {}",shift,comparing_value, array.len());
            if array.len() == 0 {
                array = old_array;
                more_than_one_number = false;
            }
            if shift == (bit_range-1){
                more_than_one_number = false;
            }
            if array.len() == 1 {
                more_than_one_number = false;
                println!("{:?}", array);
                break;
            }

        }
    }
    // let return_value : u32;
    // if array.len() != 1{
    //     if modifier {
    //         return_value = *array.iter().max().unwrap();
    //     }
    //     else{
    //         return_value = *array.iter().min().unwrap();
    //     }
    // }
    // else{
    //     return_value = array[0];
    // }
    array[0]
    // return_value
}

fn main(){
    task1();
    task2();
}
