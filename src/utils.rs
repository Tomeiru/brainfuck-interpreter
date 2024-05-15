use std::{fs::File, io::Read};

pub fn read_file(filename: &String) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    return Ok(content);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_existing_file() -> std::io::Result<()> {
        let existing_filepath = String::from("./test_files/existing_file.txt");
        let content = read_file(&existing_filepath)?;
        assert_eq!(
            content,
            String::from("I'm an existing file with content!\n")
        );
        Ok(())
    }
}
