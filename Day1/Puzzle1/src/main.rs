use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {

    let file_handle = File::open("radar.txt").expect("Error while opening file");

    let reader = BufReader::new(file_handle);

    let mut values : Vec<i32> = Vec::new();
    

    for (index,line) in reader.lines().enumerate(){
        let buff : i32 = line.unwrap().parse().unwrap();
        values.push(buff);

    }


    let mut last_value = -1;

    let mut depth_change = 0;

    for value in values.iter(){
        if(value > &last_value && last_value != -1){
            depth_change = depth_change +1;
        }

        last_value = *value;

    }

    println!("Number of increases in depth : {}", depth_change);


}
