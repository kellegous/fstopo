use std::{error::Error, fmt, str::FromStr};

use serde::{de, ser, Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct LatLng {
    pub lat: f64,
    pub lng: f64,
}

impl LatLng {
    pub fn new(lat: f64, lng: f64) -> LatLng {
        LatLng { lat, lng }
    }

    pub fn to_dms(&self) -> String {
        let (lat_d, lat_m, lat_s) = to_dms(self.lat);
        let (lng_d, lng_m, lng_s) = to_dms(self.lng);
        format!(
            "{:02}°{:02}′{:02}″{} {:03}°{:02}′{:02}″{}",
            lat_d,
            lat_m,
            lat_s,
            if self.lat < 0.0 { 'S' } else { 'N' },
            lng_d,
            lng_m,
            lng_s,
            if self.lng < 0.0 { 'W' } else { 'E' }
        )
    }
}

impl fmt::Display for LatLng {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_dms())
    }
}

impl FromStr for LatLng {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(
            r#"(\d+)°(\d+)[′'](\d+)[″"]([NSns]) (\d+)°(\d+)[′'](\d+)[″"]([EWew])"#,
        )
        .unwrap();

        let caps = re.captures(s).ok_or(format!("invalid location: {}", s))?;

        let lat_d = caps.get(1).unwrap().as_str().parse::<i32>()?;
        let lat_m = caps.get(2).unwrap().as_str().parse::<i32>()?;
        let lat_s = caps.get(3).unwrap().as_str().parse::<i32>()?;
        let lat_v = lat_d as f64 + (lat_m as f64) / 60.0 + (lat_s as f64) / 3600.0;
        let lat_v = match caps.get(4).unwrap().as_str() {
            "N" | "n" => Ok(lat_v),
            "S" | "s" => Ok(-lat_v),
            _ => Err(format!("invalid location: {}", s)),
        }?;

        let lng_d = caps.get(5).unwrap().as_str().parse::<i32>()?;
        let lng_m = caps.get(6).unwrap().as_str().parse::<i32>()?;
        let lng_s = caps.get(7).unwrap().as_str().parse::<i32>()?;
        let lng_v = lng_d as f64 + (lng_m as f64) / 60.0 + (lng_s as f64) / 3600.0;
        let lng_v = match caps.get(8).unwrap().as_str() {
            "E" | "e" => Ok(lng_v),
            "W" | "w" => Ok(-lng_v),
            _ => Err(format!("invalid location: {}", s)),
        }?;

        Ok(Self {
            lat: lat_v,
            lng: lng_v,
        })
    }
}

impl ser::Serialize for LatLng {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let v = format!("{}", self);
        s.serialize_str(&v)
    }
}

impl<'d> de::Deserialize<'d> for LatLng {
    fn deserialize<D>(de: D) -> Result<LatLng, D::Error>
    where
        D: de::Deserializer<'d>,
    {
        de.deserialize_str(LatLngVisitor)
    }
}

struct LatLngVisitor;

impl<'d> de::Visitor<'d> for LatLngVisitor {
    type Value = LatLng;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        LatLng::from_str(v).map_err(|e| E::custom(e.to_string()))
    }
}

fn to_dms(v: f64) -> (i32, i32, i32) {
    let v = v.abs();

    let mut d = v as i32;

    let v = v - d as f64;

    let mut m = (v * 60.0) as i32;

    let v = v - m as f64 / 60.0;

    let mut s = (v * 3600.0).round() as i32;

    if s == 60 {
        s = 0;
        m += 1;
    }

    if m == 60 {
        m = 0;
        d += 1;
    }

    (d, m, s)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rect {
    pub nw: LatLng,
    pub se: LatLng,
}

impl Rect {
    pub fn new(nw: LatLng, se: LatLng) -> Self {
        Self { nw, se }
    }

    pub fn from_arg(s: &str) -> Result<Self, String> {
        s.parse().map_err(|_| format!("invalid rect: {}", s))
    }
}

impl FromStr for Rect {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (nw, se) = s
            .split_once('-')
            .ok_or(format!("invalid geo rect: {}", s))?;

        let nw = nw.parse::<LatLng>()?;
        let se = se.parse::<LatLng>()?;

        Ok(Self { nw, se })
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.nw, self.se)
    }
}
