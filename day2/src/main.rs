use std::fs;
fn task1() {
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    for line in contents.lines(){
        if !line.is_empty(){
        
            let mut split = line.split_whitespace();
            let instruction = split.next().unwrap_or(" ");
            let units: i32 = split.next().unwrap().parse().ok().unwrap_or(0);
            //current = line.parse().unwrap();
            match instruction {
                "forward" => horizontal = horizontal + units,
                "down" => depth = depth + units,
                "up" => depth = depth - units, 
                _ => println!("could also be empty"),

            }
        }
    }
    println!("Result from part 1 is: {}",horizontal * depth);
}

fn task2() {
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim : i32 = 0;
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    for line in contents.lines(){
        if !line.is_empty(){
        
            let mut split = line.split_whitespace();
            let instruction = split.next().unwrap_or(" ");
            let units: i32 = split.next().unwrap().parse().ok().unwrap_or(0);
            //current = line.parse().unwrap();
            match instruction {
                "forward" => {horizontal = horizontal + units;
                depth = depth + aim*units;},
                "down" => aim = aim + units,
                "up" => aim = aim - units, 
                _ => println!("could also be empty"),

            }
        }
    }
    println!("Result from part 2 is: {}",horizontal * depth);
}


fn main(){
    task1();
    task2();
}
