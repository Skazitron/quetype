use tui::backend::Backend;
use termion::raw::IntoRawMode;
use std::io;
use termion::event::{ Key, Event };
use termion::screen::{ AlternateScreen, ToAlternateScreen };

// loaded modules
pub mod events;
use events::EventListener;

pub trait ScreenItem<B> 
    where B: Backend
{
   fn display(backend: B);
}

/// the run function has all the important bits!!
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    

    let stdout = io::stdout().into_raw_mode()?;
    let mut stdout = AlternateScreen::from(stdout);
    println!("{}", ToAlternateScreen);
    
    let receiver = EventListener::event_stream();
    
    
    loop {
        
        let event = receiver.recv().unwrap();

        match event {
           Event::Key(key) => {
               match key {
                   Key::Char(c) => {
                       match c {
                           'q' => {
                               break;
                           }

                           _ => {
                               println!("{}", c);
                           }

                       }
                   }

                   _ => {

                   }
               }
           }

           _ => {}
                
        }

    }
    
    
    Ok(())
        
}
