use std::env;
use std::fs;

pub fn read_input(file_name: &str) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd.join("input").join(file_name);
    fs::read_to_string(filepath).expect("Unable to open input file")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_input() {
        let input = read_input("test.txt");
        assert_eq!(input.lines().next().unwrap(), "I am a test")
    }

}