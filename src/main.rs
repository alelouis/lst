extern crate hifitime;
extern crate chrono;
extern crate divrem;

use hifitime::{Epoch};
use chrono::prelude::*;
use std::str::FromStr;
use std::{thread, time};

fn deg_hms(deg:f64) -> (f64, f64, f64) {
    let time = (deg % 360.) * 24. / 360.;
    let gmst_h = time.floor();
    let gmst_m = (time*60.) % 60.;
    let gmst_s = (time*3600.) % 60.;
    (gmst_h, gmst_m, gmst_s)
}

fn main() {
    let sl = time::Duration::from_millis(100);

    loop {
        let utc_string: String = Utc::now().format("%Y-%m-%dT%H:%M:%S%.6f UTC").to_string();
        print!("\x1B[2J\x1B[1;1H");
        println!("UTC String:\t{}",utc_string);
    
        let utc: Epoch = Epoch::from_str(&utc_string).unwrap();
        let jd_utc: f64 = utc.as_jde_utc_days();
        println!("Julian Date:\t{:?}", jd_utc);
    
        let pi = std::f64::consts::PI;
        let t_u = jd_utc - 2451545.0; // https://en.wikipedia.org/wiki/Universal_Time
        let era_rad = 2.*pi*(0.7790572732640 + 1.00273781191135448 * t_u);
        let era_deg = era_rad * 180. / pi;
        println!("ERA Deg:\t{}", era_deg % 360.);
    
        let era_d = (era_deg % 360.).floor();
        let era_h = era_deg % 360. / 15.;
        let era_m = (era_deg*3600. / 60.) % 60.;
        let era_s = era_deg*3600. % 60.;
        println!("Earth Rotation:\t{} {:0>2.0}:{:0>2.0}:{:0>6.3}", era_d, era_h, era_m, era_s);

        let gmst = era_deg.clone();
        let (gmst_h, gmst_m, gmst_s) = deg_hms(gmst);
        println!("GMST (appro.):\t{:0>2.0}:{:0>2.0}:{:0>2.0}", gmst_h, gmst_m, gmst_s);

        let toulouse_long = 1.4442;
        let toulouse_st = gmst + toulouse_long;
        let (gmst_h, gmst_m, gmst_s) = deg_hms(toulouse_st);
        println!("TLS (appro.):\t{:0>2.0}:{:0>2.0}:{:0>2.0}", gmst_h, gmst_m, gmst_s);
    
        thread::sleep(sl);
    }
}
