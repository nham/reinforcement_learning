use std::rand;
use std::rand::distributions::{IndependentSample, Range};
use std::hashmap::HashMap;
use std::fmt::{Show, Formatter};
use std::fmt;

#[deriving(Eq)]
enum Cell {
    Nil,
    X,
    O,
}

impl Show for Cell {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let s = match *self {
                    Nil => " ",
                    X   => "X",
                    O   => "O",
        };

        write!(f.buf, "{:s}", s)
    }
}

impl Show for [Cell, ..3] {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f.buf, "|{} {} {}|", self[0], self[1], self[2])
    }
}

enum PlayerType {
    PX,
    PO,
}

type GameState = [[Cell, ..3], ..3];

struct Game {
    state: GameState,
}

impl Game {
    fn new() -> Game {
        Game { state: [[Nil, Nil, Nil], [Nil, Nil, Nil], [Nil, Nil, Nil]] }
    }

    fn get(&self, x: uint, y: uint) -> Result<Cell, &str> {
        if !(x >= 1 && x <= 3 && y >= 1 && y <= 3 ) {
            Err("Out of range")
        } else {
            Ok( self.state[x-1][y-1] )
        }
    }

    fn cell_is_avail(&self, x: uint, y: uint) -> bool {
        let res = self.get(x,y);
        match self.get(x,y) {
            Ok(Nil) => true,
            _       => false,
        }
    }

    fn set(&mut self, x: uint, y: uint, c: Cell) -> bool {
        if !(x >= 1 && x <= 3 && y >= 1 && y <= 3 ) {
            false
        } else {
            self.state[x-1][y-1] = c;
            true
        }
    }

    fn print(&self) {
        println!("{}\n{}\n{}", self.state[0], self.state[1], self.state[2]);
    }

    fn is_won(&self) -> bool {
        for i in range(1u, 4u) {
            if self.get(i, 1u) == self.get(i, 2u)
            && self.get(i, 2u) == self.get(i, 3u)
            && self.get(i, 1u) == self.get(i, 3u) {
                match self.get(i, 1u) {
                    Ok(X) => return true,
                    Ok(O) => return true,
                    _ => (),
                }
            }
        }

        for i in range(1u, 4u) {
            if self.get(1u, i) == self.get(2u, i)
            && self.get(2u, i) == self.get(3u, i)
            && self.get(1u, i) == self.get(3u, i) {
                match self.get(1u, i) {
                    Ok(X) => return true,
                    Ok(O) => return true,
                    _ => (),
                }
            }
        }

        if self.get(1u, 1u) == self.get(2u, 2u)
        && self.get(2u, 2u) == self.get(3u, 3u) {
            match self.get(1u, 1u) {
                Ok(X) => return true,
                Ok(O) => return true,
                _ => (),
            }
        }

        if self.get(1u, 3u) == self.get(2u, 2u)
        && self.get(2u, 2u) == self.get(3u, 1u) {
            match self.get(1u, 3u) {
                Ok(X) => return true,
                Ok(O) => return true,
                _ => (),
            }
        }

        false
    }

    fn is_over(&self) -> bool {
        if self.is_won() {
            return true;
        }

        for i in range(1u, 4u) {
            for j in range(1u, 4u) {
                if self.cell_is_avail(i, j) {
                    return false;
                }
            }

        }

        true
    }
}

trait Player {
    fn ptype(&self) -> PlayerType;
    fn move(&self, game: &mut Game);
    fn make_move(&self, game: &mut Game, x: uint, y: uint) {
        fn convert(ptype: PlayerType) -> Cell {
            match ptype {
                PX => X,
                _  => O,
            }
        }

        game.set(x, y, convert(self.ptype()));
    }

}

struct RandomPlayer {
    ptype: PlayerType,
}

impl Player for RandomPlayer {
    fn ptype(&self) -> PlayerType{
        self.ptype
    }

    fn move(&self, game: &mut Game) {
        let between = Range::new(0u, 9u);
        let mut rng = rand::task_rng();

        fn convert(x: uint) -> (uint, uint) {
            (x / 3 + 1, x % 3 + 1)
        }

        let (mut x, mut y) = convert(between.ind_sample(&mut rng));
        while !game.cell_is_avail(x, y) {
            let (newx, newy) = convert(between.ind_sample(&mut rng));
            x = newx;
            y = newy;
        }

        self.make_move(game, x, y);
    }
}

fn main() {
    let values = HashMap::<~str, f64>::new();
    let mut g = Game::new();
    let rands: [RandomPlayer, ..2] = [RandomPlayer { ptype: PX },
                                      RandomPlayer { ptype: PO }];

    g.print();
    println!("----------");
    let mut i = 0;
    while !g.is_over() {
        rands[i % 2].move(&mut g);
        g.print();
        println!("----------");
        i += 1;
    }

}
