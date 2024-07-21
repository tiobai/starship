use crossterm::cursor::{Hide, MoveTo};
use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::terminal::{self, Clear, ClearType};
use crossterm::{execute, QueueableCommand};
use std::io::{stdout, Write};
use std::sync::mpsc;
//use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
mod frames;

fn main() {
    let mut stdout = stdout();
    execute!(stdout, Hide).unwrap();
    let (mut w, mut h) = terminal::size().unwrap();
    let (mut x, mut y) = (0, h);
    let (xe, ye) = (w / 2, h / h);
    //let semaphore = Arc::new(Mutex::new(1));
    let (ex, rx) = mpsc::channel();
    //let (ey, ry) = mpsc::channel();
    //let clone_semaphore = semaphore.clone();
    thread::spawn(move || loop {
        //let _guard = clone_semaphore.lock().unwrap();
        ex.send(xe + 1).unwrap();
        //ey.send(ye - 1).unwrap();
        thread::sleep(Duration::from_secs(1));
    });

    let fy = ye;
    loop {
        let fx = rx.recv().unwrap();
        while poll(Duration::ZERO).unwrap() {
            match read().unwrap() {
                Event::Resize(nw, nh) => {
                    w = nw;
                    h = nh;
                }
                Event::Key(event) => match event.code {
                    KeyCode::Char(code) => match code {
                        'a' => {
                            if x == 0 {
                                x = x;
                            } else {
                                x = x - 1;
                            }
                        }
                        'd' => {
                            if x == w {
                            } else {
                                x = x + 1;
                            }
                        }
                        'w' => {
                            if y == 0 {
                            } else {
                                y = y - 1;
                            }
                        }
                        's' => {
                            if y == h {
                            } else {
                                y = y + 1;
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            }
        }

        stdout.queue(Clear(ClearType::All)).unwrap();
        std::io::stdout().queue(MoveTo(fx, fy)).unwrap();
        std::io::stdout().write(frames::ENEMIES.as_bytes()).unwrap();

        stdout.queue(MoveTo(x, y)).unwrap();
        stdout.write(frames::STARSHIP.as_bytes()).unwrap();
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(25));
    }
}
