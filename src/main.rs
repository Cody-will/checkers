slint::include_modules!();
mod game;
use rdev::display_size;
use crate::game::types::{Position, PlayerMove};
use crate::game::play::Game;
use slint::{ ComponentHandle };
use std::cell::RefCell;
use std::rc::Rc;


fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
 
    let ui_weak = ui.as_weak();
    
    let game = Rc::new(RefCell::new(Game::default()));
    let (screen_width, screen_height) = display_size().expect("Failed to get screen size");
    let screen_width = screen_width as i32;
    let screen_height = screen_height as i32;
    
    ui.set_screen_size(ScreenSize{screen_width, screen_height});
    ui.set_view(View::new());
    game.borrow().sync_ui(&ui); 
 
    let game_cb = Rc::clone(&game);
    
    ui.on_player_move(move |position: crate::SlintPosition| {
        
        let ui = match ui_weak.upgrade() {
            Some(ui) => ui,
            None => return,
        };
        
        let to = Position::new(position.row as i8, position.col as i8);

        let mut game = game_cb.borrow_mut();

        if let Some(from) = game.selected {
            if from == to {
                game.selected = None;
                game.sync_ui(&ui);
                return;
            }
            game.make_move(PlayerMove::new(from, to)); 
            game.sync_ui(&ui);
           
        } else {
            
            game.select(to);
            game.sync_ui(&ui);
        }
    });

    ui.run()
}

