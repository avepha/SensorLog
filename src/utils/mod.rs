pub fn bool_to_string(flag: bool) -> String {
    if flag {
        String::from("true")
    } else {
        String::from("false")
    }
}
