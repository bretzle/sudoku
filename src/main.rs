use macroquad::prelude::*;
use sudoku::Sudoku;

#[macroquad::main(config)]
async fn main() {
    let mut game = Sudoku::new();

    loop {
        clear_background(WHITE);

        // draw game
        game.render();

        // controls
        let mouse_pos = mouse_position();

        {
            game.hover(mouse_pos);

            if is_mouse_button_pressed(MouseButton::Left) {
                game.select(mouse_pos);
            }

            // if is_key_pressed(KeyCode::Key0) {
            //     game.input(0);
            // } else if is_key_pressed(KeyCode::Key1) {
            // 	game.input(1);
            // } else if is_key_pressed(KeyCode::Key1) {
            // 	game.input(2);
            // } else if is_key_pressed(KeyCode::Key1) {
            // 	game.input(3);
            // } else if is_key_pressed(KeyCode::Key1) {
            // 	game.input(4);
            // } else if is_key_pressed(KeyCode::Key1) {
            // 	game.input(5);
            // } else if is_key_pressed(KeyCode::Key1) {
            // 	game.input(6);
            // } else if is_key_pressed(KeyCode::Key1) {
            // 	game.input(7);
            // } else if is_key_pressed(KeyCode::Key1) {
            // 	game.input(8);
            // } else if is_key_pressed(KeyCode::Key1) {
            // 	game.input(9);
            // }
        }

        next_frame().await
    }
}

fn config() -> Conf {
    Conf {
        window_title: "Sudoku".to_owned(),
        window_width: 900,
        window_height: 900,
        high_dpi: false,
        fullscreen: false,
        sample_count: 4,
        ..Default::default()
    }
}
