//! Helper functions for Telegram UI

/// Check if a string is empty or contains only whitespace
pub fn is_empty_or_whitespace(s: &str) -> bool {
    s.trim().is_empty()
}

/// Truncate a string to a maximum length with ellipsis
pub fn truncate_with_ellipsis(s: &str, max_length: usize) -> String {
    if s.len() <= max_length {
        s.to_string()
    } else {
        format!("{}...", &s[..max_length - 3])
    }
}

/// Convert snake_case to camelCase
pub fn to_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize = false;

    for c in s.chars() {
        if c == '_' {
            capitalize = true;
        } else if capitalize {
            result.push(c.to_ascii_uppercase());
            capitalize = false;
        } else {
            result.push(c);
        }
    }

    result
}

/// Convert snake_case to CSS class name
pub fn to_css_class(s: &str) -> String {
    s.replace('_', "-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_empty_or_whitespace() {
        assert!(is_empty_or_whitespace(""));
        assert!(is_empty_or_whitespace("   "));
        assert!(!is_empty_or_whitespace("hello"));
    }

    #[test]
    fn test_truncate_with_ellipsis() {
        assert_eq!(truncate_with_ellipsis("hello", 10), "hello");
        assert_eq!(truncate_with_ellipsis("hello world", 8), "hello...");
        assert_eq!(truncate_with_ellipsis("test", 4), "test");
    }

    #[test]
    fn test_to_camel_case() {
        assert_eq!(to_camel_case("hello_world"), "helloWorld");
        assert_eq!(to_camel_case("test"), "test");
        assert_eq!(to_camel_case("a_b_c"), "aBC");
    }

    #[test]
    fn test_to_css_class() {
        assert_eq!(to_css_class("hello_world"), "hello-world");
        assert_eq!(to_css_class("test-case"), "test-case");
    }
}
