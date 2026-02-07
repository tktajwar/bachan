use std::net::IpAddr;
use std::hash::{DefaultHasher, Hash, Hasher};


pub fn hashed(ip: IpAddr) -> u32 {
    let mut hasher = DefaultHasher::new();

    ip.hash(&mut hasher);
    crate::SECRET_NUMBER.hash(&mut hasher);

    hasher.finish() as u32
}
