use std::fmt;
use std::io;
use std::str;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub enum PfSenseAliasType {
    Host,
    Network,
    Port,
    UrlIps,
    UrlPorts,
    UrlTableIps,
    UrlTablePorts,
}

impl TryFrom<&str> for PfSenseAliasType {
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

impl fmt::Display for PfSenseAliasType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Host => "host",
            Self::Network => "network",
            Self::Port => "port",
            Self::UrlIps => "url",
            Self::UrlPorts => "url_ports",
            Self::UrlTableIps => "urltable",
            Self::UrlTablePorts => "urltable_ports",
        };

        write!(f, "{}", s)
    }
}

impl str::FromStr for PfSenseAliasType {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}
