use macroquad::prelude::*;
use sudoku::Sudoku;

#[macroquad::main(config)]
async fn main() {
    let mut game = Sudoku::new();

    {
        let mut s = "".to_string();
        for y in 0..9 {
            s += &(0..9)
                .map(|x| game.board.solution[y][x].to_string())
                .collect::<String>();
            s += "\n";
        }
        debug!("Solution:\n{}", s);
    }

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
                    KeyCode::Key0 | KeyCode::Kp0 => game.input(Some(0)),
                    KeyCode::Key1 | KeyCode::Kp1 => game.input(Some(1)),
                    KeyCode::Key2 | KeyCode::Kp2 => game.input(Some(2)),
                    KeyCode::Key3 | KeyCode::Kp3 => game.input(Some(3)),
                    KeyCode::Key4 | KeyCode::Kp4 => game.input(Some(4)),
                    KeyCode::Key5 | KeyCode::Kp5 => game.input(Some(5)),
                    KeyCode::Key6 | KeyCode::Kp6 => game.input(Some(6)),
                    KeyCode::Key7 | KeyCode::Kp7 => game.input(Some(7)),
                    KeyCode::Key8 | KeyCode::Kp8 => game.input(Some(8)),
                    KeyCode::Key9 | KeyCode::Kp9 => game.input(Some(9)),
                    KeyCode::Backspace | KeyCode::Escape => game.input(None),
                    KeyCode::KpEnter => info!("{}", game.board.check_board()),

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
