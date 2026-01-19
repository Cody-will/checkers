slint::include_modules!();

mod game;
use rdev::display_size;
use crate::game::types::{Position, PlayerMove, GameState};
use slint::{ ComponentHandle };


fn main() -> Result<(), slint::PlatformError> {
    // Create the initial ui 
    let ui = AppWindow::new()?;

    // Get screen size to make the window size in slint
    let (screen_width, screen_height) = display_size().expect("Failed to get screen size");
    let screen_width = screen_width as i32;
    let screen_height = screen_height as i32;

    // Set the window size in slint 
    ui.set_screen_size(ScreenSize{screen_width, screen_height});

    // Starts the base view for the app Default = View::Lobby
    ui.set_view(View::new()); 
    
    // Create the game state -> base is None
    let mut game_state = GameState::new();
     
    
    
    
    
    // Lobby view
    {
        let ui_weak = ui.as_weak();

        ui.on_start_game(move || {
            if let Some(ui) = ui_weak.upgrade() { 
                ui.set_view(View::from(&CurrentView::Game));
            }
        });

        ui.on_quit(move || {
            slint::quit_event_loop().expect("Could not quit event loop");
        })
    }


    // Game view
    {
        let ui_weak = ui.as_weak();        
        let ui_weak_two = ui.as_weak();
        
        game_state.start();
        let mut game = game_state.state.expect("Could not load game");
        game.sync_ui(&ui);

        ui.on_to_lobby(move || {
            if let Some(ui) = ui_weak_two.upgrade() { 
                ui.set_view(View::from(&CurrentView::Lobby));
            } 
        });

        ui.on_player_move(move |position: crate::SlintPosition| {
        let to = Position::new(position.row as i8, position.col as i8);

        if let Some(ui) = ui_weak.upgrade() {
                
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
            }
    });
    }
  
    ui.run()
}

