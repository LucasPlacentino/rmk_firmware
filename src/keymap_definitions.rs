// keymap_definitions.rs

#![no_std]

//TODO:

// Define the KEY enum with different keyboard keys
#[derive(Debug, Clone, Copy)]
enum KEY {
    A, B, C, TRNS, MEDIA_VOL_UP, MEDIA_PAUSE, ESC, ENTER, BCKSPACE, DEL, SHIFT, CTRL, ALT,/* ... add more keys as needed */
}

// Define the KEYMAP type, representing a 3D array of keys for each layer, row, and column
type KEYMAP = [[[Option<KEY>; COL_SIZE]; ROW_SIZE]; LAYER_COUNT];

// Macro to wrap the keys with Some()
macro_rules! k {
    ($key:ident) => {
        Some(KEY::$key)
    };
}
