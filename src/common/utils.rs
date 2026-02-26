pub fn is_valid_hostname(hostname: &str) -> bool {
    let hostname = hostname.strip_suffix('.').unwrap_or(hostname);

    if hostname.is_empty() || hostname.len() > 253 || !hostname.contains('.') {
        return false;
    }

    for label in hostname.split('.') {
        let len = label.len();
        if len == 0 || len > 63 {
            return false;
        }

        let bytes = label.as_bytes();
        if !bytes[0].is_ascii_alphanumeric() || !bytes[len - 1].is_ascii_alphanumeric() {
            return false;
        }

        if !bytes
            .iter()
            .all(|&b| b.is_ascii_alphanumeric() || b == b'-')
        {
            return false;
        }
    }

    let tld = hostname.rsplit('.').next().unwrap();
    !tld.bytes().all(|b| u8::is_ascii_digit(&b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_hostnames() {
        assert!(is_valid_hostname("example.com"));
        assert!(is_valid_hostname("sub.example.com"));
        assert!(is_valid_hostname("my-host.example.co.uk"));
        assert!(is_valid_hostname("example.com."));
        assert!(is_valid_hostname("xn--nxasmq6b.com"));
        assert!(is_valid_hostname("a1.b2.c3"));
    }

    #[test]
    fn test_invalid_hostnames() {
        assert!(!is_valid_hostname(""));
        assert!(!is_valid_hostname("localhost"));
        assert!(!is_valid_hostname("-invalid.com"));
        assert!(!is_valid_hostname("invalid-.com"));
        assert!(!is_valid_hostname("invalid..com"));
        assert!(!is_valid_hostname("invalid_host.com"));
        assert!(!is_valid_hostname("192.168.1.1"));
        assert!(!is_valid_hostname(&("a".repeat(64).to_owned() + ".com")));
        assert!(!is_valid_hostname(&("a.".repeat(128) + "com")));
    }
}
