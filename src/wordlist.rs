use std::fs;
use std::path::Path;
use std::fmt::Formatter;
use std::fmt;

// Custom array type that holds 5 raw ASCII characters, represented as u8 values (a-z, lowercase)
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Word([u8; 5]);

impl Word {
    pub fn new(bytes: [u8; 5]) -> Option<Self> {
        // Check for Word validity: all characters must be in ascii lowercase
        if bytes.iter().all(u8::is_ascii_lowercase) {
            Some(Self(bytes))
        } else {
            None
        }
    }

    pub fn as_str(&self) -> &str {
         std::str::from_utf8(&self.0).unwrap()
    }

    pub fn as_bytes(&self) -> &[u8; 5] {
        &self.0
    }
}

impl std::fmt::Display for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

pub fn load_word_list<P: AsRef<Path>>(path: P) -> Result<Vec<Word>, String> {

    // Obtains a &Path reference from any type implementing AsRef<Path>
    let path = path.as_ref();
    // Read the file path contents from the provided path, outputting custom error if invalid
    let contents = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {e}", path.display()))?;

    // Read words are stored in vector array words
    let mut words: Vec<Word> = Vec::new();
    for (line_no, line) in contents.lines().enumerate() {
        // Trim trailing and leading blank space from and word
        let line = line.trim();
        // Skip word if its just blank space
        if line.is_empty() {
            continue
        }

        let bytes: [u8; 5] = line
            // Converts line into an array of ascii byte characters
            .as_bytes()
            // Lines might be lengthier than 5 characters, i.e. fallible type conversion justifies try_into()
            .try_into()
            // If an error occurs, map_err transmutes the error into a custom one
            .map_err(|_| {
                format!(
                    "{}:{}: Expected a 5 letter word, got {:?}",
                    path.display(),
                    line_no + 1,
                    line
                )
            })?;
        
        // Normalizes byte array by converting all letters to lowercase
        let bytes = bytes.map(|b| b.to_ascii_lowercase());
        
        // Attempt to convert byte array into Word struct
        let word = Word::new(bytes).ok_or_else(|| {
            format!(
                "{}:{}: invalid word {:?}",
                path.display(),
                line_no + 1,
                line
            )
        })?;

        words.push(word);
    }

    Ok(words)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_temp(name: &str, contents: &str) -> std::path::PathBuf {
        // Grabs the OS' temporary directory and appends the desired file name
        let path = std::env::temp_dir().join(name);
        // Creates a file with the desired name in the temporary directory
        let mut f = fs::File::create(&path).unwrap();
        // Writes the desired content to the file 
        f.write_all(contents.as_bytes()).unwrap();
        // Returns the final file path
        path
    }

    #[test]
    fn loads_valid_words() {
        let path = write_temp("wordlist_test_valid.txt", "crane\nmauri\n\nghoul\n");
        let words = load_word_list(&path).unwrap();
        assert_eq!(words.len(), 3);
        assert_eq!(words[0].as_str(), "crane");
        let _ = fs::remove_file(path);
    }

    #[test]
    fn rejects_too_long() {
        let path = write_temp("wordlist_test_bad.txt", "toolong\n");
        assert!(load_word_list(&path).is_err());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn rejects_too_short() {
        let path = write_temp("wordlist_test_bad.txt", "hi\n");
        assert!(load_word_list(&path).is_err());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn normalizes_uppercase() {
        let path = write_temp("wordlist_test_upper.txt", "CRANE\n");
        let words = load_word_list(&path).unwrap();
        assert_eq!(words[0].as_str(), "crane");
        let _ = fs::remove_file(path);
    }

    #[test]
    fn word_new_rejects_non_lowercase() {
        assert!(Word::new(*b"CRANE").is_none());
        assert!(Word::new(*b"cr4ne").is_none());
        assert!(Word::new(*b"crane").is_some());
    }

    #[test]
    fn load_rejects_non_ascii() {
        // "café" is technically 5 bytes so would only fail when attempting to convert into a Word type
        let path = write_temp("wordlist_emoji.txt", "café\n");
        assert!(load_word_list(&path).is_err());
        let _ = fs::remove_file(path);
    }
}