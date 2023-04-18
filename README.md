# Algorithmic-approaches-to-the-Traveling-Salesman-Problem

In this project i want to try and implement few algorithms used for finding the shortest path through hamiltonian cycles in complete, weighted, undirected graphs, more popularly know as the Traveling-Salesman-Problem.

The whole project is build using the [bevy](https://github.com/bevyengine/bevy) game engine.

# Ant-Colony-Optimization

The first algorithm i choose to implement is the ant-colony-optimization. My code follows the standard pseudocode with equations mentioned in [this paper](https://www.semanticscholar.org/paper/Ant-colony-optimization%3A-artificial-ants-as-a-Dorigo-Birattari/058c6c7c37fb6970d322aad4a46c43b1cac0bf66) authored by Marco Dorigo, Mauro Birattari, and Thomas Stützle. In app when you have constructed your graph you can solve it with this approach by writing "compute using ant-colony" into the in-app console.
