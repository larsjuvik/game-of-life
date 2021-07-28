use macroquad::prelude::*;
use macroquad::rand;

const WIDTH: u32 = 600;
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
// RENDERING   //
/////////////////
type Dimension = (f32, f32);
trait Draw {
    fn draw(&self, size_px: Dimension);
}

/////////////////
// GRID STUFF  //
/////////////////
struct Grid {
    cells: Vec<Vec<Cell>>,
}
impl Grid {
    fn new(cells_x: u32, cells_y: u32) -> Grid {
        let mut cells = Vec::new();

        for y in 0..cells_y {
            let mut row = Vec::new();

            for x in 0..cells_x {
                let cell = match rand::gen_range::<u8>(0, 2) {
                    0 => Cell::new(x, y, CellState::ALIVE),
                    _ => Cell::new(x, y, CellState::DEAD),
                };
                row.push(cell);
            }
            cells.push(row);
        }
        Grid { cells }
    }

    fn cells_x(&self) -> u32 { self.cells.get(0).unwrap().len() as u32 }
    fn cells_y(&self) -> u32 { self.cells.len() as u32 }
}
impl Draw for Grid {
    fn draw(&self, size_px: Dimension) {
        let cell_w = size_px.0 / self.cells_x() as f32;
        let cell_h = size_px.1 / self.cells_y() as f32;

        for row in &self.cells {
            for cell in row {
                cell.draw((cell_w, cell_h));
            }
        }
    }
}

/////////////////
// CELL STUFF  //
/////////////////
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
}
impl Draw for Cell {
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

    let grid = Grid::new(10, 10);

    loop {
        clear_background(WHITE);

        grid.draw((HEIGHT as f32, WIDTH as f32));

        next_frame().await
    }
}
