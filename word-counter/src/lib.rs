use std::io::{self, BufRead, BufReader, Read, Write};

#[derive(Debug)]
pub struct ReaderCounts {
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
}

impl ReaderCounts {
    pub fn print_conditional(&self, lines: bool, words: bool, chars: bool) {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        self.write_conditional(lines, words, chars, &mut handle)
            .expect("failed to write counts");
    }

    fn write_conditional<W: Write>(
        &self,
        lines: bool,
        words: bool,
        chars: bool,
        writer: &mut W,
    ) -> io::Result<()> {
        let print_all = (lines && words && chars) || (!lines && !words && !chars);

        if print_all || lines {
            writeln!(writer, "lines: {}", self.lines)?;
        }

        if print_all || words {
            writeln!(writer, "words: {}", self.words)?;
        }

        if print_all || chars {
            writeln!(writer, "chars: {}", self.chars)?;
        }

        Ok(())
    }
}

pub fn get_counts<T: Read>(reader: T) -> Result<ReaderCounts, std::io::Error> {
    let mut lines: usize = 0;
    let mut words: usize = 0;
    let mut chars: usize = 0;

    let buf = BufReader::new(reader);
    for line in buf.lines() {
        let line = line?;
        lines += 1;
        words += line.split_whitespace().count();
        chars += line.chars().count() + 1; // +1 for stripped newline.
    }

    Ok(ReaderCounts {
        lines,
        words,
        chars,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn capture_output(counts: &ReaderCounts, lines: bool, words: bool, chars: bool) -> String {
        let mut buffer = Vec::new();
        counts
            .write_conditional(lines, words, chars, &mut buffer)
            .unwrap();
        String::from_utf8(buffer).unwrap()
    }

    #[test]
    fn test_get_counts() {
        let input = "Hello, world!\nThis is a test.\n";
        let counts = get_counts(input.as_bytes()).unwrap();
        assert_eq!(counts.lines, 2);
        assert_eq!(counts.words, 6);
        assert_eq!(counts.chars, 30);
    }

    #[test]
    fn test_get_counts_empty() {
        let input = "";
        let counts = get_counts(input.as_bytes()).unwrap();
        assert_eq!(counts.lines, 0);
        assert_eq!(counts.words, 0);
        assert_eq!(counts.chars, 0);
    }

    #[test]
    fn test_get_counts_with_blank_lines_and_whitespace() {
        let input = "First line\n\nSecond   line\twith tabs\n  \n";
        let counts = get_counts(input.as_bytes()).unwrap();
        assert_eq!(counts.lines, 4);
        assert_eq!(counts.words, 6);
        assert_eq!(counts.chars, 39);
    }

    #[test]
    fn test_get_counts_without_trailing_newline() {
        let input = "Line one\nLine two";
        let counts = get_counts(input.as_bytes()).unwrap();
        assert_eq!(counts.lines, 2);
        assert_eq!(counts.words, 4);
        assert_eq!(counts.chars, 18);
    }

    #[test]
    fn test_get_counts_with_unicode_characters() {
        let input = "😀 emoji\ncafé\n";
        let counts = get_counts(input.as_bytes()).unwrap();
        assert_eq!(counts.lines, 2);
        assert_eq!(counts.words, 3);
        assert_eq!(counts.chars, 13);
    }

    #[test]
    fn test_print_conditional_prints_all_when_no_flags() {
        let counts = ReaderCounts {
            lines: 3,
            words: 5,
            chars: 8,
        };
        let output = capture_output(&counts, false, false, false);
        assert_eq!(output, "lines: 3\nwords: 5\nchars: 8\n");
    }

    #[test]
    fn test_print_conditional_respects_individual_flags() {
        let counts = ReaderCounts {
            lines: 10,
            words: 20,
            chars: 30,
        };
        let output = capture_output(&counts, true, false, true);
        assert_eq!(output, "lines: 10\nchars: 30\n");
    }

    #[test]
    fn test_print_conditional_only_words() {
        let counts = ReaderCounts {
            lines: 1,
            words: 2,
            chars: 3,
        };
        let output = capture_output(&counts, false, true, false);
        assert_eq!(output, "words: 2\n");
    }
}
