use clap::Parser;

use crate::grid::Grid;

mod grid;
mod utils;
mod constants;


#[derive(clap::Parser)]
struct SmoothLife {
    /// The height of the grid, default is 100
    #[arg(long)]
    height: Option<usize>,
    /// The width of the grid, default is 100
    #[arg(long)]
    width: Option<usize>,
    /// The shrink factor of the grid, this will make the grid smaller by a factor of the given number
    /// default is None
    #[arg(long)]
    shrink_factor: Option<usize>,
    #[arg(long)]
    /// Use raylib to display the grid, note this is not implemented yet
    raylib: bool,
    /// The time step for the simulation, default is 0.05
    #[arg(long = "delta_time")]
    dt: Option<f64>
}

fn main() {
    let smooth_life = SmoothLife::parse();
    if smooth_life.raylib { unimplemented!("the raylib flag is not implemented yet!"); }
    let width = match smooth_life.width {
        None => { 100 }
        Some(s) => { s }
    };
    let height = match smooth_life.height {
        None => { 100 }
        Some(s) => { s }
    };
    let shrink_factor = match smooth_life.shrink_factor {
        None => { None }
        Some(s) => {
            assert!(s < width && s < height);
            Some(s)
        }
    };
    let dt = match smooth_life.dt {
        None => { 0.05 }
        Some(s) => { s }
    };
    let mut grid = Grid::init(height, width, shrink_factor);
    grid.display();
    loop {
        let diff = grid.compute_diff();
        grid.apply_grid_diff(diff, dt);
        grid.display();
    }
}
