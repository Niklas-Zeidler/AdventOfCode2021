use std::fs;
use ndarray::prelude::*;
fn task1() {
    let rng_text = fs::read_to_string("rng.txt")
    // let rng_text = fs::read_to_string("sample_rng.txt")
        .expect("Something went wrong reading the file");
    let split_contents = rng_text.split(",").collect::<Vec<&str>>();
    let mut rng_numbers : Vec<i32> = vec![0;split_contents.len()];
    for (i,split) in split_contents.iter().enumerate(){
        let temp = i32::from_str_radix(split, 10).unwrap();
        rng_numbers[i] = temp;
    }
    let bingo_text = fs::read_to_string("bingo.txt")
    // let bingo_text = fs::read_to_string("sample_bingo.txt")
        .expect("Something went wrong reading the file");
    let mut turns: i32 = 1000;
    let mut score: i32 = 0;
    let mut turns_temp : i32;
    let mut score_temp: i32;
    let mut current_board = Array2::<i32>::zeros((5,5));
    
    let mut row_in_board = 0;
    for line in bingo_text.lines(){
        if (line.is_empty() )|(row_in_board == 5) {
            let result = evaluate_board(&current_board,rng_numbers.to_vec());
            turns_temp = result.1;
            score_temp = result.0;
            if turns_temp < turns {
                turns = turns_temp;
                score = score_temp;
            }
            current_board.fill(0 as i32);
            row_in_board = 0;
        }
        else{
            //  do the parsing of the board values here
            for (i,split_line) in line.split_ascii_whitespace().collect::<Vec<&str>>().iter().enumerate(){
                current_board[[row_in_board,i]]=i32::from_str_radix(split_line, 10).unwrap();
            }
            row_in_board += 1;
        }

    }
    println!("task 1 the score is {}",score);
    println!("task 1 turns are : {}",turns);
}
fn evaluate_board(board : &ndarray::ArrayBase<ndarray::OwnedRepr<i32>, ndarray::Dim<[usize; 2]>>
    ,rng_numbers : Vec<i32>) -> (i32, i32){
    let mut logical_board =  Array2::<i32>::zeros((5,5));
    let mut turns: i32 = 0;
    let mut score: i32 = 0;
    for (i,number) in rng_numbers.iter().enumerate(){
        let logical_temp =board.mapv(|a| a == *number);
        logical_board = logical_board +  logical_temp.mapv(|elem| elem as i32);
        if logical_board.sum_axis(Axis(1)).iter().any(|&x| x == 5){
            turns = i as i32;
            let invert_logical_board = (logical_board.clone() - 1 ) * (-1);
            let temp = board * invert_logical_board;
            score =  temp.sum() * *number;
            break;
        }
        if logical_board.sum_axis(Axis(0)).iter().any(|&x| x == 5){
            turns = i as i32;
            let invert_logical_board = (logical_board.clone() - 1 ) * (-1);
            let temp = board * invert_logical_board;
            score = temp.sum() * *number;
            break;
        }
        

    }
    // println!("{}",logical_board);
    return (score,turns);
}
fn task2() {
    let rng_text = fs::read_to_string("rng.txt")
    // let rng_text = fs::read_to_string("sample_rng.txt")
        .expect("Something went wrong reading the file");
    let split_contents = rng_text.split(",").collect::<Vec<&str>>();
    let mut rng_numbers : Vec<i32> = vec![0;split_contents.len()];
    for (i,split) in split_contents.iter().enumerate(){
        let temp = i32::from_str_radix(split, 10).unwrap();
        rng_numbers[i] = temp;
    }
    let bingo_text = fs::read_to_string("bingo.txt")
    // let bingo_text = fs::read_to_string("sample_bingo.txt")
        .expect("Something went wrong reading the file");
    let mut turns: i32 = 0;
    let mut score: i32 = 0;
    let mut turns_temp : i32;
    let mut score_temp: i32;
    let mut current_board = Array2::<i32>::zeros((5,5));
    
    let mut row_in_board = 0;
    for line in bingo_text.lines(){
        if (line.is_empty() )|(row_in_board == 5) {
            let result = evaluate_board(&current_board,rng_numbers.to_vec());
            turns_temp = result.1;
            score_temp = result.0;
            if turns_temp > turns {
                turns = turns_temp;
                score = score_temp;
            }
            current_board.fill(0 as i32);
            row_in_board = 0;
        }
        else{
            //  do the parsing of the board values here
            for (i,split_line) in line.split_ascii_whitespace().collect::<Vec<&str>>().iter().enumerate(){
                current_board[[row_in_board,i]]=i32::from_str_radix(split_line, 10).unwrap();
            }
            row_in_board += 1;
        }

    }
    println!("task 2 the score is {}",score);
    println!("task 2 turns are : {}",turns);
}

fn main(){
    task1();
    task2();
}