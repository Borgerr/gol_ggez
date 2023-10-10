#[cfg(test)]
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

#[test]
fn solo_alive_cell_dies_after_pass() {
    let mut instance = GOL::new_no_ctx(25, 25);
    let x = 3;
    let y = 1;
    let usize_pos = xy_to_usize((x, y), 25, 25);
    instance.make_cell_alive((x, y));

    for i in 0..instance.cells.len() {
        if i == usize_pos {
            assert!(instance.cells[i]);
        } else {
            assert!(!instance.cells[i]);
        }
    }
    assert!(instance.cells[usize_pos]);

    instance.pass();
    for cell in instance.cells {
        assert!(!cell);
    }
}

#[test]
fn square_all_stays_alive() {
    let mut instance = GOL::new_no_ctx(25, 25);
    let xy1 = (0, 0);
    let usize1 = xy_to_usize(xy1, 25, 25);
    let xy2 = (0, 1);
    let usize2 = xy_to_usize(xy2, 25, 25);
    let xy3 = (1, 0);
    let usize3 = xy_to_usize(xy3, 25, 25);
    let xy4 = (1, 1);
    let usize4 = xy_to_usize(xy4, 25, 25);
    instance.make_cell_alive(xy1);
    instance.make_cell_alive(xy2);
    instance.make_cell_alive(xy3);
    instance.make_cell_alive(xy4);

    assert!(instance.cells[usize1]);
    assert!(instance.cells[usize2]);
    assert!(instance.cells[usize3]);
    assert!(instance.cells[usize4]);
    for i in usize2 + 1..usize3 {
        assert!(!instance.cells[i]);
    }
    for i in usize4 + 1..(25 * 25) {
        assert!(!instance.cells[i]);
    }

    instance.pass();
    assert!(instance.cells[usize1]);
    assert!(instance.cells[usize2]);
    assert!(instance.cells[usize3]);
    assert!(instance.cells[usize4]);
    for i in usize2 + 1..usize3 {
        assert!(!instance.cells[i]);
    }
    for i in usize4 + 1..(25 * 25) {
        assert!(!instance.cells[i]);
    }
}
