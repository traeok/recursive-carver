# recursive-backtracker
Recursive backtracker alg. w/ adjustable cap, written (safely) in Rust

## Setup
Simply add the RecursiveBacktracker struct, impl blocks, and constants to whatever project you're using.

### Constants
```START_X:``` Starting X position in the 2D grid\
```START_Y:``` Starting Y position in the 2D grid\
```GRID_SIZE:``` Size of the 2D grid (width and height)

Due to the initial requirements for this task, the algorithm runs with the assumption that the 2D grid is of equal width and height. 

## Usage
To **create an instance** of the structure, use ```RecursiveBacktracker::new()```\
To **run** the algorithm, use ```RecursiveBacktracker::run(&self)```\
To **reset** the structure, use ```RecursiveBacktracker::reset(&mut self)```\
To **change the max iterations** for a specific instance, use ```RecursiveBacktracker::change_max_iterations(&mut self, max_iter: Option<usize>)```
- Passing ```None``` to this function will allow the algorithm to run until all neighbors in the path are visited.
