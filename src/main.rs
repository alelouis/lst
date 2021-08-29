extern crate hifitime;
extern crate chrono;

mod lst;

fn main() {
    let tls_lon = 1.4333;
    let utc_string = lst::utc_str();
    let jd_ut1 = lst::jd(&utc_string);
    let gmst = lst::era(jd_ut1);
    let tls_lst = lst::lst_at_lon(tls_lon, gmst);
    lst::print_lst(tls_lst);
}
