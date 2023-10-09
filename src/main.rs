use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::{conf, Context, ContextBuilder, GameResult};

fn main() {
    // set dimensions for window
    let width = 50;
    let height = 50;

    // Configure window...
    let cb = ContextBuilder::new("Conway", "ash")
        .window_setup(conf::WindowSetup::default().title("Conway's"))
        .window_mode(
            conf::WindowMode::default().dimensions((width * 10) as f32, (height * 10) as f32),
        );

    /*
    let (mut ctx, event_loop) = ContextBuilder::new("conway", "ash")
        .build()
        .expect("aieee, could not create ggez context!");
    */

    // create a context and event loop...
    let (mut ctx, event_loop) = cb.build().expect("guh, could not create ggez context.");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = GOL::new(&mut ctx, width, height).unwrap();

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct GOL {
    cells: Vec<bool>,
    width: u32,
    height: u32,
    cell_squares: Vec<graphics::Rect>,
}

impl GOL {
    pub fn new(ctx: &mut Context, width: u32, height: u32) -> GameResult<GOL> {
        // creating grid of cell squares here...
        let cell_squares: Vec<graphics::Rect> = vec![];
        for i in 0..height {
            for j in 0..width {}
        }
        Ok(GOL {
            cells: vec![false; (width * height) as usize],
            width,
            height,
            cell_squares,
        })
    }
    pub fn new_no_ctx(width: u32, height: u32) -> GOL {
        // not using context here since we want to test the implementation
        // and we don't need to load images or anything like that
        GOL {
            cells: vec![false; (width * height) as usize],
            width,
            height,
            cell_squares: vec![],
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
                {
                    counter += 1;
                }
            }
            // top middle neighbor
            if self.cells[Self::xy_to_usize((x as u32, (y - 1) as u32), self.width, self.height)] {
                counter += 1;
            }
            if right_boundary_valid {
                // top right neighbor
                if self.cells
                    [Self::xy_to_usize(((x + 1) as u32, (y - 1) as u32), self.width, self.height)]
                {
                    counter += 1;
                }
            }
        }

        // middle neighbors
        if left_boundary_valid {
            // left mid neighbor
            if self.cells[Self::xy_to_usize(((x - 1) as u32, y as u32), self.width, self.height)] {
                counter += 1;
            }
        }
        if right_boundary_valid {
            // right middle neighbor
            if self.cells[Self::xy_to_usize(((x + 1) as u32, y as u32), self.width, self.height)] {
                counter += 1;
            }
        }

        if bottom_boundary_valid {
            // checking relative bottom row of neighbors
            if left_boundary_valid {
                // bottom left neighbor
                if self.cells
                    [Self::xy_to_usize(((x - 1) as u32, (y + 1) as u32), self.width, self.height)]
                {
                    counter += 1;
                }
            }
            // bottom middle neighbor
            if self.cells[Self::xy_to_usize((x as u32, (y + 1) as u32), self.width, self.height)] {
                counter += 1;
            }
            if right_boundary_valid {
                // bottom right neighbor
                if self.cells
                    [Self::xy_to_usize(((x + 1) as u32, (y + 1) as u32), self.width, self.height)]
                {
                    counter += 1;
                }
            }
        }

        counter
    }

    // the following three functions all have to do with interactions on click
    pub fn make_cell_alive(&mut self, xy: (u32, u32)) {
        self.cells[Self::xy_to_usize(xy, self.width, self.height)] = true;
    }
    pub fn kill_cell(&mut self, xy: (u32, u32)) {
        self.cells[Self::xy_to_usize(xy, self.width, self.height)] = false;
    }
    pub fn switch_cell(&mut self, xy: (u32, u32)) {
        self.cells[Self::xy_to_usize(xy, self.width, self.height)] =
            !self.cells[Self::xy_to_usize(xy, self.width, self.height)];
    }

    pub fn pass(&mut self) {
        // represents a single pass of GOL
        // this is where all the actual logic of the game comes together
        for i in 0..*(&self.cells.len()) {
            let pos = Self::usize_to_xy(i, self.width, self.height);
            let alive_neighbor_count = self.count_alive_neighbors(pos);

            // using condensed rules...
            if self.cells[i] && (alive_neighbor_count == 2 || alive_neighbor_count == 3) {
                // any live cell with two or three live neighbours survives
                continue;
            } else if !(self.cells[i]) && (alive_neighbor_count == 3) {
                // any dead cell with three live neighbours becomes a live cell
                self.cells[i] = true;
            } else {
                // all other live cells die in the next generation
                // obviously all other dead cells stay dead
                self.cells[i] = false;
            }
        }
    }
}

impl EventHandler for GOL {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        self.pass();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        // Draw code here...
        for i in 0..*(&self.cells.len()) {
            let pos = Self::usize_to_xy(i, self.width, self.height);
            // check if cell is alive
        }
        canvas.finish(ctx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    // GOL tests...
    #[test]
    fn created_game_instance() {
        GOL::new_no_ctx(25, 25);
    }

    fn all_neighbors_top_left_template(width: u32, height: u32) {
        // tests that the top left corner with all neighbors has a count of exactly 3 neighbors
        let mut instance = GOL::new_no_ctx(width, height);
        instance.make_cell_alive((0, 1)); // lower neighbor
        instance.make_cell_alive((1, 0)); // right neighbor
        instance.make_cell_alive((1, 1)); // lower-right neighbor
        assert_eq!(instance.count_alive_neighbors((0, 0)), 3);
    }
    #[test]
    fn all_neighbors_top_left() {
        let mut rng = rand::thread_rng();
        for _i in 0..10 {
            all_neighbors_top_left_template(rng.gen_range(3..2000), rng.gen_range(3..2000));
        }
    }

    fn all_neighbors_top_right_template(width: u32, height: u32) {
        // tests that the top left corner with all neighbors has a count of exactly 3 neighbors
        let mut instance = GOL::new_no_ctx(width, height);
        instance.make_cell_alive((width - 1, 1)); // lower neighbor
        instance.make_cell_alive((width - 2, 0)); // left neighbor
        instance.make_cell_alive((width - 2, 1)); // lower-left neighbor
        assert_eq!(instance.count_alive_neighbors((width - 1, 0)), 3);
    }
    #[test]
    fn all_neighbors_top_right() {
        let mut rng = rand::thread_rng();
        for _i in 0..10 {
            all_neighbors_top_right_template(rng.gen_range(3..2000), rng.gen_range(3..2000));
        }
    }

    fn all_neighbors_bottom_left_template(width: u32, height: u32) {
        // tests that the top left corner with all neighbors has a count of exactly 3 neighbors
        let mut instance = GOL::new_no_ctx(width, height);
        instance.make_cell_alive((0, height - 2)); // upper neighbor
        instance.make_cell_alive((1, height - 1)); // right neighbor
        instance.make_cell_alive((1, height - 2)); // upper-right neighbor
        assert_eq!(instance.count_alive_neighbors((0, height - 1)), 3);
    }
    #[test]
    fn all_neighbors_bottom_left() {
        let mut rng = rand::thread_rng();
        for _i in 0..10 {
            all_neighbors_bottom_left_template(rng.gen_range(3..2000), rng.gen_range(3..2000));
        }
    }

    fn all_neighbors_bottom_right_template(width: u32, height: u32) {
        // tests that the top left corner with all neighbors has a count of exactly 3 neighbors
        let mut instance = GOL::new_no_ctx(width, height);
        instance.make_cell_alive((width - 1, height - 2)); // upper neighbor
        instance.make_cell_alive((width - 2, height - 1)); // left neighbor
        instance.make_cell_alive((width - 2, height - 2)); // upper-left neighbor
        assert_eq!(instance.count_alive_neighbors((width - 1, height - 1)), 3);
    }
    #[test]
    fn all_neighbors_bottom_right() {
        let mut rng = rand::thread_rng();
        for _i in 0..10 {
            all_neighbors_bottom_right_template(rng.gen_range(3..2000), rng.gen_range(3..2000));
        }
    }

    fn all_neighbors_top_template(rng: &mut rand::rngs::ThreadRng) {
        let width = &rng.gen_range(3..2000);
        let height = &rng.gen_range(3..2000);
        let mut instance = GOL::new_no_ctx(*width, *height);
        let x = &rng.gen_range(1..width - 2);
        let x = *x;
        let y = 0; // must be at top row
        instance.make_cell_alive((x - 1, y)); // left neighbor
        instance.make_cell_alive((x + 1, y)); // right neighbor
        instance.make_cell_alive((x - 1, y + 1)); // bottom left neighbor
        instance.make_cell_alive((x, y + 1)); // bottom neighbor
        instance.make_cell_alive((x + 1, y + 1)); // bottom right neighbor
        assert_eq!(instance.count_alive_neighbors((x, y)), 5);
    }
    #[test]
    fn all_neighbors_top() {
        let mut rng = rand::thread_rng();
        for _i in 0..10 {
            all_neighbors_top_template(&mut rng);
        }
    }

    fn all_neighbors_bottom_template(rng: &mut rand::rngs::ThreadRng) {
        let width = &rng.gen_range(3..2000);
        let height = &rng.gen_range(3..2000);
        let mut instance = GOL::new_no_ctx(*width, *height);
        let x = &rng.gen_range(1..width - 2);
        let x = *x;
        let y = height - 1; // must be at top row
        instance.make_cell_alive((x - 1, y)); // left neighbor
        instance.make_cell_alive((x + 1, y)); // right neighbor
        instance.make_cell_alive((x - 1, y - 1)); // top left neighbor
        instance.make_cell_alive((x, y - 1)); // top neighbor
        instance.make_cell_alive((x + 1, y - 1)); // top right neighbor
        assert_eq!(instance.count_alive_neighbors((x, y)), 5);
    }
    #[test]
    fn all_neighbors_bottom() {
        let mut rng = rand::thread_rng();
        for _i in 0..10 {
            all_neighbors_bottom_template(&mut rng);
        }
    }

    fn all_neighbors_left_template(rng: &mut rand::rngs::ThreadRng) {
        let width = &rng.gen_range(3..2000);
        let height = &rng.gen_range(3..2000);
        let mut instance = GOL::new_no_ctx(*width, *height);
        let x = 0; // must be left side
        let y = &rng.gen_range(1..height - 2);
        let y = *y;
        instance.make_cell_alive((x, y - 1)); // top neighbor
        instance.make_cell_alive((x, y + 1)); // bottom neighbor
        instance.make_cell_alive((x + 1, y - 1)); // top right neighbor
        instance.make_cell_alive((x + 1, y)); // right neighbor
        instance.make_cell_alive((x + 1, y + 1)); // bottom right neighbor
        assert_eq!(instance.count_alive_neighbors((x, y)), 5);
    }
    #[test]
    fn all_neighbors_left() {
        let mut rng = rand::thread_rng();
        for _i in 0..10 {
            all_neighbors_left_template(&mut rng);
        }
    }

    fn all_neighbors_right_template(rng: &mut rand::rngs::ThreadRng) {
        let width = &rng.gen_range(3..2000);
        let height = &rng.gen_range(3..2000);
        let mut instance = GOL::new_no_ctx(*width, *height);
        let x = height - 1; // must be left side
        let y = &rng.gen_range(1..height - 2);
        let y = *y;
        instance.make_cell_alive((x, y - 1)); // top neighbor
        instance.make_cell_alive((x, y + 1)); // bottom neighbor
        instance.make_cell_alive((x - 1, y - 1)); // top left neighbor
        instance.make_cell_alive((x - 1, y)); // left neighbor
        instance.make_cell_alive((x - 1, y + 1)); // bottom left neighbor
        assert_eq!(instance.count_alive_neighbors((x, y)), 5);
    }
    #[test]
    fn all_neighbors_right() {
        let mut rng = rand::thread_rng();
        for _i in 0..10 {
            all_neighbors_right_template(&mut rng);
        }
    }

    fn arbitrary_center_template(rng: &mut rand::rngs::ThreadRng) {
        let width = &rng.gen_range(3..2000);
        let height = &rng.gen_range(3..2000);
        let mut instance = GOL::new_no_ctx(*width, *height);
        let x = &rng.gen_range(1..width - 2);
        let x = *x;
        let y = &rng.gen_range(1..height - 2);
        let y = *y;

        instance.make_cell_alive((x, y - 1)); // top neighbor
        instance.make_cell_alive((x, y + 1)); // bottom neighbor
        instance.make_cell_alive((x - 1, y - 1)); // top left neighbor
        instance.make_cell_alive((x - 1, y)); // left neighbor
        instance.make_cell_alive((x - 1, y + 1)); // bottom left neighbor
        instance.make_cell_alive((x + 1, y - 1)); // top right neighbor
        instance.make_cell_alive((x + 1, y)); // right neighbor
        instance.make_cell_alive((x + 1, y + 1)); // bottom right neighbor
        assert_eq!(instance.count_alive_neighbors((x, y)), 8);
    }
    #[test]
    fn arbitrary_center_all_neighbors_alive() {
        let mut rng = rand::thread_rng();
        for _i in 0..10 {
            arbitrary_center_template(&mut rng);
        }
    }
}
