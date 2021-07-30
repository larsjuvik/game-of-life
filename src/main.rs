use rand::random;
use macroquad::miniquad::conf::Conf;
use macroquad::shapes::{draw_rectangle, draw_rectangle_lines};
use macroquad::color::{WHITE, BLACK};
use macroquad::window::{clear_background, next_frame};
use macroquad::input::{KeyCode, is_key_down};
use macroquad::text::draw_text;
use macroquad::ui::root_ui;
use macroquad::math::vec2;

// WINDOW
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
    gen: u32,
}
impl World2D {

    // for testing
    #[cfg(test)]
    fn all(size: u32, state: CellState) -> World2D {
        let mut cells = Vec::new();

        for y in 0..size as i32 {
            let mut row = Vec::new();

            for x in 0..size as i32 {
                row.push(Cell::new(x, y, state));
            }
            cells.push(row);
        }
        World2D { cells, gen: 0 }
    }

    fn new(cells_x: u32, cells_y: u32) -> World2D {
        let mut cells = Vec::new();

        for y in 0..cells_y as i32 {
            let mut row = Vec::new();

            for x in 0..cells_x as i32 {
                let cell = match random() {
                    false => Cell::new(x, y, CellState::ALIVE),
                    true => Cell::new(x, y, CellState::DEAD),
                };
                row.push(cell);
            }
            cells.push(row);
        }
        World2D { cells, gen: 0 }
    }

    fn set_cell(&mut self, grid_x: i32, grid_y: i32, state: CellState) {
        match self.cells.get_mut(grid_y as usize) {
            Some(row) => match row.get_mut(grid_x as usize) {
                Some(cell) => { cell.state = state; },
                None => (),
            },
            None => (),
        }
    }

    fn get_cell(&self, grid_x: i32, grid_y: i32) -> Option<&Cell> {
        match self.cells.get(grid_y as usize) {
            Some(row) => row.get(grid_x as usize),
            None => None,
        }
    }

    fn get_neighbours(&self, grid_x: i32, grid_y: i32) -> u32 {
        // checking 3x3 grid
        let mut counter = 0;
        for y in -1..2 {
            for x in -1..2 {
                if x == 0 && y == 0 {
                    continue
                }

                match self.get_cell(grid_x+x, grid_y+y) {
                    Some(c) => {
                        if c.state == CellState::ALIVE {
                            counter += 1;
                        }
                    },
                    None => ()
                }
            }
        }

        counter
    }

    // performs one tick; applies the rules of Conway's Game of Life
    fn tick(&mut self) {
        let old_world = World2D { cells: self.cells.clone(), gen: self.gen };

        for row in &old_world.cells {
            for cell in row {
                let neighbours = old_world.get_neighbours(cell.grid_x, cell.grid_y);

                if cell.state == CellState::DEAD && neighbours == 3 {
                    self.set_cell(cell.grid_x, cell.grid_y, CellState::ALIVE);
                }
                else if cell.state == CellState::ALIVE && !(neighbours == 2 || neighbours == 3) {
                    self.set_cell(cell.grid_x, cell.grid_y, CellState::DEAD);
                }
            }
        }
        self.gen += 1;
    }

    fn cells_x(&self) -> u32 { self.cells.get(0).unwrap().len() as u32 }
    fn cells_y(&self) -> u32 { self.cells.len() as u32 }
}
impl Draw for World2D {
    fn draw(&self, size_px: Dimension) {
        // draw border
        draw_rectangle_lines(-1.0, -1.0, size_px.0+2.0, size_px.1+2.0, 2.0, BLACK);

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
#[derive(PartialEq, Copy, Clone)]
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
    const WORLD_SIZE: (f32, f32) = (600.0, 600.0);

    // Mutable variables
    let cells_x = 100;
    let cells_y = 100;
    let mut world = World2D::new(cells_x, cells_y);

    loop {
        clear_background(WHITE);

        // UI
        if root_ui().button(vec2(WORLD_SIZE.0+30.0, 40.0), "Re-Generate") {
            println!("Generating new world!");
            world = World2D::new(cells_x, cells_y);
        };

        // input
        if is_key_down(KeyCode::Space) {
            world.tick();
        }

        world.draw(WORLD_SIZE);

        draw_text(format!("Generation {}", world.gen).as_str(), WORLD_SIZE.0+30.0, 30.0, 40.0, BLACK);
        draw_text("(hold space to iterate generations)", WORLD_SIZE.0+30.0, 90.0, 20.0, BLACK);

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
    fn test_get_neighbours_correct_amount() {
        let grid = World2D::all(3, CellState::ALIVE);
        assert_eq!(grid.get_neighbours(1, 1), 8);
    }

    #[test]
    fn test_get_cell_not_none() {
        let grid = World2D::new(10, 10);
        match grid.get_cell(0, 0) {
            Some(_) => (),
            None => panic!("Cell should not be 'None'."),
        }
    }

    #[test]
    fn test_get_cell_correct_coords() {
        let grid = World2D::new(10, 10);
        match grid.get_cell(0, 0) {
            Some(cell) => {
                assert_eq!(cell.grid_x, 0);
                assert_eq!(cell.grid_y, 0);
            },
            None => panic!("Cell should not be 'None'."),
        }
    }

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
