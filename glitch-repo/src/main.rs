mod webpage_build_imports;
mod css_build_imports;

mod webpage_build {
    include!("webpage_build/html/game_list.rs");
}

mod webpage_css_build {
    include!("webpage_build/css/styles.rs");
}

fn css_build() {
    const OUTPUT_PATH: &str = "webpage/styles";
    webpage_css_build::game_list_render_css(OUTPUT_PATH);
}

fn html_build() {
    const OUTPUT_PATH: &str = "webpage";
    webpage_build::game_list_render(OUTPUT_PATH);
}

fn main() {
    // Build the CSS first just as a best practice.
    css_build();
    html_build();
}
