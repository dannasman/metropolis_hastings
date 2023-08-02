# metropolis_hastings
![ci](https://github.com/dannasman/metropolis_hastings/actions/workflows/rust.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)

Metropolis-Hastings algorithm for modeling 2D Ising model written in Rust.

## How to use
### Default simulation
The program takes six arguments in the following order: file name, grid width, grid height, interaction strength $\frac{J}{k_B}$ ( $k_B$ being the Boltzmann constant) and temperature $T$ in K. For example, if we want to run `100` Monte Carlo steps on a `128x128` grid with interaction strength `1.0` and temperature `2.269` K and save the configuration to file `data.txt` we run the following command in console:
```
cargo run data.txt 128 1.0 2.269 100
```
### Simulation with Hilbert Curve traversal
The program takes five arguments in the following order:
file name, Hilbert curve order number, interaction strength $\frac{J}{k_B}$ ( $k_B$ being the Boltzmann constant) and temperature $T$ in K. For example, if we want to run `100` Monte Carlo steps on a grid with size correspondint to a `7`th order Hilbert curve with interaction strength `1.0` and temperature `2.269` K and save the configuration to file `data.txt` we run the following command in console:
```
cargo run data.txt 7 1.0 2.269 100
```

## Further reading
Further discussion about the implementation in [here](https://dannasman.github.io/metropolis-hastings-algorithm) and [here](https://dannasman.github.io/hilbert-curve-ising-model).
