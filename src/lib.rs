use std::io::Write;

use anyhow::{Context, Result};

pub fn find_matches(pattern: &str, content: &str, mut writer: impl Write) -> Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).context("writing match failed")?;
        }
    }
    Ok(())
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem", "lorem ipsum\ndolor sit amet", &mut result).unwrap();
    assert_eq!(result, b"lorem ipsum\n");
}
