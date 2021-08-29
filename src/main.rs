use std::rc::Rc;
use std::cell::RefCell;
use rand::seq::SliceRandom;

// Author: Trae Yelovich <trae@trae.is>
// Created during Summer 2021 for CS 1666, Roguelike project
// Usable under MIT license.

// NOTE: Specify your own grid size here!!!
pub const GRID_SIZE: usize = 8;

// Position to start path-carving within the grid
pub const START_X: i32 = 3;
pub const START_Y: i32 = 3;

struct RecursiveBacktracker {
    cells: [Vec<Rc<RefCell<bool>>>; GRID_SIZE],
    max_iterations: Option<usize>,
}

impl RecursiveBacktracker {
    // Create a new instance of the RecursiveBacktracker structure
    pub fn new( max_iterations: Option<usize> ) -> RecursiveBacktracker {
        // Initialize cells as default
        let mut cells : [Vec<Rc<RefCell<bool>>>; GRID_SIZE as usize] = Default::default();
        for y in 0 .. GRID_SIZE {
            for _x in 0 .. GRID_SIZE {
                // Create a ref-counted shared pointer to mutably borrow visited flag 
                // without making the path-carving function mutable
                cells[ y ].push( Rc::new( RefCell::new( false ) ) );
            }
        }

        RecursiveBacktracker { cells, max_iterations, }
    }

    // Change max iterations, if desired.
    pub fn change_max_iterations( &mut self, max_iter: Option< usize > )
    {
        self.max_iterations = max_iter;
    }

    pub fn valid_cell( &self, x: i32, y: i32 ) -> bool {
        // Is the cell in a valid position in the grid?
        // x = { x | x >= 0 && x < GRID_SIZE - 1 }
        // y = { y | y >= 0 && y < GRID_SIZE - 1 }

        x < self.cells.len() as i32 && y < self.cells.len() as i32 && x >= 0 && y >= 0
    }

    pub fn carve_path( &self, x: i32, y: i32, layout: &mut Vec<(i32, i32)> ) {
        // If we have met the max amount of iterations for this
        // instance of the RecursiveBacktracker, stop carving and return.
        match self.max_iterations {
            Some( max ) => {
                if layout.len() >= max {
                    return;
                }
            },
            None => {},
        }

        let cell = &self.cells[ y as usize ][ x as usize ];
        // Mark the cell as visited and add it to the vector of valid grid positions
        *cell.borrow_mut() = true;
        layout.push( (x, y) );
        println!( "Visited cell at ({}, {})", x, y );

        // Pick a random direction to take in the grid.
        let mut rng = rand::thread_rng();
        let mut directions = vec![ (0, -1), (1, 0), (0, 1), (-1, 0) ];
        directions.shuffle( &mut rng );
        for direction in directions.iter() {
            // Check to see if the direction is valid before continuing
            let new_loc = (x + direction.0, y + direction.1);
            if self.valid_cell( new_loc.0, new_loc.1 ) {
                // If the cell is unvisited, move in this direction. (recursive call)
                let next_cell = &self.cells[ new_loc.1 as usize ][ new_loc.0 as usize ];
                if !*next_cell.borrow_mut() {
                    self.carve_path( new_loc.0, new_loc.1, layout );
                }
            }
        }

        // After this point, the function will return (backtracking to the last call)
        println!( "Backtracking..." );
    }

    pub fn run( &self ) {
        match self.max_iterations {
            Some( max ) => println!( "\nStarting Recursive Backtracker ({} iterations)", max ),
            None => println!( "\nStarting Recursive Backtracker" ),
        }
        
        let mut layout = Vec::new();

        // Begin carving the path
        self.carve_path( START_X, START_Y, &mut layout );
        println!( "Done" );

        // Print the path to the console
        print!( "Path: " );
        for p in layout.iter() {
            if p.0 == START_X && p.1 == START_Y {
                print!( "[Start] -> " );
            } else {
                print!( "[{}, {}] -> ", p.0, p.1 );
            }
        }
        println!( "[End]" );
    }

    // Resets the structure for another use.
    pub fn reset( &mut self ) {
        self.cells = Default::default();
        for y in 0 .. GRID_SIZE {
            for _x in 0 .. GRID_SIZE {
                self.cells[ y ].push( Rc::new( RefCell::new( false ) ) );
            }
        }
    }
}

// Example test of the Recursive Backtracker
// with a max iteration cap of 8
pub fn recursive_backtracker_test() {
    println!("Recursive Backtracker test!");
    let mut rb = RecursiveBacktracker::new( Some( 8 ) );
    rb.run();

    // Change to 10 iterations, reset and run once more
    rb.reset();
    rb.change_max_iterations( Some( 10 ) );
    rb.run();
}

// Entry-point
fn main() {
    recursive_backtracker_test();
}