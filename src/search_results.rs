use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SearchResults {
    pub pattern: String,
    pub results: Vec<SearchResult>,
}

impl SearchResults {
    pub fn new(pattern: String) -> Self {
        Self {
            pattern,
            results: vec![],
        }
    }

    pub fn add_result(&mut self, result: SearchResult) {
        self.results.push(result);
    }
}

impl PartialEq for SearchResults {
    fn eq(&self, other: &Self) -> bool {
        if self.pattern == other.pattern && self.results == other.results {
            return true;
        }
        return false;
    }
}

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub line: usize,
    pub content: String,
    pub matches: Vec<Match>,
}

impl SearchResult {
    pub fn new(line: usize, content: String) -> Self {
        Self {
            line,
            content,
            matches: vec![],
        }
    }

    pub fn add_match(&mut self, m: Match) {
        self.matches.push(m);
    }
}

impl PartialEq for SearchResult {
    fn eq(&self, other: &Self) -> bool {
        if self.line == other.line && self.content == other.content && self.matches == other.matches
        {
            return true;
        }
        return false;
    }
}

#[derive(Debug, Serialize)]
pub struct Match {
    pub content: String,
    pub start_pos: usize,
    pub end_pos: usize,
}

impl Match {
    pub fn new(content: String, start_pos: usize, end_pos: usize) -> Self {
        Self {
            content,
            start_pos,
            end_pos,
        }
    }
}

impl PartialEq for Match {
    fn eq(&self, other: &Self) -> bool {
        if self.content == other.content
            && self.start_pos == other.start_pos
            && self.end_pos == other.end_pos
        {
            return true;
        }
        return false;
    }
}
