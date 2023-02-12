use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}
pub fn mean(numbers: &[f32]) -> f32 {
    let mean = numbers.iter().sum::<f32>() / numbers.len() as f32;
    return mean.floor();
}

pub fn standard_deviation(numbers: &[f32]) -> f32 {
    let mean = mean(numbers);
    let deviation_squared = numbers.iter().map(|&x| (x - mean).powf(2.0)).sum::<f32>();
    let deviation = (deviation_squared / (numbers.len() - 1) as f32).sqrt();
    if (deviation > 0.0) {
        return deviation.floor();
    } else {
        return 0.0;
    }
}
