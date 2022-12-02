use std::fs;

fn build_input_path(day: i8) -> String {
    let base_path = "./input/";
    let mut file = String::from(base_path);
    file.push_str("day");
    let mut day_str = day.to_string();
    if day_str.len() < 2 {
        day_str.insert(0, '0');
    }
    file.push_str(day_str.as_str());
    file.push_str(".txt");
    return file;
}

pub fn get_input(day: i8) -> String {
    let file = build_input_path(day);
    let contents = fs::read_to_string(file);
    return contents.expect("Could not find File");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_input_path() {
        assert_eq!(String::from("./input/day01.txt"), build_input_path(1));
        assert_eq!(String::from("./input/day22.txt"), build_input_path(22));
    }

    #[test]
    fn test_get_input() {
        let content = get_input(1);
        assert!(content.len() > 10);
    }
}