#[derive(Debug, Clone)]
pub enum Line {
    Context(String),
    Addition(String),
    Deletion(String),
}

#[derive(Debug, Clone)]
pub struct Hunk {
    pub old_start: usize,
    pub new_start: usize,
    pub lines: Vec<Line>,
}

#[derive(Debug, Clone)]
pub struct FileDiff {
    pub filename: String,
    pub hunks: Vec<Hunk>,
}

#[derive(Debug, Clone)]
pub struct Diff {
    pub files: Vec<FileDiff>,
}

pub fn parse_diff(diff_text: &str) -> Diff {
    let mut files = Vec::new();
    let mut current_file: Option<FileDiff> = None;
    let mut current_hunk: Option<Hunk> = None;

    for line in diff_text.lines() {
        // Skip diff header lines that shouldn't be displayed
        if line.starts_with("diff --git") {
            if let Some(file) = current_file.take() {
                files.push(file);
            }
            // Extract filename from "diff --git a/file b/file"
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let filename = parts[2].trim_start_matches("a/").to_string();
                current_file = Some(FileDiff {
                    filename,
                    hunks: Vec::new(),
                });
            }
        } else if line.starts_with("index ")
            || line.starts_with("--- ")
            || line.starts_with("+++ ")
            || line.trim().is_empty()
        {
            // Skip these header lines
            continue;
        } else if line.starts_with("@@") {
            if let Some(file) = current_file.as_mut() {
                if let Some(hunk) = current_hunk.take() {
                    file.hunks.push(hunk);
                }
                // Parse @@ -old_start,old_count +new_start,new_count @@
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let old_range = parts[1].trim_start_matches('-');
                    let new_range = parts[2].trim_start_matches('+');
                    let old_start: usize = old_range
                        .split(',')
                        .next()
                        .unwrap_or("0")
                        .parse()
                        .unwrap_or(0);
                    let new_start: usize = new_range
                        .split(',')
                        .next()
                        .unwrap_or("0")
                        .parse()
                        .unwrap_or(0);
                    current_hunk = Some(Hunk {
                        old_start,
                        new_start,
                        lines: Vec::new(),
                    });
                }
            }
        } else if let Some(hunk) = current_hunk.as_mut() {
            if line.starts_with(' ') {
                hunk.lines.push(Line::Context(line[1..].to_string()));
            } else if line.starts_with('+') {
                hunk.lines.push(Line::Addition(line[1..].to_string()));
            } else if line.starts_with('-') {
                hunk.lines.push(Line::Deletion(line[1..].to_string()));
            }
        }
    }

    if let Some(file) = current_file.take() {
        if let Some(hunk) = current_hunk.take() {
            let mut f = file;
            f.hunks.push(hunk);
            files.push(f);
        } else {
            files.push(file);
        }
    }

    Diff { files }
}
