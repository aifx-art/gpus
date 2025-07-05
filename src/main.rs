mod render;
mod data;

use std::{thread, time::Duration, sync::atomic::{AtomicBool, Ordering}, sync::Arc};
use crossterm::{terminal, execute, event::{self, Event, KeyCode}};
use std::io::stdout;
use data::{get_all_gpu_data, GpuData};
use tui::backend::CrosstermBackend;
use tui::Terminal;
use render::draw_bars_tui;

fn main() {
    let mut stdout = stdout();
    terminal::enable_raw_mode().unwrap();
    execute!(stdout, terminal::EnterAlternateScreen).unwrap();
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    while running.load(Ordering::SeqCst) {
        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Char('q') => running.store(false, Ordering::SeqCst),
                    _ => {}
                }
            }
        }
        let gpu_data = get_all_gpu_data();
        draw_bars_tui(&mut terminal, &gpu_data).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
    execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen).unwrap();
    terminal::disable_raw_mode().unwrap();
}
