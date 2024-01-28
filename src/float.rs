use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug, Default, Clone, Copy)]
pub struct F32(pub f32);

/// This works like `PartialEq` on `f32`, except that `NAN == NAN` is true.
impl PartialEq for F32 {
    fn eq(&self, other: &Self) -> bool {
        if self.0.is_nan() && other.0.is_nan() {
            true
        } else {
            self.0 == other.0
        }
    }
}

impl Eq for F32 {}

/// This works like `PartialOrd` on `f32`, except that `NAN` sorts below all other floats
/// (and is equal to another NAN). This always returns a `Some`.
impl PartialOrd for F32 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// This works like `PartialOrd` on `f32`, except that `NAN` sorts below all other floats
/// (and is equal to another NAN).
impl Ord for F32 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap_or_else(|| {
            if self.0.is_nan() && !other.0.is_nan() {
                Ordering::Less
            } else if !self.0.is_nan() && other.0.is_nan() {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        })
    }
}

impl Hash for F32 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.0.is_nan() {
            0x7fc00000u32.hash(state); // a particular bit representation for NAN
        } else if self.0 == 0.0 {
            // catches both positive and negative zero
            0u32.hash(state);
        } else {
            self.0.to_bits().hash(state);
        }
    }
}

impl From<F32> for f32 {
    fn from(f: F32) -> Self {
        f.0
    }
}

impl From<f32> for F32 {
    fn from(f: f32) -> Self {
        Self(f)
    }
}

impl fmt::Display for F32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl F32 {
    pub fn round(&self) -> Self {
        Self(self.0.round())
    }
}
