use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {

    let file_handle = File::open("radar.txt").expect("Error while opening file");

    let reader = BufReader::new(file_handle);

    let mut values : Vec<i32> = Vec::new();
    

    for (_index,line) in reader.lines().enumerate(){
        let buff : i32 = line.unwrap().parse().unwrap();
        values.push(buff);

    }


    let mut last_sum = -1;
    let mut current_sum = 0;

    let mut sum_increase = 0;


    for (i,_val) in values.iter().enumerate()
    {
        if i +1 < values.len() && i +2 < values.len()
        {
            current_sum = values[i] + values[i+1] + values[i +2];
        }
        if current_sum > last_sum && last_sum != -1 
        {
            sum_increase = sum_increase +1;
        }

        last_sum = current_sum;

    }

    println!("Number of increases in sum : {}", sum_increase);
}
