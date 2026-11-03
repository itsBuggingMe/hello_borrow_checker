use rand::RngExt;
use std::{thread, time::Duration};
use std::io::{self, Read, Write};
use crossterm::event::{read, Event, KeyCode, poll};

fn main() {
    run_snake()
}

fn run_snake() {
    const MAP_WIDTH: usize = 16;
    const MAP_HEIGHT: usize = 16;

    let rail = "██".repeat(MAP_HEIGHT + 2);
    let mut map = [TileKind::Empty; MAP_WIDTH * MAP_HEIGHT];
    let mut snake_length: usize = 2;
    let mut rng = rand::rng();
    let mut need_spawn_apple = true;
    let mut snake_dir: (i32, i32) = (1, 0);
    let mut head_pos: (i32, i32) = ((MAP_WIDTH / 2) as i32 - 1, (MAP_HEIGHT / 2) as i32);
    let mut snake_dead = false;

    print!("\x1B[2J\x1B[1;1H");

    while snake_length < MAP_HEIGHT * MAP_WIDTH {
        while need_spawn_apple {
            let pos = rng.random_range(0..MAP_HEIGHT * MAP_WIDTH);
            if !matches!(map[pos], TileKind::Snake { age: _ }) {
                need_spawn_apple = false;
                map[pos] = TileKind::Apple;
            }
        }

        head_pos = (head_pos.0 + snake_dir.0,
                    head_pos.1 + snake_dir.1);
        let head_index = (head_pos.0 + head_pos.1 * MAP_WIDTH as i32) as usize;

        if(head_pos.0 < 0 || head_pos.0 >= MAP_WIDTH as i32
            || head_pos.1 < 0 || head_pos.1 >= MAP_HEIGHT as i32) {
            snake_dead = true;
            break;
        }

        if matches!(map[head_index], TileKind::Snake { .. }) {
            snake_dead = true;
            break;
        }

        if map[head_index] == TileKind::Apple {
            snake_length += 1;
            need_spawn_apple = true;
        } else {
            for i in 0..MAP_HEIGHT {
                for j in 0..MAP_WIDTH {
                    let coord = j + i * MAP_WIDTH;
                    if let TileKind::Snake { age: a } = map[coord]  {
                        map[coord] = if a == 1 { TileKind::Empty }
                        else { TileKind::Snake { age: a - 1 }};
                    }
                }
            }
        }

        map[head_index] = TileKind::Snake { age: snake_length };

        print!("\x1B[H");
        io::stdout().flush().unwrap();

        println!("{}", rail);
        for i in 0..MAP_HEIGHT {
            print!("██");
            for j in 0..MAP_WIDTH {
                let coord = j + i * MAP_WIDTH;
                print!("{}", map[coord].get_ansi_output(snake_length));
            }
            println!("██");
        }
        println!("{}", rail);

        thread::sleep(Duration::from_millis(200));

        let mut new_dir = snake_dir;
        while poll(Duration::ZERO).unwrap() {
            if let Event::Key(key) = read().unwrap() {
                new_dir = match key.code {
                    KeyCode::Char('w') if snake_dir != (0, 1) => (0, -1),
                    KeyCode::Char('s') if snake_dir != (0, -1) => (0, 1),
                    KeyCode::Char('a') if snake_dir != (1, 0) => (-1, 0),
                    KeyCode::Char('d') if snake_dir != (-1, 0) => (1, 0),
                    _ => new_dir
                };
            }
        }
        snake_dir = new_dir;
    }

    if snake_dead {
        println!("You die from a brain hemorrhage.");
    } else {
        println!("What have you done. Now that you have eaten all of the apples, you have brought forth God's wrath.");
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum TileKind {
    Snake { age: usize },
    Empty,
    Apple,
}

impl TileKind {
    fn get_ansi_output(self: TileKind, snake_len: usize) -> String {
        match self {
            TileKind::Snake { age: a } =>
                format!("\x1b[38;2;59;{};28m██\x1b[0m", ((snake_len - a) * 32) % 96 + 100),
            TileKind::Empty => String::from("\x1b[39m  \x1b[0m"),
            TileKind::Apple => String::from("\x1b[31m██\x1b[0m"),
        }
    }
}