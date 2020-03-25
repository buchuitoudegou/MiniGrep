pub fn search_str<'a>(query: &'a String, content: &'a String) -> Vec<&'a str> {
    content.lines().filter(|line| line.contains(query)).collect()
}