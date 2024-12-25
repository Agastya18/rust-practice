use pulldown_cmark::{html, Options, Parser};
use std::fs;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Specify the input and output file paths
    let input_path = "input.md"; // Your input Markdown file
    let output_path = "./output.html"; // Output HTML file

    println!("Converting {} ", input_path);
    
    // Read the Markdown file
    let markdown_input = fs::read_to_string(input_path)?;
    
    // Set up options (e.g., enable strikethrough)
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);

    // Create a parser with the specified options
    let parser = Parser::new_ext(&markdown_input, options);
    
    // Prepare to collect HTML output
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Write the HTML output to a file
    let mut output_file = fs::File::create(output_path)?;
    write!(output_file, "{}", html_output)?;

    println!("Converted {} to {}", input_path, output_path);
    Ok(())
}
