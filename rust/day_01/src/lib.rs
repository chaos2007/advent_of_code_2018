fn string_to_int(word: String) -> i32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some_words() {
        assert_eq!(-1, string_to_int("-1".to_string()));
        assert_eq!(3, string_to_int("+3".to_string()));
    }
}
