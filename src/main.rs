use rand::distributions::{Distribution, Uniform};
extern crate structopt;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short, long)]
    verbose: bool,
    #[structopt(short, long)]
    next: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Entity {
    O,
    X,
    Empty,
}
use Entity::*;

impl Entity {
    fn from_char(c: char) -> Self {
        match c {
            'o' | 'O' => O,
            'x' | 'X' => X,
            _ => Empty,
        }
    }
    fn into_char(self) -> char {
        match self {
            O => 'o',
            X => 'x',
            _ => '.',
        }
    }
}

impl std::ops::Neg for Entity {
    type Output = Self;
    fn neg(self) -> Self {
        match self {
            O => X,
            X => O,
            _ => Empty,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Move {
    col: usize,
}

impl Move {
    fn new(col: usize) -> Self {
        Self { col }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Game {
    next: Entity,
    height: usize,
    width: usize,
    data: Vec<Vec<Entity>>,
}

impl Game {
    fn read(next: Entity) -> Self {
        let stdin = std::io::stdin();
        let mut buffer: Vec<Vec<char>> = vec![];
        loop {
            let mut line = String::new();
            match stdin.read_line(&mut line) {
                Ok(x) if x > 0 => {
                    buffer.push(line.trim().chars().collect());
                }
                _ => break,
            }
        }
        let height = buffer.len();
        let width = buffer[0].len();
        assert!(height >= 4 && width >= 4);
        let data: Vec<Vec<_>> = buffer
            .iter()
            .map(|line| line.iter().cloned().map(Entity::from_char).collect())
            .collect();
        Self {
            next,
            height,
            width,
            data,
        }
    }

    fn write(&self) {
        for line in self.data.iter() {
            for &e in line.iter() {
                print!("{}", e.into_char());
            }
            println!();
        }
    }

    fn play_mut(&mut self, mv: Move) -> Result<(), ()> {
        for i in (0..self.height).rev() {
            if self.data[i][mv.col] == Empty {
                self.data[i][mv.col] = self.next;
                self.next = -self.next;
                return Ok(());
            }
        }
        Err(())
    }

    fn play(&self, mv: Move) -> Result<Self, ()> {
        let mut g = self.clone();
        match g.play_mut(mv) {
            Ok(_) => Ok(g),
            _ => Err(()),
        }
    }

    /// 勝敗判定
    /// ついてないときは Entity::Empty を返す
    fn judge(&self) -> Entity {
        // row ->
        for i in 0..self.height {
            let mut num_o = 0;
            let mut num_x = 0;
            for j in 0..4 {
                match self.data[i][j] {
                    O => num_o += 1,
                    X => num_x += 1,
                    _ => {}
                }
            }
            if num_o == 4 {
                return O;
            }
            if num_x == 4 {
                return X;
            }
            for j in 4..self.width {
                match self.data[i][j - 4] {
                    O => num_o -= 1,
                    X => num_x -= 1,
                    _ => {}
                }
                match self.data[i][j] {
                    O => num_o += 1,
                    X => num_x += 1,
                    _ => {}
                }
                if num_o == 4 {
                    return O;
                }
                if num_x == 4 {
                    return X;
                }
            }
        }
        // col v
        for j in 0..self.width {
            let mut num_o = 0;
            let mut num_x = 0;
            for i in 0..4 {
                match self.data[i][j] {
                    O => num_o += 1,
                    X => num_x += 1,
                    _ => {}
                }
            }
            if num_o == 4 {
                return O;
            }
            if num_x == 4 {
                return X;
            }
            for i in 4..self.height {
                match self.data[i - 4][j] {
                    O => num_o -= 1,
                    X => num_x -= 1,
                    _ => {}
                }
                match self.data[i][j] {
                    O => num_o += 1,
                    X => num_x += 1,
                    _ => {}
                }
                if num_o == 4 {
                    return O;
                }
                if num_x == 4 {
                    return X;
                }
            }
        }
        // cross line (left-up to right-bottom)
        for i in 0..self.height - 3 {
            for j in 0..self.width - 3 {
                let line: Vec<Entity> = (0..4).map(|k| self.data[i + k][j + k]).collect();
                if line.iter().all(|&item| item == O) {
                    return O;
                }
                if line.iter().all(|&item| item == X) {
                    return X;
                }
            }
        }
        // cross line (right-up to left-bottom)
        for i in 0..self.height - 3 {
            for j in 3..self.width {
                let line: Vec<Entity> = (0..4).map(|k| self.data[i + k][j - k]).collect();
                if line.iter().all(|&item| item == O) {
                    return O;
                }
                if line.iter().all(|&item| item == X) {
                    return X;
                }
            }
        }
        // otherwise
        Entity::Empty
    }
}

struct Solver {
    verbose: bool,
}

impl Solver {
    fn new(verbose: bool) -> Self {
        Self { verbose }
    }

    fn solve(&self, game: &Game) -> Result<Game, ()> {
        match game.judge() {
            O | X => {
                return Err(());
            }
            _ => {}
        }
        self.play_wise(&game)
    }

    fn choices(&self, game: &Game) -> Vec<Move> {
        (0..game.width)
            .map(Move::new)
            .filter(|mv| game.data[0][mv.col] == Empty)
            .collect()
    }

    fn random_play_mut(&self, game: &mut Game) -> Result<(), ()> {
        let mvs = self.choices(&game);
        if mvs.is_empty() {
            return Err(());
        }
        let mut rng = rand::thread_rng();
        let idx = Uniform::from(0..mvs.len()).sample(&mut rng);
        let mv = mvs[idx];
        game.play_mut(mv)
    }

    /// 決着までプレイして勝者を返す
    fn rollplay_random_mut(&self, mut game: &mut Game) -> Entity {
        loop {
            match game.judge() {
                O => return O,
                X => return X,
                _ => {}
            }
            match self.random_play_mut(&mut game) {
                Ok(_) => continue,
                Err(_) => break,
            }
        }
        Empty
    }

    /// ランダムにプレイして勝つ確率を推定する
    fn estimate_prob_win(&self, game: &Game, num_try: usize) -> f32 {
        let mut win = 0;
        for _ in 0..num_try {
            let mut g = game.clone();
            if game.next == self.rollplay_random_mut(&mut g) {
                win += 1;
            }
        }
        win as f32 / num_try as f32
    }

    fn play_wise(&self, game: &Game) -> Result<Game, ()> {
        let mut goodgame = None; // good to next
        let mut maxp = 0.0;
        for &mv in self.choices(&game).iter() {
            let g = game.play(mv)?; // the game after next
            if g.judge() == game.next {
                if self.verbose {
                    println!("---");
                    g.write();
                    println!("You can win soon!");
                    println!();
                }
                return Ok(g);
            }
            let mut minp = 1.0;
            for &mv in self.choices(&g).iter() {
                let h = g.play(mv)?; // the game after prev (or enemy)
                let p = self.estimate_prob_win(&h, 200); // prob for the next win

                if self.verbose {
                    println!("---");
                    h.write();
                    println!("Prob to win: {:.3}", p);
                    println!();
                }

                if minp > p {
                    minp = p;
                }
            }
            if maxp < minp {
                maxp = minp;
                goodgame = Some(g);
            }
        }
        if let Some(g) = goodgame {
            Ok(g)
        } else {
            Err(())
        }
    }
}

fn main() {
    let opt = Opt::from_args();
    let game = Game::read(Entity::from_char(opt.next.chars().next().unwrap()));
    let solver = Solver::new(opt.verbose);

    if let Ok(goodgame) = solver.solve(&game) {
        goodgame.write();
    } else {
        println!("No choice");
    }
}
