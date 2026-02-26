pub fn is_valid_alias_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_alias() {
        assert!(is_valid_alias_name("alias_123"));
        assert!(is_valid_alias_name("A_B_C"));
    }

    #[test]
    fn test_invalid_alias() {
        assert!(!is_valid_alias_name("alias-name"));
        assert!(!is_valid_alias_name("alias!"));
        assert!(!is_valid_alias_name(""));
    }
}
