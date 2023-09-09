# inverse_distance_weight

[![Latest version](https://img.shields.io/crates/v/inverse_distance_weight.svg)](https://crates.io/crates/inverse_distance_weight)
[![Documentation](https://docs.rs/inverse_distance_weight/badge.svg)](https://docs.rs/inverse_distance_weight)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)

An implementation of the [Inverse Distance Weighting (IDW)](https://en.wikipedia.org/wiki/Inverse_distance_weighting) algorithm.

The crate supports points of 1 to 3 dimension to perform the interpolation.

The weighted function used in the algorithm is `weightᵢ = 1 / distance(pointᵢ, position)ᵖ`.

# Examples

```rust
use inverse_distance_weight::IDW;

// 1 dimension
let points = vec![0.0, 1.0];
let values = vec![0.0, 1.0];
let idw = IDW::new(points, values);

let result = idw.evaluate(0.5);

// 2 dimension
let points = vec![(0.0, 0.0), (1.0, 1.0)];
let values = vec![0.0, 1.0];
let idw = IDW::new(points, values);

let result = idw.evaluate((0.5, 0.5));

// 3 dimension
let points = vec![(0.0, 0.0, 0.0), (1.0, 1.0, 1.0)];
let values = vec![0.0, 1.0];
let idw = IDW::new(points, values);

let result = idw.evaluate((0.5, 0.5, 0.5));

// Customize
let points = vec![0.0, 1.0];
let values = vec![0.0, 1.0];
let idw = IDW::new(points, values)
    // Sets a power parameter. Default is 2.
    .power(0.5)
    // Sets a transform function for weights.
    .weighted_function(|weight| (1.0 + (4.0 * std::f64::consts::PI * weight).sin()) * 0.5);
```