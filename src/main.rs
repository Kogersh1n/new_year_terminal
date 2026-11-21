use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute, queue,
    style::Print,
    terminal,
};
use rand::Rng;
use std::io::{Write, stdout};
use std::time::Duration;

struct Flake {
    x: f32,
    y: f32,
    speed: f32,
}

fn main() -> std::io::Result<()> {
    let mut out = stdout();
    terminal::enable_raw_mode()?;
    execute!(out, terminal::EnterAlternateScreen, cursor::Hide)?;
    let mut rng = rand::thread_rng();
    let mut flakes: Vec<Flake> = Vec::new();

    loop {
        if event::poll(Duration::from_millis(17))? {
            if let Event::Key(k) = event::read()? {
                if k.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        let (w, h) = terminal::size()?;
        if rng.gen_bool(0.5) {
            flakes.push(Flake {
                x: rng.gen_range(0.0..w as f32),
                y: 0.0,
                speed: rng.gen_range(0.2..0.6),
            });
        }
        for f in &mut flakes {
            f.y += f.speed;
        }
        flakes.retain(|f| f.y < h as f32);

        queue!(out, terminal::Clear(terminal::ClearType::All))?;
        for f in &flakes {
            queue!(out, cursor::MoveTo(f.x as u16, f.y as u16), Print('*'))?;
        }
        out.flush()?;
    }

    execute!(out, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()
}
