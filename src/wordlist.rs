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