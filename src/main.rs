const BOARD_HEIGHT: usize = 3;
const BOARD_WIDTH: usize = 3;

struct Board<'a> {
    board: [[&'a Token; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Board<'static> {
    fn set_token(&mut self, point: Point<usize>, token: &'static Token) {
        let x = (BOARD_WIDTH - 1) - point.y;
        let y = point.x;
        self.board[x][y] = token;
    }

    fn print_board(&self) {
        for row in self.board.iter() {
            for cell in row {
                print!("|{}", cell.value());
            }
            print!("|\n");
        }
    }

    fn is_full(&self) -> bool {

        for row in self.board.iter() {
            for cell in row {
                if *cell == &Token::None {
                    return false;
                }
            }
        }
        return true;
    }

    fn is_winner(&self, token: &Token) -> bool {
        self.check_rows(token) || self.check_columns(token) || self.check_diagonals(token)
    }

    fn check_rows(&self, token: &Token) -> bool {
        'outer: for row in self.board.iter() {
            'inner: for cell in row {
                if *cell != token {
                    continue 'outer;
                }
            }
            println!("3 in a row");
            return true;
        }
        return false;
    }

    fn check_columns(&self, token: &Token) -> bool {
        'outer: for i in 0..2 {
            'inner: for j in 0..2 {
                if self.board[j][i] != token {
                    continue 'outer;
                }
            }
            println!("3 in a column");
            return true;
        }
        return false;
    }

    fn check_diagonals(&self, token: &Token) -> bool {
        if self.board[1][1] != token {
            return false
        }
        let descending_diagonal = self.board[0][BOARD_HEIGHT - 1] == token && self.board[BOARD_WIDTH - 1][0] == token;
        let ascending_diagonal = self.board[0][0] == token && self.board[BOARD_WIDTH - 1][BOARD_HEIGHT - 1] == token;

        descending_diagonal || ascending_diagonal
    }
}

#[derive(PartialEq)]
enum Token {
    X,
    O,
    None,
}

impl Token {
    fn value(&self) -> &str {
        match self {
            Token::X => "X",
            Token::O => "O",
            Token::None => "-",
        }
    }
}

struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let mut b = Board { board: [[&Token::None; BOARD_HEIGHT ]; BOARD_WIDTH] };
    println!("Tic Tac Toe!\n");
    let mut turn = 0;

    while !b.is_full() {
        let token = match turn % 2 == 0 {
            true  => &Token::X,
            false => &Token::O,
        };
        b.print_board();
        println!("Choose your space, ({})", token.value());
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Unable to read line.");
        let coordinates: Vec<&str> = choice.split(",").collect();
        let point = Point {
            x: coordinates[0].trim().parse().expect("failed to parse X coordinate"),
            y: coordinates[1].trim().parse().expect("failed to parse Y coordinate"),
        };

        b.set_token(point, token);
        if b.is_winner(token){
            println!("{} wins!", token.value());
            return;
        }
        turn = turn + 1;
    }
    println!("Cats!");
}

#[test]
fn test_rows_finds_tokens() {
    let mut board = Board {board: [
        [&Token::None; 3]; 3]
    };
    board.set_token(Point{x: 0, y: 0}, &Token::X);
    board.set_token(Point{x: 1, y: 0}, &Token::X);
    board.set_token(Point{x: 2, y: 0}, &Token::X);
    assert!(board.check_rows(&Token::X));
}

#[test]
fn test_rows_less_than_three_is_false() {
    let board = Board {board: [[&Token::None; 3]; 3]};
    assert!(!board.check_rows(&Token::O));
}

#[test]
fn test_column_winner_is_true() {
    let mut board = Board {board: [[&Token::None; 3]; 3]};
    board.set_token(Point {x: 0, y: 0}, &Token::X);
    board.set_token(Point {x: 0, y: 1}, &Token::X);
    board.set_token(Point {x: 0, y: 2}, &Token::X);
    assert!(board.check_columns(&Token::X));
}

#[test]
fn column_check_is_false_when_not_column_win() {
    let mut board = Board {board: [[&Token::None; 3]; 3]};
    board.set_token(Point {x: 0, y: 0}, &Token::X);
    board.set_token(Point {x: 0, y: 1}, &Token::X);
    board.set_token(Point {x: 0, y: 2}, &Token::O);
    assert!(!board.check_columns(&Token::X));
}

#[test]
fn diagonal_check_is_true_when_win() {
    let mut board = Board {board: [[&Token::None; 3]; 3]};
    board.set_token(Point {x: 0, y: 0}, &Token::X);
    board.set_token(Point {x: 1, y: 1}, &Token::X);
    board.set_token(Point {x: 2, y: 2}, &Token::X);
    assert!(board.check_diagonals(&Token::X));

    let mut board2 = Board {board: [[&Token::None; 3]; 3]};
    board2.set_token(Point {x: 0, y: 2}, &Token::X);
    board2.set_token(Point {x: 1, y: 1}, &Token::X);
    board2.set_token(Point {x: 2, y: 0}, &Token::X);
    assert!(board2.check_diagonals(&Token::X));
}

#[test]
fn diagonal_check_is_false_when_not_win() {
    let mut board = Board {board: [[&Token::None; 3]; 3]};
    board.set_token(Point {x: 0, y: 0}, &Token::X);
    board.set_token(Point {x: 0, y: 1}, &Token::X);
    board.set_token(Point {x: 0, y: 2}, &Token::O);
    assert!(!board.check_columns(&Token::X));
}
