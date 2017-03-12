extern crate rand;

use std::env;
use rand::Rng;
use std::process;

fn main() {
    let num_queens = env::args().nth(1).unwrap();
    let num_queens = num_queens.parse::<u32>().unwrap();

    let mut board : Vec<u32> = Vec::new();
    init_board(&mut board, num_queens);
    repair(&mut board);
    println!("\nSOLVED");
    print_board(&mut board);
}

// Calculates how many collisions a given position has on the board

fn count_conflicts(x : u32, y : u32, board: &mut Vec<u32>) -> u32 {
    let mut num_conflict = 0;
    let y = y as i32;
    let x = x as i32;
    let mut count_row = false;
    let mut count_diag_left = false;
    let mut count_diag_right = false;
    let len : i32 = board.len() as i32;
    for pos in 0..len {
        if x != pos {
            // Queens in the same row

            if board[pos as usize] == y as u32 && count_row == false {
                num_conflict += 1;
                count_row = true;
            }

            // Diagonals

            let diff = (x - pos).abs();
            if board[pos as usize] as i32 == y - diff && count_diag_left == false {
                num_conflict += 1;
                count_diag_left = true;
            }
            if board[pos as usize] as i32 == y + diff && count_diag_right == false {
                num_conflict += 1;
                count_diag_right = true;
            }
        }
    }
    num_conflict
}

// Initializes the nxn board with random positions

fn init_board(board : &mut Vec<u32>, num_queens : u32) {
    board.truncate(0);
    let mut rng = rand::thread_rng();
    let mut num;
    for _ in 0..num_queens {
        num = rng.gen::<u32>() % num_queens;
        while board.contains(&num) {
            num = rng.gen::<u32>() % num_queens;
        }
        board.push(num);
    }
}

// Main function to perform the heuristic repair

fn repair(board : &mut Vec<u32>) {
    let mut num_conflicts;
    let mut has_moved = true;
    let mut passes = 0;
    let mut smallest_conf;
    let mut contestant_conf;
    let mut smallest_pos;
    let len = board.len();
    while has_moved {
        passes += 1;
        has_moved = false;
        // print_board(board);
        for x in 0..board.len() {
            smallest_pos = board[x];
            num_conflicts = count_conflicts(x as u32, board[x], board);
            smallest_conf = num_conflicts;
            if num_conflicts > 0 {
                has_moved = true;
                for y in 0..len {
                    if y != board[x] as usize {
                    contestant_conf = count_conflicts(x as u32,y as u32,board);
                    if contestant_conf <= smallest_conf {
                        smallest_conf = contestant_conf;
                        smallest_pos = y as u32;
                    }
                    }
                }
                if smallest_pos != board[x] {
                    board[x] = smallest_pos;
                }
            }
        }
        if passes > len as i32 {
            println!("Could not solve from that starting position. Please try again.");
            std::process::exit(0);
        }
    }
    print!("Passes : {}", passes);
}

// Helper function to print the board to stdout.

fn print_board(board : &mut Vec<u32>) {
    let len : usize = board.len();
    for x in 0..len {
        for y in 0..len {
            if board[x] == y as u32 {
                print!("[Q]");
            }
            else {
                print!("[ ]");
            }
        }
        println!("");
    }
}
