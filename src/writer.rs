use std::fs::File;
use std::io::{stdout, Result};
use std::path::PathBuf;

use minigreplib::output_destination::OutputDestination;
use owo_colors::AnsiColors;

use minigreplib::output_format::OutputFormat;
use minigreplib::output_style::OutputStyle;
use minigreplib::output_writer::OutputWriter;

pub fn get_writer<'a>(
    output_path: Option<PathBuf>,
    output_format: OutputFormat,
    line_number: bool,
    content_color: Option<AnsiColors>,
    match_color: Option<AnsiColors>,
) -> Result<OutputWriter<'a>> {
    if output_path.is_none() {
        // Get style
        let output_style = OutputStyle::new(line_number, content_color, match_color);

        let base_writer = Box::new(stdout());
        Ok(OutputWriter::new(
            base_writer,
            OutputDestination::Standard,
            output_format,
            Some(output_style),
        ))
    } else {
        let base_writer = Box::new(File::create(output_path.unwrap()).unwrap());
        Ok(OutputWriter::new(
            base_writer,
            OutputDestination::File,
            output_format,
            None,
        ))
    }
}
