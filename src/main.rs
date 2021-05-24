use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Don't let the machines win. You are humanity's last hope...
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let width = parse_input!(input_line, i32); // the number of cells on the X axis
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let height = parse_input!(input_line, i32); // the number of cells on the Y axis
    let node = (-1, -1);
    let node_right = (-1, -1);
    let node_bottom = (-1, -1);
    let node_list: Vec<(usize, usize)>;
    for i in 0..height as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line: Vec<(usize, &str)> = input_line.split("\n").enumerate().collect(); // width characters, each either 0 or .
        
    }

    // Write an action using println!("message...");
    // To debug: eprintln!("Debug message...");


    // Three coordinates: a node, its right neighbor, its bottom neighbor
    println!("0 0 1 0 0 1");
}
