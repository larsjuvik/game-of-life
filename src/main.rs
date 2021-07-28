use macroquad::prelude::*;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 600;

fn config() -> Conf {
    Conf {
        window_title: "Game of Life".to_owned(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

/////////////////
// CELL STUFF  //
/////////////////
type Dimension = (f32, f32);
enum CellState {
    DEAD,
    ALIVE,
}
struct Cell {
    grid_x: u32,
    grid_y: u32,
    state: CellState,
}
impl Cell {
    fn new(grid_x: u32, grid_y: u32, state: CellState) -> Cell {
        Cell { grid_x, grid_y, state }
    }

    fn draw(&self, size_px: Dimension) {
        draw_rectangle(
            self.grid_x as f32 * size_px.0,
            self.grid_y as f32 * size_px.1,
            size_px.0,
            size_px.1,
            match &self.state {
                CellState::DEAD => WHITE,
                CellState::ALIVE => BLACK,
            }
        )
    }
}


#[macroquad::main(config)]
async fn main() {

    let c1 = Cell::new(3u32, 4u32, CellState::ALIVE);

    loop {
        clear_background(WHITE);

        c1.draw((60.0, 60.0));

        next_frame().await
    }
}
