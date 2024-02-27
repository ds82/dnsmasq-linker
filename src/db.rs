//
// Read data from lease file and process it
//
//
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct LeaseEntry {
    pub last_change: String,
    pub mac: String,
    pub ip: String,
    pub name: String,
    pub raw: String,
}

impl FromStr for LeaseEntry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();

        fn _error(key: &str) -> anyhow::Error {
            anyhow::anyhow!("Error parsing {}", key)
        }

        let lease = LeaseEntry {
            last_change: parts.get(0).ok_or(_error("last_change"))?.to_string(),
            mac: parts.get(1).ok_or(_error("mac"))?.to_string(),
            ip: parts.get(2).ok_or(_error("ip"))?.to_string(),
            name: parts
                .get(3)
                .ok_or(_error("name"))?
                .to_string()
                .to_lowercase(),
            raw: s.to_string(),
        };

        Ok(lease)
    }
}

pub fn read_lease_file<S: Into<String>>(uri: S) -> Result<Vec<String>> {
    std::fs::read_to_string(uri.into())?
        .lines()
        .map(|line| Ok(line.to_string()))
        .collect()
}

pub fn transform_lease_file(leases: Vec<String>) -> Result<Vec<LeaseEntry>> {
    leases
        .into_iter()
        .map(|s| LeaseEntry::from_str(s.as_str()))
        .collect::<Result<Vec<_>>>()
        .map(|mut entries| {
            entries.sort_unstable_by(|a, b| a.name.cmp(&b.name));
            entries
        })
}

pub fn read<S: Into<String>>(uri: S) -> Result<Vec<LeaseEntry>> {
    read_lease_file(uri).and_then(transform_lease_file)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lease_entry_from_str_1() {
        let example = "1704179285 90:2b:34:5e:34:2d 10.0.4.3 draka ff:34:5e:34:2d:00";

        let lease = LeaseEntry::from_str(example).unwrap();

        assert_eq!(lease.last_change, "1704179285");
        assert_eq!(lease.mac, "90:2b:34:5e:34:2d");
        assert_eq!(lease.ip, "10.0.4.3");
        assert_eq!(lease.name, "draka");
        assert_eq!(lease.raw, example);
    }
}
