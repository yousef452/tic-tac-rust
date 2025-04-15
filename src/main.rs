use std::io;


const WHITE_SPACE : char = ' ';
const LOSE_CHAR : char = ',';


fn print_board(board : [char;9]) {
    println!("{} | {} | {}",board[0],board[1],board[2]);
    println!("---------");
    println!("{} | {} | {}",board[3],board[4],board[5]);
    println!("---------");
    println!("{} | {} | {}",board[6],board[7],board[8]);

}

fn winner(board : [char;9]) -> char {
    for row in 0..3 {
        let idx = row *3;
        if board[idx] != WHITE_SPACE && board[idx+1] == board[idx] && board[idx+2] == board[idx] {
            return board[idx];
        }
    }

    for col in 0..3 {
        if board[col] != WHITE_SPACE && board[col+3] == board[col] && board[col+6] == board[col] {
            return board[col];
        }
    }

    if board[0] != WHITE_SPACE && board[4] == board[0] && board[8] == board[0] {
        return board[0];
    }

    if board[2] != WHITE_SPACE && board[4] == board[2] && board[6] == board[2] {
        return board[2];
    }

    return LOSE_CHAR;
}

fn main() {
    let player : [char;2] = ['X','O'];
    let mut board : [char;9] = [WHITE_SPACE;9];

    let mut switch : usize = 0;

    loop {
        print_board(board);
        println!("P{} turn : ",switch);

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("   ___ERROR___   can not read user input");

        let mut index : usize = user_input.trim().parse().expect("can not turn the input into unsigned int");
        index -= 1;
        if index > 8 {
            println!("[TRY AGAIN] enter number less than 9");
            continue;
        }
        else if board[index] != WHITE_SPACE {
            println!("[TRY AGAIN] this place is already taken by: {}",board[index]);
            continue;
        }
        board[index] = player[switch];
        switch+=1;
        switch = switch % 2;

        let winner_ = winner(board);
        if winner_ != LOSE_CHAR {
            print_board(board);
            println!("{} WIN GG",winner_);
            break;
        }
    }
}
