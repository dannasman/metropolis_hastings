mod metropolis_hastings;
use metropolis_hastings::SpinGrid;
use std::io::Error;

fn main() -> Result<(), Error> {
    let mut spin_grid = SpinGrid::new(100, 100, 0.01, 273.15);
    spin_grid.calculate_configurations();
    spin_grid.save("data.txt")?;
    Ok(())
}
