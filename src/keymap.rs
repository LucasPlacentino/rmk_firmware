#![no_std]

// Import the definitions from the keymap_definitions.rs file
mod keymap_definitions;
use keymap_definitions::{KEY, KEYMAP};

// Define constants for the size of the keymap
const COL_SIZE: usize = 8;
const ROW_SIZE: usize = 3;
const LAYER_COUNT: usize = 3;

// Define your keymaps for each layer, row, and column
const KEYMAP_LAYOUT: KEYMAP = [
    // Layer 0
    [
        [KEY::A, KEY::B, KEY::C, KEY::A, KEY::B, KEY::C, KEY::A, KEY::B],
        [KEY::B, KEY::C, KEY::A, KEY::B, KEY::C, KEY::A, KEY::B, KEY::C],
        [KEY::C, KEY::A, KEY::B, KEY::C, KEY::A, KEY::B, KEY::C, KEY::A],
    ],
    // Layer 1
    [
        [KEY::A, KEY::B, KEY::C, KEY::A, KEY::B, KEY::C, KEY::A, KEY::B],
        [KEY::B, KEY::C, KEY::A, KEY::B, KEY::C, KEY::A, KEY::B, KEY::C],
        [KEY::C, KEY::A, KEY::B, KEY::C, KEY::A, KEY::B, KEY::C, KEY::A],
    ],
    // Layer 2
    [
        [KEY::TRNS, KEY::TRNS, KEY::TRNS, KEY::TRNS, KEY::TRNS, KEY::TRNS, KEY::TRNS, KEY::TRNS],
        [KEY::B, KEY::C, KEY::A, KEY::B, KEY::C, KEY::A, KEY::B, KEY::C],
        [KEY::C, KEY::A, KEY::B, KEY::C, KEY::A, KEY::B, KEY::C, KEY::A],
    ],
];

// -------------------------------------------------------------------------


/*
// Define the KEY enum with different keyboard keys
#[derive(Debug, Clone, Copy)]
enum KEY {
    A, B, C, TRNS, NULL // ... add more keys as needed
}

// Define the KEYMAP type, representing a 3D array of keys for each layer, row, and column
type KEYMAP = [[[KEY; COL_SIZE]; ROW_SIZE]; LAYER_COUNT];

// Function to get the key at a specific layer, row, and column
fn get_key(layer: usize, row: usize, col: usize) -> KEY {
    KEYMAP_LAYOUT[layer][row][col]
}

// Example usage of the keymap
fn main() {
    let layer = 1; // Select Layer 1
    let row = 2;   // Select Row 3 (0-indexed)
    let col = 3;   // Select Column 4 (0-indexed)

    let key = get_key(layer, row, col);
  
    //debug:
    //println!("Key pressed: {:?}", key);
}
*/
