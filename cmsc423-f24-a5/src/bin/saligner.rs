use {
    // bincode::{serialize, Result},
    std::{
        env, fs::File, io::{Read},
    }
};
fn main() {

let args: Vec<String> = env::args().collect();
let input_file: &str = &args[1];
let mis_pen: isize = str::parse::<isize>(&args[2]).unwrap();
let gap_pen: isize = str::parse::<isize>(&args[3]).unwrap();
let output_file: &str = &args[4];

let mut file = File::open(&input_file).expect("Unable to open file");

let mut reads: Vec<u8> = Vec::new();
file.read_to_end(&mut reads).expect("Error reading file");
let mut outputs: Vec<String> = Vec::new();

let mut iter = reads.split(|ele| *ele == b'\n');
let mut name = iter.next();
let mut x = iter.next();
let mut y = iter.next();

while name != None {
    outputs.push(calc_opt_alignment(name.unwrap(),x.unwrap(),y.unwrap()));
    name = iter.next();
    x = iter.next();
    y = iter.next();
}

}

fn calc_opt_alignment(header: &[u8], x: &[u8],y: &[u8]) -> String {
    println!{"{:?}:\n\t{:?}\n\t{:?}",std::str::from_utf8(header).unwrap(),std::str::from_utf8(x).unwrap(),std::str::from_utf8(y).unwrap()};
    return "".to_string();
}
