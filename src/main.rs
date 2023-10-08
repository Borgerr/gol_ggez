use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("conway", "ash")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = GOL::new(25, 25);

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct GOL {
    cells: Vec<Cell>,
    width: u32,
    height: u32,
}

impl GOL {
    pub fn new(width: u32, height: u32) -> GOL {
        // not using context here since we want to test the implementation
        // and we don't need to load images or anything like that
        GOL {
            cells: vec![Cell::new(); (width * height) as usize],
            width,
            height,
        }
    }

    fn usize_to_xy(n: usize, width: u32, height: u32) -> (u32, u32) {
        (n as u32 % width, n as u32 % height)
    }
    fn xy_to_usize(xy: (u32, u32), width: u32, _height: u32) -> usize {
        // wanted a similar signature to usize_to_xy
        (xy.0 + (xy.1 * width)) as usize
    }

    pub fn count_alive_neighbors(&self, tup: (u32, u32)) -> u8 {
        // if a cell neighbor is "negative" or greater than width or height,
        // we don't add it, period.
        // made public merely for ease of testing
        let x = tup.0 as i32;
        let y = tup.1 as i32;
        let left_boundary_valid = x - 1 >= 0;
        let right_boundary_valid = x + 1 < self.width as i32;
        let top_boundary_valid = y - 1 >= 0;
        let bottom_boundary_valid = y + 1 < self.height as i32;

        let mut counter = 0;
        if top_boundary_valid {
            // checking relative top row of neighbors
            if left_boundary_valid {
                // top left neighbor
                if self.cells
                    [Self::xy_to_usize(((x - 1) as u32, (y - 1) as u32), self.width, self.height)]
                .alive
                {
                    counter += 1;
                }
            }
            // top middle neighbor
            if self.cells[Self::xy_to_usize((x as u32, (y - 1) as u32), self.width, self.height)]
                .alive
            {
                counter += 1;
            }
            if right_boundary_valid {
                // top right neighbor
                if self.cells
                    [Self::xy_to_usize(((x + 1) as u32, (y - 1) as u32), self.width, self.height)]
                .alive
                {
                    counter += 1;
                }
            }
        }

        // middle neighbors
        if left_boundary_valid {
            // left mid neighbor
            if self.cells[Self::xy_to_usize(((x - 1) as u32, y as u32), self.width, self.height)]
                .alive
            {
                counter += 1;
            }
        }
        if right_boundary_valid {
            // right middle neighbor
            if self.cells[Self::xy_to_usize(((x + 1) as u32, y as u32), self.width, self.height)]
                .alive
            {
                counter += 1;
            }
        }

        if bottom_boundary_valid {
            // checking relative bottom row of neighbors
            if left_boundary_valid {
                // bottom left neighbor
                if self.cells
                    [Self::xy_to_usize(((x - 1) as u32, (y + 1) as u32), self.width, self.height)]
                .alive
                {
                    counter += 1;
                }
            }
            // bottom middle neighbor
            if self.cells[Self::xy_to_usize((x as u32, (y + 1) as u32), self.width, self.height)]
                .alive
            {
                counter += 1;
            }
            if right_boundary_valid {
                // bottom right neighbor
                if self.cells
                    [Self::xy_to_usize(((x + 1) as u32, (y + 1) as u32), self.width, self.height)]
                .alive
                {
                    counter += 1;
                }
            }
        }

        counter
    }

    pub fn make_cell_alive(&mut self, xy: (u32, u32)) {
        self.cells[Self::xy_to_usize(xy, self.width, self.height)].alive = true;
    }
    pub fn kill_cell(&mut self, xy: (u32, u32)) {
        self.cells[Self::xy_to_usize(xy, self.width, self.height)].alive = false;
    }
    pub fn switch_cell(&mut self, xy: (u32, u32)) {
        self.cells[Self::xy_to_usize(xy, self.width, self.height)].alive =
            !self.cells[Self::xy_to_usize(xy, self.width, self.height)].alive;
    }
}

impl EventHandler for GOL {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        // Draw code here...
        canvas.finish(ctx)
    }
}

struct Cell {
    alive: bool,
}

impl Cell {
    pub fn new() -> Self {
        Cell { alive: false }
    }
}

impl Clone for Cell {
    fn clone(&self) -> Self {
        Self {
            alive: self.alive.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    // GOL tests...
    #[test]
    fn created_game_instance() {
        GOL::new(25, 25);
    }

    fn all_neighbors_top_left_template(width: u32, height: u32) {
        // tests that the top left corner with all neighbors has a count of exactly 3 neighbors
        let mut instance = GOL::new(width, height);
        instance.make_cell_alive((0, 1)); // lower neighbor
        instance.make_cell_alive((1, 0)); // right neighbor
        instance.make_cell_alive((1, 1)); // lower-right neighbor
        assert_eq!(instance.count_alive_neighbors((0, 0)), 3);
    }

    #[test]
    fn all_neighbors_top_left() {
        let mut rng = rand::thread_rng();
        all_neighbors_top_left_template(rng.gen_range(0..2000), rng.gen_range(0..2000));
        all_neighbors_top_left_template(rng.gen_range(0..2000), rng.gen_range(0..2000));
        all_neighbors_top_left_template(rng.gen_range(0..2000), rng.gen_range(0..2000));
        all_neighbors_top_left_template(rng.gen_range(0..2000), rng.gen_range(0..2000));
    }

    fn all_neighbors_top_right_template(width: u32, height: u32) {
        // tests that the top left corner with all neighbors has a count of exactly 3 neighbors
        let mut instance = GOL::new(width, height);
        instance.make_cell_alive((width - 1, 1)); // lower neighbor
        instance.make_cell_alive((width - 2, 0)); // left neighbor
        instance.make_cell_alive((width - 1, 1)); // lower-left neighbor
        assert_eq!(instance.count_alive_neighbors((width - 1, 0)), 3);
    }

    #[test]
    fn all_neighbors_top_right() {
        let mut rng = rand::thread_rng();
        all_neighbors_top_right_template(rng.gen_range(0..2000), rng.gen_range(0..2000));
        all_neighbors_top_right_template(rng.gen_range(0..2000), rng.gen_range(0..2000));
        all_neighbors_top_right_template(rng.gen_range(0..2000), rng.gen_range(0..2000));
        all_neighbors_top_right_template(rng.gen_range(0..2000), rng.gen_range(0..2000));
    }

    fn all_neighbors_bottom_left_template(width: u32, height: u32) {
        // tests that the top left corner with all neighbors has a count of exactly 3 neighbors
        let mut instance = GOL::new(width, height);
        instance.make_cell_alive((0, height - 2)); // upper neighbor
        instance.make_cell_alive((1, height - 1)); // right neighbor
        instance.make_cell_alive((1, height - 2)); // upper-right neighbor
        assert_eq!(instance.count_alive_neighbors((0, height - 1)), 3);
    }

    #[test]
    fn all_neighbors_bottom_left() {
        let mut rng = rand::thread_rng();
        all_neighbors_bottom_left_template(rng.gen_range(0..2000), rng.gen_range(0..2000));
        all_neighbors_bottom_left_template(rng.gen_range(0..2000), rng.gen_range(0..2000));
        all_neighbors_bottom_left_template(rng.gen_range(0..2000), rng.gen_range(0..2000));
        all_neighbors_bottom_left_template(rng.gen_range(0..2000), rng.gen_range(0..2000));
    }

    fn all_neighbors_bottom_right_template(width: u32, height: u32) {
        // tests that the top left corner with all neighbors has a count of exactly 3 neighbors
        let mut instance = GOL::new(width, height);
        instance.make_cell_alive((width - 1, height - 2)); // upper neighbor
        instance.make_cell_alive((width - 2, height - 1)); // left neighbor
        instance.make_cell_alive((width - 2, height - 2)); // upper-left neighbor
        assert_eq!(instance.count_alive_neighbors((width - 1, height - 1)), 3);
    }

    #[test]
    fn all_neighbors_bottom_right() {
        let mut rng = rand::thread_rng();
        all_neighbors_bottom_right_template(rng.gen_range(0..2000), rng.gen_range(0..2000));
        all_neighbors_bottom_right_template(rng.gen_range(0..2000), rng.gen_range(0..2000));
        all_neighbors_bottom_right_template(rng.gen_range(0..2000), rng.gen_range(0..2000));
        all_neighbors_bottom_right_template(rng.gen_range(0..2000), rng.gen_range(0..2000));
    }

    #[test]
    fn check_clone() {
        let c = Cell::new();
        assert_eq!(c.alive, c.clone().alive);
    }
}
