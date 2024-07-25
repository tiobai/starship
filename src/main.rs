use crossterm::cursor::{Hide, MoveTo};
use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::terminal::{self, Clear, ClearType};
use crossterm::{execute, QueueableCommand};
use std::io::{stdout, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
mod frames;
mod update_move;

fn main() {
    let mut stdout = stdout();
    execute!(stdout, Hide).unwrap();
    let length_starship = frames::LENGTH_STARSHIP;
    let length_enemie = frames::LENGTH_ENEMIES;
    let (mut w, mut h) = terminal::size().unwrap();
    let (mut x, mut y) = (0, h);
    let limit_y = h;
    let limit_x = w;
    let position_y = Arc::new(Mutex::new(0));
    let position_x = Arc::new(Mutex::new(0));
    let update_position_y = position_y.clone();
    let update_position_x = position_x.clone();
    thread::spawn(move || loop {
        update_move::update_counter_value(&update_position_y, &update_position_x, limit_y, limit_x);
    });
    loop {
        let value_y = position_y.lock().unwrap();
        let value_x = position_x.lock().unwrap();
        while poll(Duration::ZERO).unwrap() {
            match read().unwrap() {
                Event::Resize(nw, nh) => {
                    w = nw;
                    h = nh;
                }
                Event::Key(event) => match event.code {
                    KeyCode::Char(code) => match code {
                        'a' => x = if x != 0 { x - 1 } else { x },
                        'd' => x = if x != w - 5 { x + 1 } else { x },
                        'w' => y = if y != 0 { y - 1 } else { y },
                        's' => y = if y != h { y + 1 } else { y },
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            }
        }

        stdout.queue(Clear(ClearType::All)).unwrap();
        if ((x..=x + length_starship).contains(&value_x)
            || (*value_x..=*value_x + length_enemie).contains(&x))
            && y == *value_y
        {
            stdout.queue(MoveTo(w, h / 2)).unwrap();
            stdout.write(frames::GAME_OVER.as_bytes()).unwrap();
            break;
        }
        stdout.queue(MoveTo(*value_x, *value_y)).unwrap();
        stdout.write(frames::ENEMIES.as_bytes()).unwrap();
        stdout.queue(MoveTo(x, y)).unwrap();
        stdout.write(frames::STARSHIP.as_bytes()).unwrap();
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(20));
    }
}
