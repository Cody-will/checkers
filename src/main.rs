mod game;
use slint::{ModelRc, ComponentHandle, VecModel};
slint::include_modules!();
use crate::game::board::Board;


fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let mut board = Board::new();
    let flat = board.flatten();
    let model = ModelRc::new(VecModel::from(flat));
    ui.on_player_move(move |position| {println!("position: {:?}", position);});
    ui.set_board(model);
    ui.run()
}

