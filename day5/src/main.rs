use std::fs;
use ndarray::prelude::*;

fn main() {
    let input_text = fs::read_to_string("sample.txt")
    // let input_text = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    let mut nbr_lines : usize = 0;
    for line in input_text.lines(){
        if !line.is_empty(){nbr_lines += 1;}
    }
    let mut input_vec = Array2::<usize>::zeros((nbr_lines,4));
    for (j,line) in input_text.lines().enumerate(){
        let words = line.split(" ").collect::<Vec<&str>>();
        for (i,word) in words.iter().enumerate(){
            if i == 0 {
                // parse as source
                let numbers = word.split(",").collect::<Vec<&str>>();
                for (k,split) in numbers.iter().enumerate(){
                    let temp = usize::from_str_radix(split, 10).unwrap();
                    input_vec[[j,k]] = temp;
                }

            }
            if i == 2 {
                // parse as destionation
                let numbers = word.split(",").collect::<Vec<&str>>();
                for (k,split) in numbers.iter().enumerate(){
                    let temp = usize::from_str_radix(split, 10).unwrap();
                    input_vec[[j,k+2]] = temp;
                }

            }
        }
    }
    let mut max_all : usize = 0;
    for (i, row) in input_vec.axis_iter(Axis(1)).enumerate() {
        let (max_idx, max_val) =
            row.iter()
                .enumerate()
                .fold((0, row[0]), |(idx_max, val_max), (idx, val)| {
                    if &val_max > val {
                        (idx_max, val_max)
                    } else {
                        (idx, *val)
                    }
                });
        if max_all < max_val { max_all = max_val}
    }
    let size = max_all as usize + 1;
    let mut board = Array2::<i32>::zeros((size,size));
    println!("{}",input_vec);
    for row in input_vec.axis_iter(Axis(0)){
        // add +1 to all elements in slice
        if row[1] == row[3]{
            if row[0] > row[2] {
                let mut slice = board.slice_mut(s![row[1],row[2]..row[0]+1]);
                slice += 1;
            }
            else{
                let mut slice = board.slice_mut(s![row[1],row[0]..row[2]+1]); 
                slice += 1;
            }
        }
        if row[0] == row[2]{
            if row[1] < row[3]{
                let mut slice = board.slice_mut(s![row[1]..row[3]+1,row[2]]); 
                slice += 1;
            }
            else{
                let mut slice = board.slice_mut(s![row[3]..row[1]+1,row[2]]); 
                slice += 1;
            }
        }
        if (row[0] != row[2]) | (row[1] != row[3]){
            if row[2] == row[3]{
                if row[0]<row[2]{let start_idx = row[0];let end_idx = row[2] + 1;
                    for i in start_idx..end_idx{
                        board[[i,row[1] + i]] += 1;
                    }}
                else{let start_idx = row[2];let end_idx = row[0] + 1;
                    for i in start_idx..end_idx{
                        board[[i,row[1] + i]] += 1;
                    }}   
            }
            if row[2] < row[3]{
                
                for i in row[2]..row[3]+1{

                }
            }
            if row[3] > row[2]{

            }
        }
    }
    
    println!("{}",board);
    let mut counter = 0;
    for element in board.iter(){
        let temp = *element;
        if temp > 1{
            counter += 1;
        }
    }

    println!("points with min two lines : {}", counter);

}
