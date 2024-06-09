mod webpage_build_imports;
// use imports::*;

mod webpage_build {
    include!("webpage_build/game_list.rs");
}

fn main() {
    // Call the function from the included module
    webpage_build::render_template();
}