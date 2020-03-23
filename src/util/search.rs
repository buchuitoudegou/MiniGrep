pub fn search_str(query: &String, content: &String) -> Vec<String> {
    let mut result = vec![];
    for line in content.as_str().lines() {
        if line.contains(query.as_str()) {
            result.push(String::from(line));
        }
    }
    result
}