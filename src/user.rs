use std::{thread::sleep, time};

use getch_rs::{Getch, Key};

#[derive(Clone)]
enum ExecMode {
    Run,
    ManualStep,
    AutoStep,
    Exit,
}

pub struct UserMode {
    exec_mode: ExecMode,
}

fn get_input() -> Option<ExecMode> {
    let k: Getch = Getch::new();
    match k.getch() {
        Ok(Key::Char(' ')) => Some(ExecMode::ManualStep),
        Ok(Key::Char('r')) => Some(ExecMode::AutoStep),
        Ok(Key::Char('q')) => Some(ExecMode::Exit),
        Ok(Key::Char('\r')) => Some(ExecMode::Run),
        Ok(_) => None,
        Err(e) => panic!("{}", e),
    }
}

impl UserMode {
    pub fn wait(&mut self) -> bool {
        loop {
            match self.exec_mode {
                ExecMode::Run => {
                    break;
                }
                ExecMode::ManualStep => match get_input() {
                    Some(new_exec_mode) => {
                        self.exec_mode = new_exec_mode;
                        break;
                    }
                    None => (),
                },
                ExecMode::AutoStep => {
                    sleep(time::Duration::from_millis(100));
                    // match get_input() {
                    //     Some(new_exec_mode) => {
                    //         self.exec_mode = new_exec_mode;
                    //     }
                    //     None => {}
                    // }
                    break;
                }
                ExecMode::Exit => {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn new() -> UserMode {
        return UserMode {
            exec_mode: ExecMode::ManualStep,
        };
    }
}
