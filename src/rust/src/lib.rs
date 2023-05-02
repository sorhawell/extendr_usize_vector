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

trait MySpaceTrait {
    //pub fn new() -> Self

    fn subset(&self, idx: i32) -> String;
    fn length(&self) -> i32;
    fn to_text(&self) -> Vec<String>;
    fn data_id(&self) -> String; //should be replaced by an DataType-like enum
}

impl std::fmt::Display for Box<dyn MySpaceTrait> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let x: std::fmt::Result = self
            .as_ref()
            .to_text()
            .iter()
            .map(|s| write!(f, "{}, ", s))
            .collect();
        x
    }
}

impl std::fmt::Debug for Box<dyn MySpaceTrait> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x: std::fmt::Result = self
            .as_ref()
            .to_text()
            .iter()
            .map(|s| write!(f, "{}, ", s))
            .collect();
        x
    }
}

//macrorules could write write all these implementations
impl MySpaceTrait for Vec<f64> {
    fn subset(&self, idx: i32) -> String {
        format!("f64 value: [{}]", self[idx as usize])
    }
    fn length(&self) -> i32 {
        self.len() as i32
    }
    fn to_text(&self) -> Vec<String> {
        self.iter().map(|val| val.to_string()).collect()
    }
    fn data_id(&self) -> String {
        "Vec<f64>".to_string()
    }
}
impl MySpaceTrait for Vec<i32> {
    fn subset(&self, idx: i32) -> String {
        format!("i32 value: [{}]", self[idx as usize])
    }
    fn length(&self) -> i32 {
        self.len() as i32
    }
    fn to_text(&self) -> Vec<String> {
        self.iter().map(|val| val.to_string()).collect()
    }
    fn data_id(&self) -> String {
        "Vec<i32>".to_string()
    }
}

#[derive(Debug)]
struct MySpace(pub Box<dyn MySpaceTrait>);

#[extendr]
impl MySpace {
    pub fn new(robj: Robj) -> MySpace {
        match robj.rtype() {
            Rtype::Integers => MySpace(Box::new(robj.as_integer_vector().unwrap())),
            Rtype::Doubles => MySpace(Box::new(robj.as_real_vector().unwrap())),
            _ => todo!("not implemented input"),
        }
    }

    fn subset(&self, idx: i32) -> String {
        self.0.subset(idx)
    }
    fn length(&self) -> i32 {
        self.0.length()
    }
    fn to_text(&self) -> Vec<String> {
        self.0.to_text()
    }
    fn data_id(&self) -> String {
        self.0.data_id()
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod helloextendr;
    fn hello_world;
    impl MySpace;
    impl VecUsize;
}
