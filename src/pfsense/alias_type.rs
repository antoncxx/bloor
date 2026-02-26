use std::fmt;
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AliasType {
    Host,
    Network,
    Port,
    UrlIps,
    UrlPorts,
    UrlTableIps,
    UrlTablePorts,
}

impl TryFrom<&str> for AliasType {
    type Error = io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "host" => Ok(Self::Host),
            "network" => Ok(Self::Network),
            "port" => Ok(Self::Port),
            "url" => Ok(Self::UrlIps),
            "url_ports" => Ok(Self::UrlPorts),
            "urltable" => Ok(Self::UrlTableIps),
            "urltable_ports" => Ok(Self::UrlTablePorts),
            other => {
                let message = format!("Value {other} is an invalid alias type");
                Err(io::Error::new(io::ErrorKind::InvalidInput, message))
            }
        }
    }
}

impl fmt::Display for AliasType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            AliasType::Host => "host",
            AliasType::Network => "network",
            AliasType::Port => "port",
            AliasType::UrlIps => "url",
            AliasType::UrlPorts => "url_ports",
            AliasType::UrlTableIps => "urltable",
            AliasType::UrlTablePorts => "urltable_ports",
        };

        write!(f, "{}", s)
    }
}
