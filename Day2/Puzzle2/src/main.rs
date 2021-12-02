use std::fs::File;
use std::io::{BufRead, BufReader};

struct Submarine {
    hor_pos : i32,
    depth   : i32,
    aim     : i32,
}
fn main(){
    let file_handle = File::open("commands.txt").expect("Error while opening file");

    let reader = BufReader::new(file_handle);

    let mut submarine = Submarine{hor_pos : 0, depth : 0, aim : 0};

    for (_index,line) in reader.lines().enumerate(){

        let buff  = line.unwrap();
        let line_split : Vec<&str> = buff.split(" ").collect();

        let command = line_split[0];

        let val : i32 = line_split[1].parse().unwrap();
        match command{
            

            "forward" => {submarine.hor_pos += val; submarine.depth += submarine.aim * val},
            "down" => submarine.aim += val,
            "up" => submarine.aim -= val,
            _ =>{}
        }
    }



    println!("The result is {}", submarine.depth * submarine.hor_pos);

}