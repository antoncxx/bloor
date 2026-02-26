use std::fmt;
use std::io;
use std::net::IpAddr;
use std::str;
use std::str::FromStr;

use crate::common;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub enum PfSenseAliasHostValue {
    Address(IpAddr),
    FQDN(String),
    AliasName(String),
}

impl TryFrom<&str> for PfSenseAliasHostValue {
    type Error = io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(ip) = IpAddr::from_str(value) {
            return Ok(Self::Address(ip));
        }

        if common::utils::is_valid_hostname(value) {
            return Ok(Self::FQDN(value.to_owned()));
        }

        if super::utils::is_valid_alias_name(value) {
            Ok(Self::AliasName(value.to_owned()))
        } else {
            let message = format!("{value} is not a valid host address, FQDN or alias.");
            Err(io::Error::new(io::ErrorKind::InvalidInput, message))
        }
    }
}

impl fmt::Display for PfSenseAliasHostValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PfSenseAliasHostValue::Address(ip) => write!(f, "{ip}"),
            PfSenseAliasHostValue::FQDN(fqdn) => write!(f, "{fqdn}"),
            PfSenseAliasHostValue::AliasName(alias) => write!(f, "{alias}"),
        }
    }
}

impl str::FromStr for PfSenseAliasHostValue {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_ip() {
        let ip = "192.168.1.1";
        let value = PfSenseAliasHostValue::try_from(ip).unwrap();
        match value {
            PfSenseAliasHostValue::Address(a) => assert_eq!(a.to_string(), ip),
            _ => panic!("Expected Address variant"),
        }
    }

    #[test]
    fn test_try_from_fqdn() {
        let fqdn = "example.com";
        let value = PfSenseAliasHostValue::try_from(fqdn).unwrap();
        match value {
            PfSenseAliasHostValue::FQDN(name) => assert_eq!(name, fqdn),
            _ => panic!("Expected FQDN variant"),
        }
    }

    #[test]
    fn test_try_from_alias() {
        let alias = "my_alias_1";
        let value = PfSenseAliasHostValue::try_from(alias).unwrap();
        match value {
            PfSenseAliasHostValue::AliasName(a) => assert_eq!(a, alias),
            _ => panic!("Expected AliasName variant"),
        }
    }

    #[test]
    fn test_invalid_value() {
        let invalid = "invalid@host!";
        let result = PfSenseAliasHostValue::try_from(invalid);
        assert!(result.is_err());
    }
}
