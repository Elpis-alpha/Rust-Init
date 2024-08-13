use std::io;
const EMPTY_SQUARE: char = ' ';
const X_SQUARE: char = 'X';
const O_SQUARE: char = 'O';

fn main() {
    println!("Welcome to Tic Tac Toe! \n\n\n");
    let mut board_array: [char; 9] = [EMPTY_SQUARE; 9];
    print_board(board_array);

    loop {
        let mut player_input: String;

        // Player 1 (X) turn
        loop {
            player_input = String::new();
            println!("Player 1 (X), please enter a number between 0 and 8: ");
            io::stdin() // :: indicates that new is an associated function of the stdin type
                .read_line(&mut player_input)
                .expect("Failed to read line");

            match player_input.trim().parse::<usize>() {
                Ok(num) => {
                    if num > 8 {
                        println!("Number must be between 0 and 8 \n\n");
                        continue;
                    } else if board_array[num] != EMPTY_SQUARE {
                        println!("Square already taken! \n\n");
                        continue;
                    } else {
                        println!("\n\n");
                        board_array[num] = X_SQUARE;
                        print_board(board_array);
                        break;
                    }
                }
                Err(_) => {
                    println!("Invalid input!! \n\n\n");
                    continue;
                }
            }
        }

        // Check if player 1 (X) wins
        if check_winner(board_array) {
            println!("Player 1 (X) wins! \n\n");
            break;
        } else if check_tie(board_array) {
            println!("It's a tie! \n\n");
            break;
        }

        // Player 2 (O) turn
        loop {
            player_input = String::new();

            println!("Player 2 (0), please enter a number between 0 and 8: ");
            io::stdin()
                .read_line(&mut player_input)
                .expect("Failed to read line");

            match player_input.trim().parse::<usize>() {
                Ok(num) => {
                    if num > 8 {
                        println!("Number must be between 0 and 8 \n\n");
                    } else if board_array[num] != EMPTY_SQUARE {
                        println!("Square already taken! \n\n");
                        continue;
                    } else {
                        println!("\n\n");
                        board_array[num] = O_SQUARE;
                        print_board(board_array);
                        break;
                    }
                }
                Err(_) => {
                    println!("Invalid input!! \n\n");
                    continue;
                }
            }
        }
        
        // Check if player 2 (O) wins
        if check_winner(board_array) {
            println!("Player 2 (O) wins! \n\n");
            break;
        } else if check_tie(board_array) {
            println!("It's a tie! \n\n");
            break;
        }
    }
}

fn print_board(board_array: [char; 9]) {
    println!(
        " {} | {} | {}            0 | 1 | 2 ",
        board_array[0], board_array[1], board_array[2]
    );
    println!("-----------          -----------");
    println!(
        " {} | {} | {}     ->     3 | 4 | 5 ",
        board_array[3], board_array[4], board_array[5]
    );
    println!("-----------          -----------");
    println!(
        " {} | {} | {}            6 | 7 | 8 \n\n\n",
        board_array[6], board_array[7], board_array[8]
    );
}

fn check_winner(board_array: [char; 9]) -> bool {
    let winning_combinations: [[usize; 3]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];

    for combination in winning_combinations.iter() {
        if board_array[combination[0]] == board_array[combination[1]]
            && board_array[combination[1]] == board_array[combination[2]]
            && board_array[combination[0]] != EMPTY_SQUARE
        {
            return true;
        }
    }
    return false;
}

fn check_tie(board_array: [char; 9]) -> bool {
    for square in board_array.iter() {
        if *square == EMPTY_SQUARE {
            return false;
        }
    }
    return true;
}