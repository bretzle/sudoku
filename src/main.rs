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

            if let Some(key) = get_last_key_pressed() {
                match key {
                    KeyCode::Key0 | KeyCode::Kp0 => game.input(0),
                    KeyCode::Key1 | KeyCode::Kp1 => game.input(1),
                    KeyCode::Key2 | KeyCode::Kp2 => game.input(2),
                    KeyCode::Key3 | KeyCode::Kp3 => game.input(3),
                    KeyCode::Key4 | KeyCode::Kp4 => game.input(4),
                    KeyCode::Key5 | KeyCode::Kp5 => game.input(5),
                    KeyCode::Key6 | KeyCode::Kp6 => game.input(6),
                    KeyCode::Key7 | KeyCode::Kp7 => game.input(7),
                    KeyCode::Key8 | KeyCode::Kp8 => game.input(8),
                    KeyCode::Key9 | KeyCode::Kp9 => game.input(9),
                    _ => {}
                }
            }
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
