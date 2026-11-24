use std::io;
use rand::prelude::*;

enum GameState {
    Playing,
    Won,
    Lost,
}

struct Board {
    width: i32,
    height: i32,
    minecount: i32,
}

const BEGINNER: Board = Board {
    width: 9,
    height: 9,
    minecount: 10,
};
const INTERMEDIATE: Board = Board {
    width: 16,
    height: 16,
    minecount: 40,
};
const EXPERT: Board = Board {
    width: 30,
    height: 16,
    minecount: 99,
};

fn main() {
    //println!("\x1b[48;5;234m\x1b[38;5;13m!\x1b[38;5;8m#\x1b[38;5;12m1\x1b[38;5;2m2\x1b[38;5;9m3\x1b[38;5;5m4\x1b[38;5;1m5\x1b[38;5;6m6\x1b[38;5;0m7\x1b[38;5;7m8\x1b[39m\x1b[49m");
    //println!("\x1b[48;5;235m\x1b[38;5;13m!\x1b[38;5;8m#\x1b[38;5;12m1\x1b[38;5;2m2\x1b[38;5;9m3\x1b[38;5;5m4\x1b[38;5;1m5\x1b[38;5;6m6\x1b[38;5;0m7\x1b[38;5;7m8\x1b[39m\x1b[49m");
    println!("\x1b[38;5;13mWelcome to minesweeper!\x1b[39m\n");

    let board: Board;
    println!("Select your board size");
    println!("Type \"beginner\", \"intermediate\", or \"expert\" or input a custom board size in the form <width>x<height>/<minecount>");
    println!("(For example: 30x16/99)");
    loop {
        let mut entry = String::new();
        io::stdin().read_line(&mut entry).expect("\x1b[38;5;13mfailed to read line\x1b[39m");
        entry = entry.trim().to_lowercase();
        if entry == String::from("beginner") {
            board = BEGINNER;
            break;
        }
        else if entry == String::from("intermediate") {
            board = INTERMEDIATE;
            break;
        }
        else if entry == String::from("expert") {
            board = EXPERT;
            break;
        }
        else {
            match read_size(entry) {
                Some(b) => {
                    board = b;
                    break;
                },
                None => continue,
            }
        }
    }
    let mut flags = 0;
    let mut cells = board.width * board.height - board.minecount;
    let mut game_state = GameState::Playing;

    let mut minefield = vec![vec![0; board.width as usize]; board.height as usize];
    //0 indicates a safe square; 1 indicates a mine
    let mines = choose_random(board.width * board.height, board.minecount);
    for m in &mines {
        //set the corresponding 0 in minefield to a 1
        minefield[(m / board.width) as usize][(m % board.width) as usize] = 1;
        //having to manually cast an int to a "usize" feels weird to me
    }
    let mut playfield = vec![vec![-1; board.width as usize]; board.height as usize];
    //-1 indicates an unopened tile; -2 indicates a flag; 0-8 indicate numbers

    let mut dig_buffer = vec![];

    let mut first_click = true;

    println!("To play, input commands in the form <action> <row> <column>");
    println!("<action> should be either 'd' to dig or 'f' to flag");
    println!("The row and column numbers should be 0-indexed");
    println!("For example, to dig the cell in the top-left corner, input \"d 0 0\"");

    loop {
        //print out field
        let mut a: i32 = 0;
        for row in &playfield {
            let mut b: i32 = 0;
            for cell in row {
                //put background color
                match ((a / 5) + (b / 5)) % 2 {
                    0 => print!("\x1b[48;5;234m"),
                    1 => print!("\x1b[48;5;235m"),
                    _ => print!("\x1b[48;5;9m\x1b38;5;0mSomething has gone horribly wrong."), //should logically never happen, i think; x % 2 should return either 0 or 1 for all natural numbers x
                }
                //the problem with using iterators here is that you can't naturally use the actual index
                match cell {
                    -1 => print!("\x1b[38;5;8m#"),
                    -2 => print!("\x1b[38;5;13m!"),
                    0 => print!(" "),
                    1 => print!("\x1b[38;5;12m1"),
                    2 => print!("\x1b[38;5;2m2"),
                    3 => print!("\x1b[38;5;9m3"),
                    4 => print!("\x1b[38;5;5m4"),
                    5 => print!("\x1b[38;5;1m5"),
                    6 => print!("\x1b[38;5;6m6"),
                    7 => print!("\x1b[38;5;0m7"),
                    8 => print!("\x1b[38;5;7m8"),
                    9 => print!("\x1b[38;5;13mX"),
                    _ => print!("\x1b[38;5;14m?"),
                }
                b += 1;
            }
            a += 1;
            print!("\x1b[39m\x1b[49m\n");
        }
        match game_state {
            GameState::Won => {
                println!("You win!");
                break;
            }
            GameState::Lost => {
                println!("You lose :(");
                break;
            }
            GameState::Playing => ()
        }
        println!("mines left: {}", board.minecount - flags);

        //await input
        let mut entry = String::new();
        io::stdin().read_line(&mut entry).expect("\x1b[38;5;13mfailed to read line\x1b[39m");
        let command = read_command(entry);
        //read input
        match command.0 {
            'd' => {
                if first_click {
                    if minefield[command.1 as usize][command.2 as usize] == 1 {
                        let (row, column) = safety(&mines, &minefield);
                        minefield[command.1 as usize][command.2 as usize] = 0;
                        minefield[row][column] = 1;
                    }
                    first_click = false;
                }
                let (success, state) = dig(command.1 as usize, command.2 as usize, &mut playfield, &minefield, &mut dig_buffer);
                cells -= success;
                game_state = state;
            },
            'f' => flags += flag(command.1 as usize, command.2 as usize, &mut playfield),
            _ => println!("\x1b[38;5;9mfailed to read command\x1b[39m"),
        }

        //clear out buffer
        while dig_buffer.len() > 0 {
            match dig_buffer.pop() {
                Some([y, x]) => {
                    let (success, _) = dig(y, x, &mut playfield, &minefield, &mut dig_buffer);
                    cells -= success;
                },
                None => (),
            }
        }

        if cells == 0 {
            game_state = GameState::Won;
        }
    }
}

//choose_random(total, number) -> numbers
//chooses <number> distinct random numbers from 0 to <total - 1>
fn choose_random(total: i32, number: i32) -> Vec<i32> {
    let mut result:Vec<i32> = vec![]; //the list of numbers to return
    let mut bag:Vec<i32> = vec![]; //the list of numbers to choose from

    //populate the bag
    for i in 0..total {
        bag.push(i);
    }

    //pick elements from the bag
    for _ in 0..number {
        let index = rand::rng().random_range(0..bag.len());
        let n = bag.swap_remove(index);
        result.push(n);
    }

    result
}

//read_command(entry) -> (command, row, column)
//parses a flag or dig command
fn read_command(entry: String) -> (char, i32, i32) {
    let entry: Vec<char> = entry.chars().collect();
    if entry.len() == 0 {
        return ('0', -1, -1);
    }
    let action = entry[0];

    let mut i = 1;
    while i < entry.len() && entry[i] != ' ' {
        i += 1;
    }
    i += 1;
    if i >= entry.len() {
        return('0', -1, -1);
    }

    let mut y = 0;
    while i < entry.len() && entry[i] != ' ' {
        if entry[i].is_numeric() {
            y = y * 10 + (entry[i] as i32 - '0' as i32); //there's probably a cleaner way to do this
        }
        else {
            return ('e', -1, -1);
        }
        i += 1;
    }
    i += 1;
    if i >= entry.len() {
        return('0', -1, -1);
    }

    let mut x = 0;
    while i < entry.len() && entry[i] != ' ' && entry[i] != '\n' {
        if entry[i].is_numeric() {
            x = x * 10 + (entry[i] as i32 - '0' as i32);
        }
        else {
            return ('e', -1, -1);
        }
        i += 1;
    }

    (action, y, x)
}

//read_size(entry) -> board
//parses a custom board size; returns None if it can't read it
fn read_size(entry: String) -> Option<Board> {
    let entry: Vec<char> = entry.chars().collect();
    let mut i = 0;

    let mut width = 0;
    while i < entry.len() && entry[i] != 'x' {
        if entry[i].is_numeric() {
            width = width * 10 + (entry[i] as i32 - '0' as i32); //there's probably a cleaner way to do this
        }
        else {
            println!("\x1b[38;5;13mfailed to read board size");
            return None;
        }
        i += 1;
    }
    i += 1;

    let mut height = 0;
    while i < entry.len() && entry[i] != '/' {
        if entry[i].is_numeric() {
            height = height * 10 + (entry[i] as i32 - '0' as i32); //there's probably a cleaner way to do this
        }
        else {
            println!("\x1b[38;5;13mfailed to read board size");
            return None;
        }
        i += 1;
    }
    i += 1;

    let mut minecount = 0;
    while i < entry.len() {
        if entry[i].is_numeric() {
            minecount = minecount * 10 + (entry[i] as i32 - '0' as i32); //there's probably a cleaner way to do this
        }
        else {
            println!("\x1b[38;5;13mfailed to read board size");
            return None;
        }
        i += 1;
    }

    let board = Board {
        width,
        height,
        minecount,
    };
    Some(board)
}

//safety(row, column, mines) -> (row, column)
//returns a location that doesn't contain a mine; only to be used at the start of a game
fn safety(mines: &Vec<i32>, minefield: &Vec<Vec<i32>>) -> (usize, usize) {
    let width = minefield[0].len();
    let height = minefield.len();

    let mut bag = Vec::new();
    for i in 0..(height * width) {
        bag.push(i);
    }

    let mut old_mines = Vec::new();
    for i in mines {
        old_mines.push(*i as usize);
    }
    old_mines.sort();
    old_mines.reverse();

    for i in old_mines {
        bag.remove(i);
        //we do this in reverse order because the bag's elements are the same as their indices to start with
        //and because that's faster than searching for each number in a 100+ element array
    }

    let m = bag[rand::rng().random_range(0..bag.len())];
    ((m / width), (m % width))
}

//dig(row, column, cells, mines, buffer) -> (cells_opened, game_state)
//digs an unopened cell, or digs all unopened cells around an opened cell with all flags placed
//returns the number of cells opened (0 or 1) and the state of the game (Playing or Lost)
fn dig(y: usize, x: usize, cells: &mut Vec<Vec<i32>>, mines: &Vec<Vec<i32>>, buffer: &mut Vec<[usize; 2]>) -> (i32, GameState) {
    if y >= cells.len() || x >= cells[y].len() {
        println!("\x1b[38;5;13mcell out of bounds");
        return (0, GameState::Playing);
    }

    if cells[y][x] == -1 {
        if mines[y][x] == 1 {
            cells[y][x] = 9;
            return (0, GameState::Lost);
        }
        else {
            //check surrounding cells and count mines
            let mut counter = 0;
            for i in [if y==0 {1000000} else {y-1}, y, y+1] {
                //this looks so stupid and would not be a problem if not for Rust's constant insistence on distinguishing between i32 and usize
                for j in [if x==0 {1000000} else {x-1}, x, x+1] {
                    if (i < mines.len()) && (j < mines[i].len()) && mines[i][j] == 1 {
                        counter += 1;
                    }
                }
            }
            cells[y][x] = counter;
            if counter == 0 {
                //zero chaining
                //this would be a very good place to use recursion, but i don't think i can because of borrowing rules
                //i think i can put a buffer instead though
                for i in [if y==0 {1000000} else {y-1}, y, y+1] {
                    for j in [if x==0 {1000000} else {x-1}, x, x+1] {
                        if (i < mines.len()) && (j < mines[i].len()) && cells[i][j] == -1 && !buffer.contains(&[i, j]) {
                            buffer.push([i, j]);
                        }
                    }
                }
            }
            return (1, GameState::Playing);
        }
    }
    else if cells[y][x] != -2 {
        //count flags
        let mut counter = 0;
        for i in [if y==0 {1000000} else {y-1}, y, y+1] {
            //this looks so stupid and would not be a problem if not for Rust's constant insistence on distinguishing between i32 and usize
            for j in [if x==0 {1000000} else {x-1}, x, x+1] {
                if (i < cells.len()) && (j < cells[i].len()) && cells[i][j] == -2 {
                    counter += 1;
                }
            }
        }
        //if equal to number, open unflagged cells
        if counter == cells[y][x] {
            for i in [if y==0 {1000000} else {y-1}, y, y+1] {
                //this looks so stupid and would not be a problem if not for Rust's constant insistence on distinguishing between i32 and usize
                for j in [if x==0 {1000000} else {x-1}, x, x+1] {
                    if (i < cells.len()) && (j < cells[i].len()) && cells[i][j] == -1 {
                        buffer.push([i, j]);
                    }
                }
            }
        }
        return(0, GameState::Playing);
    }
    (0, GameState::Playing)
}

//flag(row, column, playfield) -> flag_count
//places a flag on an unopened cell, or removes a flag from a cell
//returns the change to the number of flags placed
fn flag(y: usize, x: usize, cells: &mut Vec<Vec<i32>>) -> i32 {
    if y >= cells.len() || x >= cells[y].len() {
        println!("\x1b[38;5;13mcell out of bounds");
        return 0;
    }

    if cells[y][x] == -1 {
        cells[y][x] = -2;
        return 1;
    }
    else if cells[y][x] == -2 {
        cells[y][x] = -1;
        return -1;
    }
    0
}