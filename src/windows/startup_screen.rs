use crate::{ScreenItem, TermType};
use tui::{
    layout::{Constraint, Direction, Layout, Alignment},
    widgets::{Block, Borders},
};

pub struct StartupScreen;



impl ScreenItem for StartupScreen {
    fn display(&self, terminal: &mut TermType) {
    
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(20),
                        Constraint::Percentage(80),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let block = Block::default()
                .title("Block")
                .borders(Borders::ALL);

            f.render_widget(block, chunks[0]);


            let block = Block::default().title("Block 2").borders(Borders::ALL);
            f.render_widget(block, chunks[1]);

        }).unwrap();
    }
}
