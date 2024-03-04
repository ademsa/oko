use serde_json::{json, to_string};

use crate::output_format::OutputFormat;
use crate::output_writer::OutputWriter;
use crate::search_results::SearchResults;

/// Write search results
pub fn write_search_results(results: SearchResults, writer: &mut OutputWriter) {
    match writer.format {
        OutputFormat::Plain {} => {
            for result in results.results {
                // Line number
                if !writer.style.is_none() && writer.style.as_ref().unwrap().line_number {
                    writer.write_content(&format!("{}: ", result.line.to_string()));
                }

                // Write content before and matches
                let mut prev_m_end_pos = 0usize;
                for m in result.matches {
                    let content_before =
                        &result.content.as_str()[prev_m_end_pos..m.start_pos].to_string();
                    prev_m_end_pos = m.end_pos;

                    writer.write_content(&content_before);
                    writer.write_match(&m.content);
                }

                // Write remaining content
                let remaining_content = &result.content.as_str()[prev_m_end_pos..].to_string();
                writer.write_content(&remaining_content);

                // Close the line
                writer.write_content(&"\n".to_string())
            }
        }
        OutputFormat::Json {} => {
            writer.write_content(&to_string(&results).unwrap());
        }
    }
}

///Write count results
pub fn write_count_results(results: String, writer: &mut OutputWriter) {
    match writer.format {
        OutputFormat::Plain {} => {
            writer.write_content(&results);
        }
        OutputFormat::Json {} => {
            let content_json = json!({
                "results": &results
            });
            writer.write_content(&to_string(&content_json).unwrap());
        }
    }
}
