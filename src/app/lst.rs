use hifitime::{Epoch};
use chrono::prelude::*;
use std::str::FromStr;

pub fn deg_to_hms(deg:f64) -> (f64, f64, f64) {
    // Convert degrees to (hour, minutes, seconds)
    let time = (deg % 360.) * 24. / 360.;
    let gmst_h = time.floor();
    let gmst_m = (time*60.) % 60.;
    let gmst_s = (time*3600.) % 60.;
    (gmst_h, gmst_m, gmst_s)
}

pub fn utc_str() -> String {
    // Return UTC hour in String format
    let format = "%Y-%m-%dT%H:%M:%S%.6f UTC";
    let utc_string: String = Utc::now().format(format).to_string();
    utc_string
}

pub fn utc_str_simple() -> String {
    // Return UTC hour in String format
    let format = "%H:%M:%S";
    let utc_string: String = Utc::now().format(format).to_string();
    utc_string
}

pub fn jd(utc_string: &String) -> f64 {
    // Returns julian date for a given UTC String
    let utc: Epoch = Epoch::from_str(&utc_string).unwrap();
    let jd_utc: f64 = utc.as_jde_utc_days();
    jd_utc
}

pub fn era(jd_ut1: f64) -> f64{
    // Compute Earth Rotation Angle from julian date
    let t_u = jd_ut1 - 2451545.0;
    let era_rad = 2.*std::f64::consts::PI*(0.7790572732640 + 1.00273781191135448 * t_u);
    let era_deg = era_rad * 180. / std::f64::consts::PI;
    era_deg
}

pub fn lst_at_lon(lon: f64, gmst: f64) -> f64{
    // Compute local sidereal angle for a given longitude
    let lst = gmst + lon;
    lst
}
