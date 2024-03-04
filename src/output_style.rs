use owo_colors::AnsiColors;

pub struct OutputStyle {
    pub line_number: bool,
    pub content_color: Option<AnsiColors>,
    pub match_color: Option<AnsiColors>,
}

impl OutputStyle {
    pub fn new(
        line_number: bool,
        content_color: Option<AnsiColors>,
        match_color: Option<AnsiColors>,
    ) -> Self {
        Self {
            line_number,
            content_color,
            match_color,
        }
    }
}
