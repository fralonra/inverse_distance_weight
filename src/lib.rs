//! # inverse_distance_weight
//!
//! An implementation of the [Inverse Distance Weighting (IDW)](https://en.wikipedia.org/wiki/Inverse_distance_weighting) algorithm.
//!
//! The crate supports points of 1 to 3 dimension to perform the interpolation.
//!
//! The weighted function used in the algorithm is `weightᵢ = 1 / distance(pointᵢ, position)ᵖ`.
//!
//! You can transform the weights by setting a transform function by calling [`IDW::weighted_function`].
//!
//! The default power parameter used in the algorithm is 2 and can be set by [`IDW::power`].
//!
//! # Examples
//!
//! ```
//! use inverse_distance_weight::IDW;
//!
//! // 1 dimension
//! let points = vec![0.0, 1.0];
//! let values = vec![0.0, 1.0];
//! let idw = IDW::new(points, values);
//!
//! let result = idw.evaluate(0.5);
//!
//! // 2 dimension
//! let points = vec![(0.0, 0.0), (1.0, 1.0)];
//! let values = vec![0.0, 1.0];
//! let idw = IDW::new(points, values);
//!
//! let result = idw.evaluate((0.5, 0.5));
//!
//! // 3 dimension
//! let points = vec![(0.0, 0.0, 0.0), (1.0, 1.0, 1.0)];
//! let values = vec![0.0, 1.0];
//! let idw = IDW::new(points, values);
//!
//! let result = idw.evaluate((0.5, 0.5, 0.5));
//!
//! // Customize
//! let points = vec![0.0, 1.0];
//! let values = vec![0.0, 1.0];
//! let idw = IDW::new(points, values)
//!     .power(0.5)
//!     .weighted_function(|weight| (1.0 + (4.0 * std::f64::consts::PI * weight).sin()) * 0.5);
//! ```

mod coord;
mod idw;

pub use idw::IDW;

#[cfg(test)]
#[macro_use]
extern crate approx;
