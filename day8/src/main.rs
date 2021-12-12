use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use itertools::Itertools;
use undup::undup_chars;
fn task1(){
    // let input_text = fs::read_to_string("sample.txt")
    let input_text = fs::read_to_string("input.txt")
    .expect("Something went wrong reading the file");
    let mut encoding : Vec<&str> = vec![" "," "];
    let mut numbers : Vec<&str> = vec![" ", " "];
    let mut active_segments = vec![0;8];
    for line in input_text.lines(){
        let parts = line.split("|").collect::<Vec<&str>>();
        // encoding = parts[0].split(" ").collect::<Vec<&str>>();
        numbers = parts[1].split(" ").collect::<Vec<&str>>();
        for element in numbers.iter(){
            active_segments[element.len()] +=1;
        }
    }
    
    println!("encoding : {:?}, numbers : {:?}",encoding,numbers);
    println!("total of numbers with active segments {:?}",active_segments);
    let answer = active_segments[2] + active_segments[3] + active_segments[4] + active_segments[7];
    println!("total number of digits 1, 4, 7, 8 : {}",answer);

}
fn task2(){
    let mut  segment_std_wiring = HashMap::new();
    segment_std_wiring.insert(
        "abcefg".to_string(),
        "0",);
    segment_std_wiring.insert(
        "cf".to_string(),
        "1",);
    segment_std_wiring.insert(
        "acdeg".to_string(),
        "2",);
    segment_std_wiring.insert(
        "acdfg".to_string(),
        "3",);
    segment_std_wiring.insert(
        "bcdf".to_string(),
        "4",);
    segment_std_wiring.insert(
        "abdfg".to_string(),
        "5",);
    segment_std_wiring.insert(
        "abdefg".to_string(),
        "6",);
    segment_std_wiring.insert(
        "acf".to_string(),
        "7",);
    segment_std_wiring.insert(
        "abcdefg".to_string(),
        "8",);
    segment_std_wiring.insert(
        "abcdfg".to_string(),
        "9",);
    
    // let input_text = fs::read_to_string("input.txt")
    let input_text = fs::read_to_string("sample.txt")
    .expect("Something went wrong reading the file");
    let mut encoding : Vec<&str> = vec![" "," "];
    // let mut numbers : Vec<&str> = vec![" ", " "];
    // let mut active_segments = vec![0;8];
    // let mut total = 0;
    for line in input_text.lines(){
        let parts = line.split("|").collect::<Vec<&str>>();
        encoding = parts[0].split(" ").collect::<Vec<&str>>();
        let mut two_segments = "";
        let mut three_segments = two_segments;
        let mut four_segments = two_segments;
        let mut five_segments = vec![];
        let mut six_segments = vec![];
        let mut seven_segments = two_segments;
        for code in encoding.iter(){
            let length = code.len();
            match length {
                2 => two_segments = code,
                3 => three_segments = code,
                4 => four_segments = code,
                5 => five_segments.push(code),
                6 => six_segments.push(code),
                7 => seven_segments = code,
                0 => (),
                _ => (),
            }
            //  figure out encoding and store into hashmap  
        }
        println!("{}",two_segments);
        println!("{}",three_segments);
        println!("{}",four_segments);
        println!("{:?}",five_segments);
        println!("{:?}",six_segments);
        println!("{}",seven_segments);
        
        let mut lookup_encoding = HashMap::new();
        let a = non_shared_character(two_segments,three_segments);
        let mut  b_or_d = non_shared_character(four_segments, two_segments);
        // let mut unique_five = non_shared_character_vector(five_segments);
        // let mut d_or_g = non_shared_character_vector(five_segments, &a);
        let mut temp = format!("{}{}{}", five_segments[0], five_segments[1],five_segments[2]);
        temp = temp.chars().sorted().collect::<String>();
        println!("{}",temp);
        temp = remove_double_characters(&temp);
        // temp = undup_chars(&temp, temp.chars().collect());
        println!("{}",temp);
        // let mut d = shared_characters(&b_or_d,&d_or_g);
        // d_or_g = undup_chars(&d_or_g, d_or_g.chars().collect());
        // d = undup_chars(&d, d.chars().collect());
        
        // b_or_d = undup_chars(&four_segments, two_segments.chars().collect());
        // println!("b or d :{}  d or g : {}",b_or_d,d_or_g);
        // println!("d :{}",d);
        


        lookup_encoding.insert(
            a,
            "a",);
        // println!("{}",non_shared_character(two_segments,three_segments));
        // let turned_on_wires = parts[1].split(" ").collect::<Vec<&str>>();
        // let mut numbers_displayed = "".to_owned();
        // for element in numbers.iter(){
        //     let corrected_code =lookup_encoding.get(element).unwrap();
        //     let number_displayed = segment_std_wiring.get(corrected_code).unwrap().to_owned();
        //     numbers_displayed.push_str(number_displayed);
        // }
        // total += i32::from_str_radix(&numbers_displayed, 10).unwrap();
    }
    
    // println!("total from all displays {}",total);

}
fn remove_double_characters(a : &str) -> String{
    let mut non_doubled = "".to_owned();
    for character in a.chars(){
        let mut not_doubled = true;
        for check_char in non_doubled.chars(){
            if check_char == character{ not_doubled = false;}
        }
        if not_doubled {
            non_doubled.push(character);
        }
    }
    return non_doubled
}
fn shared_characters(a: &str,b : &str) -> String{
    let mut shared = "".to_owned();
    let (shorter, longer) = if a.len() > b.len() {
        (b, a)
    }  else {
        (a, b)
    };
    for char_longer in longer.chars(){
        let mut non_unique_char = false;
        for char_shorter in shorter.chars(){
            if char_longer == char_shorter{
                non_unique_char = true;
            } 
        }
        let adding_to_str = char::to_string(&char_longer) ;
        if non_unique_char {
            shared.push_str(&adding_to_str);
        }
    }
    return shared
}

fn non_shared_character(a: &str,b : &str) -> String{
    let mut non_shared = "".to_owned();
    let (shorter, longer) = if a.len() > b.len() {
        (b, a)
    }  else {
        (a, b)
    };
    for char_longer in longer.chars(){
        let mut unique_char = true;
        for char_shorter in shorter.chars(){
            if char_longer == char_shorter{
                unique_char = false;
            } 
        }
        let adding_to_str = char::to_string(&char_longer) ;
        if unique_char {
            non_shared.push_str(&adding_to_str);
        }
    }
    return non_shared
}
fn shared_chars(a: &str, b: &str) -> bool {
    // get which one is shorter
    let (shorter, longer) = if a.len() > b.len() {
        (b, a)
    }  else {
        (a, b)
    };

    // fill the set with the characters from the shorter string
    let set: HashSet<char> = shorter.chars().collect();

    longer.chars().any(|c| set.contains(&c))

}
fn main() {
    // task1();
    task2();
}
