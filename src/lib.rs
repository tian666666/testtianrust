
//! # testtian
//! `testtian` is a test lib crate, for modeling artistic concepts.

/// Adds one to the number given
///

/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = art::add_one(arg);
///
/// assert_eq!(6,answer);
pub fn add_one(x:i32) -> i32 {
    x + 1
}

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor,c2: PrimaryColor) -> SecondaryColor {
        //
        let sec = SecondaryColor::Green;
        sec
    }
}

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
