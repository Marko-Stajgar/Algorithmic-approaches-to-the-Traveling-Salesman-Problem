# Algorithmic-approaches-to-the-Traveling-Salesman-Problem

In this project I want to try and implement few algorithms for finding the shortest path through [Hamiltonian cycles](https://en.wikipedia.org/wiki/Hamiltonian_path) in complete, weighted, undirected [graphs](https://en.wikipedia.org/wiki/Graph_(discrete_mathematics)), more popularly known as the [Traveling-Salesman-Problem](https://en.wikipedia.org/wiki/Travelling_salesman_problem).

The whole project is build using the [bevy](https://github.com/bevyengine/bevy) game engine and [nalgebra](https://github.com/dimforge/nalgebra).

# How to install & run
If you don't have rust & cargo installed, follow the guide for installation via [rustup](https://www.rust-lang.org/tools/install).

Then clone the repository and run using `cargo run`.

# In-app console commands
The in-app console is enabled from the start and doesn't need any further activation. To enter a command, just start typing and hit enter.

List of commands:

`exit` - exits the app

`clear` - clears the graph

`set <variable_name>: <value>` - changes the value of the desired parameter

`solve with <algorithm_name>` - activates the desired algorithm

`stop` - terminates all running algorithms


# Ant-Colony-Optimization

The first algorithm I chose to implement is the ant-colony-optimization. My implementation follows the standard pseudocode and equations mentioned in [this paper](https://www.semanticscholar.org/paper/Ant-colony-optimization%3A-artificial-ants-as-a-Dorigo-Birattari/058c6c7c37fb6970d322aad4a46c43b1cac0bf66) authored by Marco Dorigo, Mauro Birattari, and Thomas Stützle.

Activate using the `solve with ant-colony` command.

Parameter names list in the form of `variable_name: type = default_value`: 

`number_of_ants: u32 = 50` 
- number of ants that are going to be released onto the graph

`pheromone_constant: f32 = 1.0` 
- constant that is used in "pheromone laid on path" calculation

`pheromone_evaporation_rate: f32 = 0.2` 
- rate at which all pheromones evaporate after each ant-cycle, use values only from interval <0.0, 1.0>

`alpha: f32 = 1.0`
- controls the relative importance of the "pheromone laid on edge"

`beta: f32 = 4.0`
- controls the relative importance of the heuristic information

## License
Algorithmic-approaches-to-the-Traveling-Salesman-Problem is free and open source. All code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](docs/LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](docs/LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
