use std::env;
use std::error::Error;
use std::time::Instant;

mod metropolis_hastings;
use metropolis_hastings::SpinGrid;

fn main() -> Result<(), Box<dyn Error>> {
    if env::args().len() == 6 {
        let filename = env::args()
            .nth(1)
            .expect("second argument must be file name");
        let order = std::env::args()
            .nth(2)
            .expect("third argument must be the order of the Hilbert curve")
            .parse::<usize>()?;
        let inter_strength = std::env::args()
            .nth(3)
            .expect("fourth argument must be interaction strength J/(kT)")
            .parse::<f64>()?;
        let temperature = std::env::args()
            .nth(4)
            .expect("eight argument must be temperature T in Kelvin")
            .parse::<f64>()?;

        let iterations = std::env::args()
            .nth(5)
            .expect("sixth argument must be number of Monte Carlo simulation steps")
            .parse::<u32>()?;

        let mut spin_grid = SpinGrid::new_hcurve(order);
        let mut i = 0;
        let now = Instant::now();
        while i < iterations {
            spin_grid.calculate_configurations_hcurve(inter_strength, temperature)?;
            i += 1;
        }
        let elapsed = now.elapsed();
        println!("Simulation completed in {:?}", elapsed);
        spin_grid.save(&filename)?;
    } else if env::args().len() == 7 {
        let filename = env::args()
            .nth(1)
            .expect("second argument must be file name");
        let dim_x = std::env::args()
            .nth(2)
            .expect("third argument must be length of the grid along x-axis")
            .parse::<usize>()?;
        let dim_y = std::env::args()
            .nth(3)
            .expect("fourth argument must be length of the grid along y-axis")
            .parse::<usize>()?;
        let inter_strength = std::env::args()
            .nth(4)
            .expect("fifth argument must be interaction strength J/(kT)")
            .parse::<f64>()?;
        let temperature = std::env::args()
            .nth(5)
            .expect("sixth argument must be temperature T in Kelvin")
            .parse::<f64>()?;

        let iterations = std::env::args()
            .nth(6)
            .expect("seventh argument must be number of Monte Carlo simulation steps")
            .parse::<u32>()?;

        let mut spin_grid = SpinGrid::new(dim_x, dim_y);
        let mut i = 0;
        let now = Instant::now();
        while i < iterations {
            spin_grid.calculate_configurations(inter_strength, temperature);
            i += 1;
        }
        let elapsed = now.elapsed();
        println!("Simulation completed in {:?}", elapsed);
        spin_grid.save(&filename)?;
    }
    Ok(())
}
