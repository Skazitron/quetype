use termion::raw::RawTerminal;
use windows::main_menu::MainMenuScreen;
use windows::startup_screen::StartupScreen;
use std::io;
use std::io::Stdout;
use termion::screen::ToAlternateScreen;

use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    Terminal,
};

// loaded modules
pub mod events;
use events::{Event, Events};


type TermType = Terminal<TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<Stdout>>>>>;


pub mod windows;

pub trait ScreenItem {
    fn display(&self, terminal: &mut TermType);
}

pub struct StackBox 
{
    stack: Vec<AllWindows>
}


impl StackBox {
    fn new() -> StackBox {
        StackBox {
            stack: Vec::with_capacity(3)
        }
    }

    fn push(&mut self, param: AllWindows) {
        self.stack.push(param);
    }

    fn peek(&self) -> Option<&AllWindows> {
        self.stack.last()
    }

    fn pop(&mut self) -> Option<AllWindows> {
        self.stack.pop()
    }
}




enum AllWindows {
    Startup(StartupScreen),
    MainMenu(MainMenuScreen) 
}

impl AllWindows {
    pub fn display(&self, terminal: &mut TermType) {
        match self {
            
            AllWindows::Startup(bee) => {
                bee.display(terminal);
            }

            AllWindows::MainMenu(bee) => {
                bee.display(terminal);
            }

        }
    }
}





/// the run function has all the important bits!!
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;


    
    

    println!("{}", ToAlternateScreen);

    
    let events = Events::new();


    

    let startup = AllWindows::Startup(StartupScreen);
    let mut stackbox = StackBox::new();
    stackbox.push(startup);



    loop {

        if let Some(value) = stackbox.peek() {
            value.display(&mut terminal);
        }



        if let Event::Input(input) = events.next()? {
            if let Key::Char('q') = input {
                break;
            } else if let Key::Char('m') = input {
                stackbox.push(AllWindows::MainMenu(MainMenuScreen));
            } else if let Key::Char('b') = input {
                if stackbox.stack.len() > 1 {
                    stackbox.pop();
                }
            }
        }
    }
    
    
    Ok(())
        
}
