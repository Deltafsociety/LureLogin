use maxminddb::{geoip2, Reader};
use std::net::IpAddr;

pub fn get_country(ip: &str) -> String {
    let reader = Reader::open_readfile("GeoLite2-City.mmdb");

    if reader.is_err() {
        return "Unknown".into();
    }

    let reader = reader.unwrap();

    let ip: IpAddr = match ip.parse() {
        Ok(i) => i,
        Err(_) => return "Unknown".into(),
    };

    let city: Result<geoip2::City, _> = reader.lookup(ip);

    match city {
        Ok(c) => c
            .country
            .and_then(|x| x.names)
            .and_then(|x| x.get("en").map(|s| s.to_string()))
            .unwrap_or_else(|| "Unknown".into()),
        Err(_) => "Unknown".into(),
    }
}
