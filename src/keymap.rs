#![no_std]

// Import the definitions from the keymap_definitions.rs file
mod keymap_definitions;
use keymap_definitions::{KEY, KEYMAP};

// ----- DEFINE KEYMAP HERE BELOW -----

// Define constants for the size of the keymap
const COL_SIZE: usize = 8;
const ROW_SIZE: usize = 3;
const LAYER_COUNT: usize = 3;

// Define your keymaps for each layer, row, and column
const KEYMAP_LAYOUT: KEYMAP = [
    // Layer 0
    [
        [key!(A), key!(B), key!(C), key!(A), key!(B), key!(C), key!(A), key!(B)],
        [key!(B), key!(C), key!(A), key!(B), key!(C), key!(A), key!(B), key!(C)],
        [key!(C), key!(A), key!(B), key!(C), key!(A), key!(B), key!(C), None],
    ],
    // Layer 1
    [
        [key!(A), key!(B), key!(C), key!(A), key!(B), key!(C), key!(A), key!(B)],
        [key!(B), key!(C), key!(A), key!(B), key!(C), key!(A), key!(B), key!(C)],
        [key!(C), key!(A), key!(B), key!(C), key!(A), key!(B), None, None],
    ],
    // Layer 2
    [
        [key!(A), key!(B), key!(C), key!(A), key!(B), key!(C), key!(A), key!(B)],
        [key!(B), key!(C), key!(A), key!(B), key!(C), key!(A), key!(B), key!(C)],
        [key!(C), key!(A), key!(B), key!(C), key!(A), key!(B), key!(C), key!(A)],
    ],
];

// -------- ^^^^^^^^^^^^^^^^^^ --------


/*
//TEST debug

// Function to get the key at a specific layer, row, and column
fn get_key(layer: usize, row: usize, col: usize) -> KEY {
    if let Some(row_data) = KEYMAP_LAYOUT.get(layer) {
        if let Some(row) = row_data.get(row) {
            if let Some(&key) = row.get(col) {
                return key;
            }
        }
    }
    None
}

// Example usage of the keymap
fn main() {
    let active_layer = 1; // Select Layer 1 (0-indexed)
    let row = 2;   // Select Row 3 (0-indexed)
    let col = 7;   // Select Column 8 (0-indexed)

    if let Some(key) = get_key(active_layer, row, col) {
        println!("Key pressed: {:?}", key);
    } else {
        println!("No key at this position.");
    }
}
*/
