// keymap_definitions.rs

#![no_std]

mod keycodes;
//TOOD: put KEY in keycodes.rs ?
//use keycodes::{KEY};
mod keymap;
use keymap::{COL_SIZE, ROW_SIZE, LAYER_COUNT};

//TODO:

// KEY enum
#[derive(Debug, Clone, Copy)]
enum KEY {
    A, B, C, TRNS, MEDIA_VOL_UP, MEDIA_PAUSE, ESC, ENTER, BCKSPACE, DEL, SHIFT, CTRL, ALT, //TOOD: add more keys
}

// KEYMAP type, representing a 3D array of keys for each column of each row of each layer.
type KEYMAP = [[[Option<KEY>; COL_SIZE]; ROW_SIZE]; LAYER_COUNT];

// Macro to wrap the keys with Some(), used for key!(A) inside the KEYMAP for example
macro_rules! key {
    ($key:ident) => {
        Some(KEY::$key)
    };
}
