extern crate pest_fmt;
use std::panic;
pub fn fmt(
    formatter: &pest_fmt::Settings,
    user: &String,
) -> Result<String, std::boxed::Box<dyn std::any::Any + std::marker::Send>> {
    panic::catch_unwind(|| formatter.format(user))
}
