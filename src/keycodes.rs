// check https://gist.github.com/MightyPork/6da26e382a7ad91b5496ee55fdc73db2
// or https://github.com/TeXitoi/keyberon/blob/45b8810e50e87e63adb8629f96778a86affda507/src/key_code.rs

// ?
mod keymap_definitions;
use keymap_definitions::KEY;
// or put this here? :
// KEY enum
#[derive(Debug, Clone, Copy)]
enum KEY {
    A, B, C, TRNS, MEDIA_VOL_UP, MEDIA_PAUSE, ESC, ENTER, BCKSPACE, DEL, SHIFT, CTRL, ALT, //TODO: add more keys
}
