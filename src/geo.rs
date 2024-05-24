use std::fmt::Display;

pub struct LatLng {
    lat: f64,
    lng: f64,
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

impl Display for LatLng {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_dms())
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

pub struct Rect {
    nw: LatLng,
    se: LatLng,
}
