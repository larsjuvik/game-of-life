use macroquad::prelude::*;

fn config() -> Conf {
    Conf {
        window_title: "Game of Life".to_owned(),
        window_width: 1000,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(config)]
async fn main() {

    loop {
        clear_background(WHITE);

        next_frame().await
    }
}
