use {
    // bincode::{serialize, Result},
    std::{
        env, fs::{File, OpenOptions}, io::{Read, Write},
    }
};

const NONE: u8 = 0;
const LEFT: u8 = 1;
const DOWN: u8 = 2;
const DIAG: u8 = 3;
const DIAG_MATCH: u8 = 4;
const DOWN_EMPTY:u8 = 6;


fn main() {

    let args: Vec<String> = env::args().collect();
    let input_file: &str = &args[1];
    let mode: &str = &args[2];
    let mis_pen: usize = str::parse::<usize>(&args[3]).unwrap();
    let gap_pen: usize = str::parse::<usize>(&args[4]).unwrap();
    let output_file: &str = &args[5];

    let mut file = File::open(&input_file).expect("Unable to open file");

    let mut reads: Vec<u8> = Vec::new();
    file.read_to_end(&mut reads).expect("Error reading file");

    let mut iter = reads.split(|ele| *ele == b'\n');
    let mut name = iter.next();
    let mut x = iter.next();
    let mut y = iter.next();

    let _ = File::create_new(output_file);
    let mut out = OpenOptions::new()
        .write(true)
        .append(true)
        .open(output_file)
        .unwrap();

    if mode == "global" {
        while name != None {
            _ = writeln!(out,"{}",calc_opt_alignment_global(name.unwrap(),x.unwrap(),y.unwrap(),gap_pen,mis_pen));
            name = iter.next();
            x = iter.next();
            y = iter.next();
        }
    } else {
        while name != None {
            _ = writeln!(out,"{}",calc_opt_alignment_fitting(name.unwrap(),x.unwrap(),y.unwrap(),gap_pen,mis_pen));
            name = iter.next();
            x = iter.next();
            y = iter.next();
        }
    }
}

fn calc_opt_alignment_global(header: &[u8], x: &[u8],y: &[u8], gap_pen: usize, mis_pen: usize) -> String {
    
    let mut l_gap ;
    let mut mism;
    let mut d_gap;
    let mut min;
    let mut is_match;

    let mut arr = vec![vec![(0, NONE); x.len()+1]; y.len()+1];

    for i in 0..y.len()+1{
        arr[i][0] = (gap_pen * i, DOWN);
    }
    for j in 0..x.len()+1{
        arr[0][j] = (gap_pen * j,  LEFT);
    }

    for i in 1..arr.len() {
        for j in 1..arr[i].len() {

            l_gap = arr[i][j-1].0 + gap_pen;
            d_gap = arr[i-1][j].0 + gap_pen;

            if y[i-1] == x[j-1] {
                mism = arr[i-1][j-1].0;
                is_match = true;

            } else{
                mism = arr[i-1][j-1].0 + mis_pen ;
                is_match = false;
            }

            min = std::cmp::min(mism,std::cmp::min(l_gap,d_gap));

            arr[i][j].0 = min;

            if min == mism {
                if is_match {
                    arr[i][j].1 = DIAG_MATCH
                } else {
                    arr[i][j].1 = DIAG;
                }
            } else if min == l_gap {
                arr[i][j].1 = LEFT;
            } else {
                arr[i][j].1 = DOWN
            }
        }
    }

    // reconstruct cigar string

    let mut i: usize = arr.len()-1;
    let mut j: usize = arr[i].len()-1;
    let mut cigar = "".to_string();

    let mut curr_op;
    let mut run_len = 1;
    let mut curr_char = '*';


    while i > 0 || j > 0 {

        curr_op = arr[i][j].1;

        if curr_op == DIAG {
            i -= 1;
            j -= 1;
            curr_char = 'X';
            
        } else if curr_op == DIAG_MATCH {
            i -= 1;
            j -= 1;
            curr_char = '=';

        } else if curr_op == LEFT {
            j -= 1;
            curr_char = 'I';

        } else if curr_op == DOWN {
            i -= 1;
            curr_char = 'D';

        }

        if curr_op == arr[i][j].1{
            run_len += 1;
        } else {
            cigar = format!("{}{}{}",run_len,curr_char,cigar);
            run_len = 1;
        }

    }

    return format!("{}\n{}\n{}\n-{}\t0\t{}\t{}",
        std::str::from_utf8(header).unwrap(),
        std::str::from_utf8(x).unwrap(),
        std::str::from_utf8(y).unwrap(),
        arr[y.len()][x.len()].0,
        y.len(),
        cigar
    );

}

fn calc_opt_alignment_fitting(header: &[u8], x: &[u8],y: &[u8], gap_pen: usize, mis_pen: usize) -> String {
    
    let mut l_gap ;
    let mut mism;
    let mut d_gap;
    let mut min;
    let mut is_match;

    let mut arr = vec![vec![(0, NONE); x.len()+1]; y.len()+1];

    for i in 0..y.len()+1{
        arr[i][0] = (0, DOWN_EMPTY);
    }
    for j in 1..x.len()+1{
        arr[0][j] = (gap_pen * j, LEFT);
    }

    for i in 1..arr.len() {
        for j in 1..arr[i].len() {

            l_gap = arr[i][j-1].0 + gap_pen;
            d_gap = arr[i-1][j].0 + gap_pen;

            if y[i-1] == x[j-1] {
                mism = arr[i-1][j-1].0;
                is_match = true;

            } else{
                mism = arr[i-1][j-1].0 + mis_pen ;
                is_match = false;
            }

            min = std::cmp::min(mism,std::cmp::min(l_gap,d_gap));

            arr[i][j].0 = min;

            if min == mism {
                if is_match {
                    arr[i][j].1 = DIAG_MATCH
                } else {
                    arr[i][j].1 = DIAG;
                }
            } else if min == l_gap {
                arr[i][j].1 = LEFT;
            } else {
                arr[i][j].1 = DOWN
            }
        }
    }

    // reconstruct cigar string

    let mut i: usize = arr.len()-1;
    let mut j: usize = arr[i].len()-1;
    let mut start = 0;
    let mut cigar = "".to_string();
    let mut curr_char = '*';
    let mut end = x.len();

    let mut curr_op;
    let mut run_len = 1;

    let mut min = arr[0][x.len()].0;
    for f in 0..arr.len() {
        if arr[f][x.len()].0 < min {
            start = f;
            min = arr[f][x.len()].0;
        }
    }

    i = start;
    while i > 0 || j > 0 {

        curr_op = arr[i][j].1;

        if curr_op == DIAG {
            i -= 1;
            j -= 1;
            curr_char = 'X';

        } else if curr_op == DIAG_MATCH {
            i -= 1;
            j -= 1;
            curr_char = '=';

        } else if curr_op == LEFT {
            j -= 1;
            curr_char = 'I';

        } else if curr_op == DOWN {
            i -= 1;
            curr_char = 'D';

        } else if curr_op == DOWN_EMPTY {
            end = i;
            i = 0;
            j = 0;
        }

        if curr_op == arr[i][j].1{
            run_len += 1;
        } else {
            cigar = format!("{}{}{}",run_len,curr_char,cigar);
            run_len = 1;
        }
    }


    return format!("{}\n{}\n{}\n-{}\t{}\t{}\t{}",
        std::str::from_utf8(header).unwrap(),
        std::str::from_utf8(x).unwrap(),
        std::str::from_utf8(y).unwrap(),
        min,
        end,
        start,
        cigar
    );

}
