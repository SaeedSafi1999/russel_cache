use crate::cache::Cache;
use crate::input::handle_input;

mod cache;
mod input;

fn main() {
    let cache = Cache::new();
    handle_input(&cache);
}
