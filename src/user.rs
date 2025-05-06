use getch_rs::{Getch, Key};
pub struct UserState {
    pub stepping: bool,
}

impl UserState {
    pub fn wait(&mut self) {
        match self.stepping {
            true => {
                let k = Getch::new();
                loop {
                    match k.getch() {
                        Ok(Key::Char(' ')) => break,
                        Ok(Key::Char('r')) => {
                            self.stepping = false;
                            break;
                        }
                        Ok(_) => (),
                        Err(e) => eprintln!("{}", e),
                    }
                }
            }
            false => (),
        }
    }
}
