use std::io::Read;

mod dupe_counter;
pub use dupe_counter::*;

mod skip_nth;
pub use skip_nth::*;

mod consecutive_overlapping_pairs;
pub use consecutive_overlapping_pairs::*;

mod tilemap;
pub use tilemap::*;

mod point;
pub use point::*;

pub fn read_entire_stdin() -> String {
    let mut buf = String::new();
    std::io::stdin()
        .read_to_string(&mut buf)
        .expect("unable to read stdin");
    buf
}
