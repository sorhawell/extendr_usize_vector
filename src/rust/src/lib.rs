use std::slice;

use extendr_api::prelude::*;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

#[derive(Debug, Clone)]
struct VecUsize(pub Vec<Option<usize>>);

#[extendr]
impl VecUsize {
    pub fn new(robj: Integers) -> Self {
        let x = robj
            .iter()
            .map(|x| match &x {
                _ if x.is_na() => None,
                _ => Some(x.0 as usize),
            })
            .collect();
        VecUsize(x)
    }

    pub fn subset(&self, idx: Integers) -> Self {
        let vec: Vec<_> = idx
            .iter()
            .map(|i| match &i {
                _ if i.is_na() => None,
                _ if i.0 <= 0 || i.0 as usize > self.0.len() => None,
                _ => self.0[i.0 as usize - 1],
            })
            .collect();
        VecUsize(vec)
    }

    pub fn length(&self) -> i32 {
        self.0.len() as i32
    }

    pub fn to_text(&self) -> Vec<String> {
        self.0
            .iter()
            .map(|opt_usize| opt_usize.map(|val| val.to_string()).unwrap_or("NA".into()))
            .collect()
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod helloextendr;
    fn hello_world;
    impl VecUsize;
}
