// RMK firmware build script
// MIT License

use std::fs::File;
use std::io::Read;
use serde_json::{Value, from_str};
use std::env;

/*
put :
```
include!(concat!(env!("OUT_DIR"), "/hello.rs"));
```
or
```
// Include the generated KEYMAP from the build.rs script
include!(concat!(env!("OUT_DIR"), "/keyboard_layout.rs"));
```
inside main program

*/

fn main() {
    // TODO:

    /*
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("hello.rs");
    fs::write(
        &dest_path,
        "
        let HELLO = 1;
        pub fn message() -> &'static str {
            \"Hello, World!\"
        }
        "
    ).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    */
    
    // TEST:
    // Load the contents of the JSON file
    let mut file = File::open("keymap.json").expect("Failed to open keymap.json");
    let mut json_content = String::new();
    file.read_to_string(&mut json_content).expect("Failed to read keymap.json");

    // Parse the JSON data
    let data: Value = from_str(&json_content).expect("Failed to parse JSON");

    // Extract the required information from the JSON
    let layers = data["layers"].as_u64().expect("Invalid 'layers' value") as usize;
    
    //let max_rows = data["max_rows"].as_u64().expect("Invalid 'max_rows' value") as usize;
    //let max_columns = data["max_columns"].as_u64().expect("Invalid 'max_columns' value") as usize;
    let mut max_rows = 0;
    let mut max_columns = 0;

    for layer in 1..=layers {
        let layer_key = format!("layer{}", layer);
        let layer_data = &data[&layer_key];

        for row in 1..=3 {
            let row_key = format!("row{}", row);
            if let Some(row_data) = layer_data.get(&row_key) {
                let row_columns = row_data.as_array().expect("Invalid JSON format").len();
                if row_columns == 0 {
                    break; // Stop processing if there are no more rows
                }
                if row_columns > max_columns {
                    max_columns = row_columns;
                }
            } else {
                break; // Stop processing if the row doesn't exist
            }
        }
    }
    
    let layer1 = &data["layer1"];
    let layer2 = &data["layer2"];

    // Generate the keymaps for each layer, row, and column
    let mut keymap_layout: [[[Option<KEY>; COL_SIZE]; ROW_SIZE]; LAYER_COUNT] = Default::default();
    for layer in 0..layers {
        for row in 0..max_rows {
            let row_data = match layer {
                0 => &layer1[&format!("row{}", row + 1)],
                1 => &layer2[&format!("row{}", row + 1)],
                _ => panic!("Unsupported layer"),
            };

            for (col, key) in row_data.as_array().expect("Invalid JSON format").iter().enumerate() {
                let key_name = key.as_str().expect("Invalid JSON format");
                let key_enum = match key_name {
                    "None" => None,
                    _ => Some(KEY::from_str(key_name).expect("Invalid key at row:{} col:{}", row, col)),
                };
                keymap_layout[layer][row][col] = key_enum;
            }
        }
    }

    // Serialize the keymap_layout array into Rust code
    let serialized_keymap = format!(
        "pub const COL_SIZE: usize = {};\n\
         pub const ROW_SIZE: usize = {};\n\
         pub const LAYER_COUNT: usize = {};\n\n\
         pub type KEYMAP = [[[Option<KEY>; COL_SIZE]; ROW_SIZE]; LAYER_COUNT];\n\n\
         pub const KEYMAP_LAYOUT: KEYMAP = {generated_keymap};\n",
        max_columns, // + 1 too ?
        max_rows + 1, // needed ? // Add 1 to account for 0-indexed row numbers,
        layers, // + 1 too ?
        generated_keymap = keymap_layout
    );

    // Write the serialized keymap and constants into the 'keyboard_layout.rs' file
    let out_dir = env::var("OUT_DIR").expect("Failed to get output directory");
    let dest_path = format!("{}/keyboard_layout.rs", out_dir);
    let mut file = File::create(&dest_path).expect("Failed to create keyboard_layout.rs");
    file.write_all(serialized_keymap.as_bytes())
        .expect("Failed to write keyboard_layout.rs");

    /*
    // Serialize the keymap_layout array into Rust code
    let serialized_keymap = format!("{:?}", keymap_layout);

    // Write the serialized keymap into the 'keymap_layout.rs' file
    let out_dir = env::var("OUT_DIR").expect("Failed to get output directory");
    let keymap_layout_dest_path = format!("{}/keymap_layout.rs", out_dir);
    let mut keymap_layout_file = File::create(&keymap_layout_dest_path).expect("Failed to create keymap_layout.rs");
    keymap_layout_file.write_all(serialized_keymap.as_bytes()).expect("Failed to write keymap_layout.rs");

    // Write the keymap parameters into the 'keymap_definitions.rs' file
    let keymap_definitions_dest_path = format!("{}/keymap_definitions.rs", out_dir);
    let mut keymap_definitions_file = File::create(&keymap_definitions_dest_path).expect("Failed to create keymap_definitions.rs");
    let content = format!(
        "// keymap_definitions.rs\n\n\
         // Define the KEY enum with different keyboard keys\n\
         #[derive(Debug, Clone, Copy)]\n\
         enum KEY {{\n    A, B, C, \n}}\n\n\
         // Define the KEYMAP type, representing a 3D array of keys for each layer, row, and column\n\
         type KEYMAP = [[[Option<KEY>; {max_columns}]; {max_rows}]; {layers}];\n\n\
         // Define constants for the size of the keymap\n\
         const COL_SIZE: usize = {max_columns};\n\
         const ROW_SIZE: usize = {max_rows};\n\
         const LAYER_COUNT: usize = {layers};\n",
        max_columns = max_columns,
        max_rows = max_rows,
        layers = layers
    );
    keymap_definitions_file.write_all(content.as_bytes()).expect("Failed to write keymap_definitions.rs");
    */

    println!("cargo:rerun-if-changed=keymap.json");
    println!("cargo:rerun-if-changed=build.rs"); //? needed?
}


//? use ?
fn generate_keymap_from_json(data: &Value) -> String {
}
