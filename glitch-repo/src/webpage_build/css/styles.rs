use crate::css_build_imports::*;

pub fn game_list_render_css(output_dir: &str) {
    let sass = r#"
    .container {
        width: 100%;
        max-width: 1200px;
        margin: 0 auto;
    }
    .header {
        background-color: #4CAF50;
        color: white;
        text-align: center;
        padding: 1em;
    }
    .content {
        padding: 1em;
    }
    .footer {
        background-color: #333;
        color: white;
        text-align: center;
        padding: 1em;
        position: absolute;
        bottom: 0;
        width: 100%;
    }
    "#;

    // Compile Sass to CSS with options
    let options = Options::default();
    let css = from_string(sass.to_string(), &options).expect("Failed to compile Sass");

    // Ensure the output directory exists
    std::fs::create_dir_all(output_dir).expect("Could not create output directory");

    // Construct the full path for the CSS file
    let css_file_path = Path::new(output_dir).join("styles.css");

    // Write CSS to the file
    let mut file = File::create(&css_file_path).expect("Could not create file");
    file.write_all(css.as_bytes()).expect("Could not write to file");

    println!("CSS file generated: {}", css_file_path.display());
}
