use std::sync::mpsc::{SendError, Sender};

#[derive(PartialEq, Eq, Clone)]
pub enum Input {
    Start,
    Pause,
    Next,
    Quit,
    Bs,
}

pub fn start(sdr: Sender<Input>) -> Result<(), SendError<Input>> {
    let mut input: Input;
    let c_term = console::Term::stdout();

    loop {
        input = match c_term.read_key().expect("somethings wrong") {
            console::Key::Char('q') => Input::Quit,
            console::Key::Char('n') | console::Key::ArrowLeft => Input::Next,
            console::Key::Char(' ') => Input::Pause,
            console::Key::Enter => Input::Start,
            _ => Input::Bs,
        };

        if let Input::Quit = input {
            sdr.send(input)?;
            break;
        }
        sdr.send(input)?;
    }
    Ok(())
}
