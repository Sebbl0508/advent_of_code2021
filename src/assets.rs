use include_dir::{include_dir, Dir};

pub static ASSETS_FOLDER: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets");
