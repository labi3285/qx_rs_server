pub fn round(val: f64, dec: u8) -> f64 {
    let factor = (10.0 as f64).powf(dec as f64);
    (val * factor).round() / factor
}

pub fn floor(val: f64, dec: u8) -> f64 {
    let factor = (10.0 as f64).powf(dec as f64);
    (val * factor).floor() / factor
}

pub fn ceil(val: f64, dec: u8) -> f64 {
    let factor = (10.0 as f64).powf(dec as f64);
    (val * factor).ceil() / factor
}