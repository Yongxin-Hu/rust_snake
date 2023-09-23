use std::{io, thread};
use std::io::Write;
use std::time::Duration;
use crossterm::event::{read, Event, KeyEventKind, KeyCode, poll};
use crossterm::{ExecutableCommand, terminal};
use rand::distributions::uniform::SampleRange;
use rand::prelude::*;

const WIDTH:u32 = 40;
const HEIGHT:u32 = 25;


#[derive(PartialEq, Clone)]
pub struct Point(u32, u32);

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

struct Snake {
    direction: Direction,
    head: Point,
    body: Vec<Point>
}

enum GameState{
    Running,
    Lose,
    Win
}

struct Game<'a> {
    snake: &'a mut Snake,
    seed: Option<Point>,
    score: u32,
    state: GameState,
    height: u32,
    width: u32
}

impl<'a> Game<'a> {
    fn init(snake: &'a mut Snake) -> Self{
        Game {
            snake,
            seed: Some(Point(HEIGHT/2, WIDTH/2)),
            score: 0,
            state: GameState::Running,
            height: HEIGHT,
            width: WIDTH
        }
    }

    fn handle_event(&mut self) -> std::io::Result<()>{
        if poll(Duration::from_millis(500))? {
            match read()? {
                Event::Key(event) if event.kind == KeyEventKind::Press => {
                    match event.code {
                        KeyCode::Up | KeyCode::Char('w') => self.snake.direction = Direction::UP,
                        KeyCode::Down | KeyCode::Char('s') => self.snake.direction = Direction::DOWN,
                        KeyCode::Left | KeyCode::Char('a') => self.snake.direction = Direction::LEFT,
                        KeyCode::Right | KeyCode::Char('d') => self.snake.direction = Direction::RIGHT,
                        _ => ()
                    }
                },
                _ => ()
            }
        }
        Ok(())
    }

    fn update(&mut self) -> io::Result<()> {

        for body in self.snake.body.iter() {
            if self.snake.head == *body {
                self.state = GameState::Lose;
                return Ok(());
            }
        }

        if self.snake.head == self.seed.clone().unwrap() {
            self.score += 1;
            self.snake.body.insert(0, self.snake.head.clone());
            //todo
            self.seed = self.gen_seed();
        }
        match self.snake.direction {
            Direction::UP => {
                if self.snake.head.1 > 0 {
                    // body更新
                    self.snake.body.pop();
                    self.snake.body.insert(0, self.snake.head.clone());
                    // snake头坐标更新
                    self.snake.head.1 -= 1;
                    self.state = GameState::Running;
                    return Ok(());
                }else {
                    self.state = GameState::Lose;
                    return Ok(());
                }
            }
            Direction::DOWN => {
                if self.snake.head.1 < self.height-1 {
                    // body更新
                    self.snake.body.pop();
                    self.snake.body.insert(0, self.snake.head.clone());
                    // snake头坐标更新
                    self.snake.head.1 += 1;
                    self.state = GameState::Running;
                    return Ok(());
                } else {
                    self.state = GameState::Lose;
                    return Ok(());
                }
            }
            Direction::LEFT => {
                if self.snake.head.0 > 0 {
                    // body更新
                    self.snake.body.pop();
                    self.snake.body.insert(0, self.snake.head.clone());
                    // snake头坐标更新
                    self.snake.head.0 -= 1;
                    self.state = GameState::Running;
                    return Ok(());
                }else {
                    self.state = GameState::Lose;
                    return Ok(());
                }
            }
            Direction::RIGHT => {
                if self.snake.head.0 <= self.width-1 {
                    // body更新
                    self.snake.body.pop();
                    self.snake.body.insert(0, self.snake.head.clone());
                    // snake头坐标更新
                    self.snake.head.0 += 1;
                    self.state = GameState::Running;
                    return Ok(());
                }else {
                    self.state = GameState::Lose;
                    return Ok(());
                }
            }
        }
    }

    fn draw(&self) -> std::io::Result<()>{
        let mut stdout = io::stdout();
        for y in 0..self.height {
            let mut line = String::new();
            for x in 0..self.width {
                if self.snake.head == Point(x, y) {
                    match self.snake.direction {
                        Direction::UP => line.push('^'),
                        Direction::LEFT => line.push('<'),
                        Direction::RIGHT => line.push('>'),
                        Direction::DOWN => line.push('v'),
                    }
                }
                else if self.seed == Some(Point(x, y)) {
                    line.push('#');
                }
                else if self.snake.body.contains(&Point(x, y)) {
                    line.push('·');
                } else {
                    line.push(' ');
                }
            }
            stdout.write(line.as_bytes())?;
            stdout.write("\n".as_bytes())?;
            stdout.flush()?;
        }
        Ok(())
    }

    fn clear_screen(&self) -> io::Result<()> {
        let mut stdout = io::stdout();
        stdout.write("\x1b[2J\x1B[1;1H".as_bytes())?;
        stdout.flush()?;
        Ok(())
    }

    fn gen_seed(&self) -> Option<Point> {
        // todo maybe bug
        if (self.snake.body.len()+1) == ((self.height * self.width) as usize) {
            return None;
        }

        let mut rng = rand::thread_rng();
        let mut x:u32 = rng.gen_range(0..self.width);
        let mut y:u32 = rng.gen_range(0..self.height);
        let mut point = Point(x, y);
        while self.snake.head == point || self.snake.body.contains(&point) {
            x = rng.gen_range(0..self.width);
            y = rng.gen_range(0..self.height);
            point = Point(x, y);
        }
        Some(point)
    }
}

fn main() -> std::io::Result<()>{
    let mut snake = Snake{
        direction: Direction::RIGHT,
        head: Point(0, 0),
        body: vec![]
    };
    let mut game = Game::init(&mut snake);

    loop {
        game.handle_event()?;
        game.update()?;
        match game.state {
            GameState::Running => (),
            GameState::Lose => {
                print!("Game lose! score: {}", game.score);
                break;
            }
            GameState::Win => {
                print!("Congratulation, your score is {}", game.score);
            }
        };
        game.clear_screen()?;
        game.draw()?;
    }
    Ok(())
}
