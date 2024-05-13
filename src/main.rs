use std::{
    env,
    io::{self, Write},
    process,
};

use ctrlc::set_handler;
use enigo::{
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Settings,
};

enum Zoom {
    In(usize),
    #[allow(dead_code)]
    Out(usize),
    Reset,
}

impl Zoom {
    fn zoom(self) {
        let mut enigo = Enigo::new(&Settings::default()).unwrap();

        enigo.key(Key::Control, Press).unwrap();
        match self {
            Zoom::In(x) | Zoom::Out(x) => {
                let key = if matches!(self, Zoom::In(_)) {
                    '='
                } else {
                    '-'
                };
                for _ in 0..x {
                    enigo.key(Key::Unicode(key), Click).unwrap();
                }
            }
            Zoom::Reset => enigo.key(Key::Unicode('0'), Click).unwrap(),
        }
        enigo.key(Key::Control, Release).unwrap();
    }
}

fn main() {
    set_handler(move || {
        Zoom::Reset.zoom();
        process::exit(0);
    })
    .unwrap();

    let zoom_level: usize = env::args()
        .nth(1)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(25);

    Zoom::In(zoom_level).zoom();

    let input = &mut String::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let _ = io::stdin().read_line(input);
    }
}
