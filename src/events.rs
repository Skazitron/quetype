use std::sync::mpsc;
use std::thread;
use termion::input::TermRead;
use termion::event::{ Event, Key };

/// A very basic event emitter that works on a separate thread than 
/// the one used for the frontend
pub struct EventListener;

impl EventListener {
    pub fn event_stream() -> mpsc::Receiver<termion::event::Event> {
        let (sendx, recvx) = mpsc::channel();

        thread::spawn(move || {

            let stdin = std::io::stdin();


            for event in stdin.events() {
                
                let event = event.unwrap();

                sendx.send(event).unwrap();
            }
        });
        recvx
    }

}



