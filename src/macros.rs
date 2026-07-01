#[macro_export]
macro_rules! char_len {
    ($x:expr) => {
        $x.chars().collect::<Vec<_>>().len()
    };
}
