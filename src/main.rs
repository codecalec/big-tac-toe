use anyhow::{anyhow, Result};

use big_tac_toe::{Marking, OuterBoard};

fn get_position(input: String) -> Result<(usize, usize)> {
    let mut v: Vec<Result<usize, std::num::ParseIntError>> =
        input.split_whitespace().map(|s| s.parse::<usize>()).collect();

    if v.len() != 2 {
        return Err(anyhow!("Not enough argument"));
    }

    let (i, j) = (v.remove(0)?, v.remove(0)?);
    if i > 3 || j > 3 || i == 0 || j == 0 {
        return Err(anyhow!("Bad values: ({}, {})", i, j));
    }

    Ok((i-1, j-1))
}

fn main() {
    let nought = Marking::Nought;
    let cross = Marking::Cross;

    let mut outer = OuterBoard::new();

    println!("Give first block: \033[3mrow col\033[0m");

    let mut num_turns = 1u32;

    let (i, j);
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Issue reading input");
        let temp = input.clone();
        match get_position(input) {
            Ok((row, col)) => {
                i = row;
                j = col;
                break;
            },
            Err(e) => println!("Error: {}\nInput: {}", e.root_cause(), temp),
        }

    }

    let mut next_loc = (i, j);
    // Game Loop
    loop {
        let current = if num_turns.rem_euclid(2) == 0 {
            &nought
        } else {
            &cross
        };
        num_turns += 1;

        println!("It is {}'s turn in board {}:{}", current, next_loc.0 + 1, next_loc.1 + 1);

        // Turn Loop
        loop {
            println!("Enter your move (row col): ");
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Issue reading input");

            match get_position(input) {
                Ok((row, col)) => {
                    match outer.place(next_loc.0, next_loc.1, &row, &col, current) {
                        Ok(x) => {
                            print!("\x1B[2J\x1B[1;1H"); // Clear Terminal window
                            next_loc = x;
                            break;
                        },
                        Err(e) => {
                            println!("{}", e);
                        },
                    }
                },
                Err(e) => println!("{}", e),
            }

        }
        println!("{}", outer);
        println!("{}", outer.master_board);
    }
}
