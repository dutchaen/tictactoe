use rand::{thread_rng, Rng};
use read_input::shortcut::simple_input;
use std::{collections::HashMap, str::FromStr};

const WINNER_COORDINATES: &[[[usize; 2]; 3]; 8] = &[
    [[0, 0], [0, 1], [0, 2]],
    [[1, 0], [1, 1], [1, 2]],
    [[2, 0], [2, 1], [2, 2]],

    [[0, 0], [1, 0], [2, 0]],
    [[0, 1], [1, 1], [2, 1]],
    [[0, 2], [1, 2], [2, 2]],

    [[0, 0], [1, 1], [2, 2]],
    [[2, 0], [1, 1], [0, 2]]
];

struct Point {
    x: u8,
    y: u8,
}

impl Point {
    fn new(x: u8, y: u8) -> Result<Point, Box<dyn std::error::Error>> {
        if x > 2 || y > 2 {
            return Err("OUT OF BOUNDS".into());
        }
        return Ok(Point { x, y })
    }

    fn random() -> Point {
        let mut rng = thread_rng();
        let x: u8 = rng.gen_range(0..2);
        let y: u8 = rng.gen_range(0..2);
        return Point {
            x,
            y
        };
    }
}

#[repr(u8)]
#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
enum Player {
    X = b'X',
    Y = b'Y',
    NA = b' '
}

impl FromStr for Player {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Player::X),
            "Y" => Ok(Player::Y),
            _ => Ok(Player::NA)  
        }
    }
}

fn main() {
    let mut game_board = [
        [' ', ' ', ' '],
        [' ', ' ', ' '],
        [' ', ' ', ' '],
    ];


    loop {
        print_game_board(&game_board);

        eprint!("x axis: ");
        let x = simple_input::<u8>();

        eprint!("y axis: ");
        let y = simple_input::<u8>();

        if let Ok(point) = Point::new(x, y) {
            play_turn(&mut game_board, &point, Player::X)
                .expect("valid turn");
        }

        if let Some(winner) = find_winner(&game_board) {
            print_game_board(&game_board);
            println!("{:?} has won the game :) ", winner);
            std::process::exit(0);
        } 

        let mut random_point = Point::random();
        while let Err(_) = play_turn(&mut game_board, &random_point, Player::Y) {
            random_point = Point::random();
        }

        if let Some(winner) = find_winner(&game_board) {
            print_game_board(&game_board);
            println!("{:?} has won the game :) ", winner);
            std::process::exit(0);
        }
    }
}

fn print_game_board(board: &[[char; 3]; 3]) {
    clearscreen::clear().expect("cleared console");
    for y in 0..board.len() {
        let level = &board[y];
        if y == 0 {
            println!("-------------------");
        }

        println!("|  {}  |  {}  |  {}  |", level[0], level[1], level[2]);

        if y != board.len() - 1 {
            println!("-------------------");
        }
    }
    println!("-------------------");
}

fn play_turn(board: &mut [[char; 3]; 3], point: &Point, player: Player) -> Result<(), Box<dyn std::error::Error>> {
    if board[point.y as usize][point.x as usize] != ' ' {
        return Err("Invalid turn".into());
    }
    board[point.y as usize][point.x as usize] = player as u8 as char;

    return Ok(());
}

fn find_winner(board: &[[char;3]; 3]) -> Option<Player> {
    for coords in WINNER_COORDINATES {
        let mut map: HashMap<Player, i32> = HashMap::new();
        for coord in coords {
            let player = board[coord[0]][coord[1]]
                .to_string()
                .parse::<Player>()
                .unwrap();

            map.entry(player)
                .and_modify(|item| {*item += 1} )
                .or_insert(1); 
        }

        if let Some(winner) = map.iter().find(|kv| *kv.1 == 3) {
            let winning_player = *winner.0;
            if winning_player != Player::NA {
                return Some(winning_player);
            }
        }
    }
    return None;
}