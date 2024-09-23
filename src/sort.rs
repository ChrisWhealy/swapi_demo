use std::cmp::Ordering;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub type Sorter<T> = fn(s1: &T, s2: &T) -> std::cmp::Ordering;
pub fn compare_strs(s1: &str, s2: &str) -> Ordering {
    s1.to_ascii_lowercase().cmp(&s2.to_ascii_lowercase())
}
pub fn compare_strs_as_f64s(s1: &str, s2: &str) -> Ordering {
    str_to_f64(s1).total_cmp(&str_to_f64(s2))
}

pub fn str_to_f64(s: &str) -> f64 {
    s.replace("unknown", "")
        .replace("n/a", "")
        .replace(&[',', ' '], "")
        .parse::<f64>()
        .unwrap_or_else(|_| std::f64::INFINITY)
}
