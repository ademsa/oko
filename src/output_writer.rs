use std::io::{Result, Write};

use owo_colors::{AnsiColors, OwoColorize};

use crate::output_destination::OutputDestination;
use crate::output_format::OutputFormat;
use crate::output_style::OutputStyle;

pub struct OutputWriter<'a> {
    pub writer: Box<dyn Write + 'a>,
    pub destination: OutputDestination,
    pub format: OutputFormat,
    pub style: Option<OutputStyle>,
}

impl<'a> OutputWriter<'a> {
    pub fn new(
        sink: Box<dyn Write + 'a>,
        destination: OutputDestination,
        format: OutputFormat,
        style: Option<OutputStyle>,
    ) -> Self {
        Self {
            writer: sink,
            destination,
            format,
            style,
        }
    }

    pub fn write_content(&mut self, content: &String) {
        if self.style.is_none() {
            self.write(content, None).unwrap();
        } else {
            self.write(content, self.style.as_ref().unwrap().content_color)
                .unwrap();
        }
    }

    pub fn write_match(&mut self, content: &String) {
        if self.style.is_none() {
            self.write(content, None).unwrap();
        } else {
            self.write(content, self.style.as_ref().unwrap().match_color)
                .unwrap();
        }
    }

    fn write(&mut self, content: &String, color: Option<AnsiColors>) -> Result<()> {
        if self.destination == OutputDestination::Standard && !color.is_none() {
            write!(self.writer, "{}", content.color(color.unwrap())).unwrap();
        } else {
            write!(self.writer, "{}", content).unwrap();
        }
        Ok(())
    }
}
