use std::io;

use crossterm::event;

fn main() -> io::Result<()> {
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| {
                frame.render_widget("Hello World", frame.area());
            })?;

            if event::read()?.is_key_press() {
                break Ok(());
            }
        }
    })
}
