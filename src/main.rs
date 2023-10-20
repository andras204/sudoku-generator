mod board;

use rand::seq::SliceRandom;
use std::{env, process::exit};
use board::Board;

fn main() {
    let mut args = env::args().skip(1);

    let mut difficulty: u8 = 0;
    let mut show_iter = false;

    while let Some(arg) = args.next() {
        match &arg[..] {
            "-h" | "--help" => {
                help_text();
                exit(0);
            },
            "-d" | "--difficulty" => {
                difficulty = args.next().expect("missing argument").parse::<u8>().expect("incorrect argument");
            },
            "-s" | "--show-iterations" => {
                show_iter = true;
            },
            _ => {
                println!("unknown argumet");
                help_text();
                exit(1);
            }
        }
    }

    let mut game = Board::new();

    if show_iter {
        iterate(&mut game, difficulty);
        exit(0);
    }
    iterate_clean(&mut game, difficulty);
    exit(0);
}

fn iterate(board: &mut Board, difficulty: u8) {
    let mut i = 0;
    for _x in 0..81 {
        let cell = board.select_random_cell();

        let step = board.select_random_number(cell);

        board.play_move(step.0, step.1, step.2);

        i += 1;
        let s = board.to_string().trim_end().to_owned();
        println!("------------------{i}\n{s}");

        if board.is_finished() { break; }
    }

    let mut rng = rand::thread_rng();

    for _x in 0..difficulty {
        board.reset_cell(board.get_collapsed_vec().choose(&mut rng).unwrap());

        i += 1;
        let s = board.to_string().trim_end().to_owned();
        println!("------------------{i}\n{s}");
    }


    println!("finished in {i} iterations");
}

fn iterate_clean(board: &mut Board, difficulty: u8) {
    for _x in 0..81 {
        let cell = board.select_random_cell();

        let step = board.select_random_number(cell);

        board.play_move(step.0, step.1, step.2);

        if board.is_finished() { break; }
    }

    let mut rng = rand::thread_rng();

    for _x in 0..difficulty {
        board.reset_cell(board.get_collapsed_vec().choose(&mut rng).unwrap());
    }

    let s = board.to_string().trim_end().to_owned();
    println!("{s}");
}

fn help_text() {
    println!(
"
Usage:
sudoku [args]

-d\t--difficulty [0-81]\tset number of empty cells in output
-s\t--show-iterations\toutput every generation step
-h\t--help\t\t\tshow this help text
"
             );
}
