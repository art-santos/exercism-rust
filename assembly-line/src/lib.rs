// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {

    pub const HOURLY_PRODUCED_CARS: f64 = 221.0;

    fn success_rate(speed: f64) -> f64{
        if speed as f64 >= 1.0 && speed as f64 <= 4.0 {
            1.0
        }
        else if speed as f64 >= 5.0 && speed as f64 <=8.0 {
            0.9
        }
        else if speed as f64 >= 8.0 && speed as f64 <=10.0 {
            0.77
        }else{
            0.0
        }
    }

    return HOURLY_PRODUCED_CARS * speed as f64 * success_rate(speed as f64) as f64
}

pub fn working_items_per_minute(speed: u8) -> u32 {

    pub const MINUTE_PRODUCED_CARS: f64 = 221.0/60.0;

    fn success_rate(speed: f64) -> f64{
        if speed as f64 >= 1.0 && speed as f64 <= 4.0 {
            1.0
        }
        else if speed as f64 >= 5.0 && speed as f64 <=8.0 {
            0.9
        }
        else if speed as f64 >= 8.0 && speed as f64 <=10.0 {
            0.77
        }else{
            0.0
        }
    }

    return (MINUTE_PRODUCED_CARS * success_rate(speed as f64) * speed as f64).floor() as u32;
}
