pub fn reverse(input: &str) -> String {
    let mut res = "".to_string();
    for i in input.chars() {
        res.push(i);
    }
    res.to_string()
}

