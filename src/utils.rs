use cfg_if::cfg_if;

cfg_if! {
    // https://github.com/rustwasm/console_error_panic_hook#readme
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}
pub fn mean(numbers: &[f32]) -> f32 {
    numbers.iter().sum::<f32>() / numbers.len() as f32
}

pub fn standard_deviation(numbers: &[f32]) -> f32 {
    let mean = mean(numbers);
    let deviation_squared = numbers.iter().map(|&x| (x - mean).powf(2.0)).sum::<f32>();
    (deviation_squared / (numbers.len() - 1) as f32).sqrt()
}
