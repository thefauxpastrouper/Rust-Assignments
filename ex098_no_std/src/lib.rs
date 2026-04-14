#![no_std] // Always no_std by default

// Pull in alloc for collections when not using std
#[cfg(not(feature = "std"))]
extern crate alloc;

// Re-link std when the "std" feature is enabled (since #![no_std] unlinks it)
#[cfg(feature = "std")]
extern crate std;

// Define a "collection" alias based on the feature
pub mod collections {
    #[cfg(not(feature = "std"))]
    pub use alloc::collections::BTreeMap as HashMap;
    
    #[cfg(feature = "std")]
    pub use std::collections::HashMap;
}

use collections::HashMap;

pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

// This function only exists if "std" is enabled
#[cfg(feature = "std")]
pub fn use_hashmap() -> HashMap<i32, i32> {
    let mut hashmap = HashMap::new();
    hashmap.insert(1, 2);
    hashmap
}

// This works on BOTH because of the alias above
pub fn create_lookup() -> HashMap<u32, &'static str> {
    let mut map = HashMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    map
}
