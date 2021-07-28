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
struct World2D {
    cells: Vec<Vec<Cell>>,
}
impl World2D {
    fn new(cells_x: u32, cells_y: u32) -> World2D {
        let mut cells = Vec::new();

        for y in 0..cells_y as i32 {
            let mut row = Vec::new();

            for x in 0..cells_x as i32 {
                let cell = match rand::gen_range::<u8>(0, 2) {
                    0 => Cell::new(x, y, CellState::ALIVE),
                    _ => Cell::new(x, y, CellState::DEAD),
                };
                row.push(cell);
            }
            cells.push(row);
        }
        World2D { cells }
    }

    fn cells_x(&self) -> u32 { self.cells.get(0).unwrap().len() as u32 }
    fn cells_y(&self) -> u32 { self.cells.len() as u32 }
}
impl Draw for World2D {
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
#[derive(Copy, Clone)]
enum CellState {
    DEAD,
    ALIVE,
}
#[derive(Copy, Clone)]
struct Cell {
    grid_x: i32,
    grid_y: i32,
    state: CellState,
}
impl Cell {
    fn new(grid_x: i32, grid_y: i32, state: CellState) -> Cell {
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

    let grid = World2D::new(10, 10);

    loop {
        clear_background(WHITE);

        // input
        if is_key_pressed(KeyCode::Space) {
            println!("Space pressed!");
        }

        grid.draw((HEIGHT as f32, WIDTH as f32));

        next_frame().await
    }
}

/////////////////
// TESTS       //
/////////////////
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn grid_get_correct_number_cells() {
        let grid = World2D::new(10, 20);
        assert_eq!(grid.cells_x(), 10);
        assert_eq!(grid.cells_y(), 20);
    }

    #[test]
    fn grid_get_correct_clone_cells() {
        let dim = (20, 10);
        let grid = World2D::new(dim.0, dim.1);
        let cells_original = &grid.cells;
        let cells_clone = &grid.cells.clone();

        for y in 0..dim.1 as usize {
            for x in 0..dim.0 as usize {
                let cells_original = &cells_original.get(y).unwrap().get(x).unwrap();
                let cells_clone = &cells_clone.get(y).unwrap().get(x).unwrap();

                assert_eq!(cells_original.grid_x, cells_clone.grid_x);
                assert_eq!(cells_original.grid_y, cells_clone.grid_y);
            }
        }
    }
}
