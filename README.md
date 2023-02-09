# metropolis_hastings
![ci](https://github.com/dannasman/metropolis_hastings/actions/workflows/rust.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)

Metropolis-Hastings algorithm for modeling 2D Ising model written in Rust.

## How to use
The program takes six arguments in the following order: file name, grid width, grid height, interaction strength $\frac{J}{k_B}$ ( $k_B$ being the Boltzmann constant) and temperature $T$ in K. For example, if we want to run `100` Monte Carlo steps on a `10x10` grid with interaction strength `1.0` and temperature `273.15` K and save the configuration to file `data.txt` we run the following command in console:
```
cargo run data.txt 10 10 1.0 273.15 100
```
