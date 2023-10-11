use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::input::keyboard;
use ggez::{conf, Context, ContextBuilder, GameResult};
use rand::Rng;

fn usize_to_xy(n: usize, width: u32, height: u32) -> (u32, u32) {
    let x = (n as u32) % width;
    let y = ((n as u32) - x) / height;
    (x, y)
}
fn xy_to_usize(xy: (u32, u32), width: u32, _height: u32) -> usize {
    // wanted a similar signature to usize_to_xy
    (xy.0 + (xy.1 * width)) as usize
}

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
    let mut my_game = GOL::new(&mut ctx, width, height).unwrap();

    my_game.randomize(); // randomize before use...

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct GOL {
    cells: Vec<bool>,
    width: u32,
    height: u32,
    cell_squares: Vec<graphics::Mesh>,
    paused: bool,
}

impl GOL {
    pub fn new(ctx: &mut Context, width: u32, height: u32) -> GameResult<GOL> {
        // creating grid of cell squares here...
        let mut cell_squares: Vec<graphics::Mesh> = vec![];
        for i in 0..height {
            for j in 0..width {
                let x = (j * 10) as f32;
                let y = (i * 10) as f32;
                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect {
                        x,
                        y,
                        w: 10.0,
                        h: 10.0,
                    },
                    graphics::Color::WHITE,
                );
                cell_squares.push(rectangle.unwrap());
            }
        }
        Ok(GOL {
            cells: vec![false; (width * height) as usize],
            width,
            height,
            cell_squares,
            paused: false,
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
            paused: false,
        }
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
                    [xy_to_usize(((x - 1) as u32, (y - 1) as u32), self.width, self.height)]
                {
                    counter += 1;
                }
            }
            // top middle neighbor
            if self.cells[xy_to_usize((x as u32, (y - 1) as u32), self.width, self.height)] {
                counter += 1;
            }
            if right_boundary_valid {
                // top right neighbor
                if self.cells
                    [xy_to_usize(((x + 1) as u32, (y - 1) as u32), self.width, self.height)]
                {
                    counter += 1;
                }
            }
        }

        // middle neighbors
        if left_boundary_valid {
            // left mid neighbor
            if self.cells[xy_to_usize(((x - 1) as u32, y as u32), self.width, self.height)] {
                counter += 1;
            }
        }
        if right_boundary_valid {
            // right middle neighbor
            if self.cells[xy_to_usize(((x + 1) as u32, y as u32), self.width, self.height)] {
                counter += 1;
            }
        }

        if bottom_boundary_valid {
            // checking relative bottom row of neighbors
            if left_boundary_valid {
                // bottom left neighbor
                if self.cells
                    [xy_to_usize(((x - 1) as u32, (y + 1) as u32), self.width, self.height)]
                {
                    counter += 1;
                }
            }
            // bottom middle neighbor
            if self.cells[xy_to_usize((x as u32, (y + 1) as u32), self.width, self.height)] {
                counter += 1;
            }
            if right_boundary_valid {
                // bottom right neighbor
                if self.cells
                    [xy_to_usize(((x + 1) as u32, (y + 1) as u32), self.width, self.height)]
                {
                    counter += 1;
                }
            }
        }

        counter
    }

    // the following three functions all have to do with interactions on click
    pub fn make_cell_alive(&mut self, xy: (u32, u32)) {
        self.cells[xy_to_usize(xy, self.width, self.height)] = true;
    }
    pub fn kill_cell(&mut self, xy: (u32, u32)) {
        self.cells[xy_to_usize(xy, self.width, self.height)] = false;
    }
    pub fn switch_cell(&mut self, xy: (u32, u32)) {
        self.cells[xy_to_usize(xy, self.width, self.height)] =
            !self.cells[xy_to_usize(xy, self.width, self.height)];
    }

    pub fn pass(&mut self) {
        // represents a single pass of GOL
        // this is where all the actual logic of the game comes together
        let mut new_cells: Vec<bool> = vec![false; (self.width * self.height) as usize];
        for i in 0..*(&self.cells.len()) {
            let pos = usize_to_xy(i, self.width, self.height);
            let alive_neighbor_count = self.count_alive_neighbors(pos);

            // using condensed rules...
            if self.cells[i] && (alive_neighbor_count == 2 || alive_neighbor_count == 3) {
                // any live cell with two or three live neighbours survives
                new_cells[i] = true;
            } else if !(self.cells[i]) && (alive_neighbor_count == 3) {
                // any dead cell with three live neighbours becomes a live cell
                new_cells[i] = true;
            } else {
                // all other live cells die in the next generation
                // obviously all other dead cells stay dead
                new_cells[i] = false;
            }
        }
        self.cells = new_cells;
    }

    // interactions...
    pub fn randomize(&mut self) {
        // game completely randomizes on "R" keypress
        let mut rng = rand::thread_rng();
        for i in 0..self.cells.len() {
            self.cells[i] = rng.gen_bool(1.0 / 2.0); // one eighth
        }
    }
    pub fn pause(&mut self) {
        // game pauses on "P" keypress
        self.paused = !self.paused;
    }
}

impl EventHandler for GOL {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        if !self.paused {
            self.pass();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, None);
        // Draw code here...
        // continues drawing even when paused because of keypress N
        // which passes by one
        for i in 0..*(&self.cells.len()) {
            if self.cells[i] {
                // cell is alive
                canvas.draw(
                    &self.cell_squares[i],
                    graphics::DrawParam::default().color(graphics::Color::BLACK),
                );
            } else {
                // cell is not alive
                canvas.draw(
                    &self.cell_squares[i],
                    graphics::DrawParam::default().color(graphics::Color::WHITE),
                );
            }
        }

        canvas.finish(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        match input.keycode {
            None => (),
            Some(kc) => match kc {
                keyboard::KeyCode::Escape => ctx.request_quit(),
                keyboard::KeyCode::R => self.randomize(),
                keyboard::KeyCode::P => self.pause(),
                keyboard::KeyCode::N => {
                    // pass by one, should only be a thing on pause
                    if self.paused {
                        self.pass()
                    }
                }
                _ => (),
            },
        }

        Ok(())
    }
}

// for the tests
mod tests;
