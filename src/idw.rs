use num_traits::Float;

use crate::coord::Coord;

/// The `IDW` struct represents an Inverse Distance Weighting interpolator.
///
/// The weighted function used in the algorithm is `weightᵢ = 1 / distance(pointᵢ, position)ᵖ`.
///
/// You can transform the weights by setting a transform function by calling [`IDW::weighted_function`].
///
/// The default power parameter used in the algorithm is 2 and can be set by [`IDW::power`].
pub struct IDW<C, N>
where
    C: Coord<N>,
    N: Float,
{
    points: Vec<C>,
    values: Vec<N>,
    power_parameter: N,
    weighted_function: Option<Box<dyn Fn(N) -> N>>,
}

impl<C, N> IDW<C, N>
where
    C: Coord<N>,
    N: Float,
{
    /// Creates a new instance of the `IDW` struct.
    ///
    /// # Arguments
    ///
    /// - `points` - A vector of points.
    /// - `values` - A vector of values associated with each point.
    ///
    /// # Returns
    ///
    /// A new instance of the struct.
    ///
    /// # Panics
    ///
    /// - Points vector is empty.
    /// - Values vector is empty.
    /// - Points and values vectors have different length.
    pub fn new(points: Vec<C>, values: Vec<N>) -> Self {
        assert_ne!(points.len(), 0, "Points vector must not be empty.");
        assert_ne!(values.len(), 0, "Values vector must not be empty.");
        assert_eq!(
            points.len(),
            values.len(),
            "Points and values vectors must be the same length."
        );

        Self {
            points,
            values,
            power_parameter: N::from(2).unwrap(),
            weighted_function: None,
        }
    }

    /// Sets the custom power parameter used in the algorithm.
    ///
    /// # Arguments
    ///
    /// * `power` - The new power parameter value.
    ///
    /// # Returns
    ///
    /// The modified instance of the struct.
    pub fn power(mut self, power: N) -> Self {
        self.power_parameter = power;

        self
    }

    /// Sets the custom weighted function to be applied to the weights.
    ///
    /// # Arguments
    ///
    /// - `func` - A function that takes a weight and returns a new weight.
    ///
    /// # Returns
    ///
    /// The modified instance of the struct.
    pub fn weighted_function(mut self, func: impl Fn(N) -> N + 'static) -> Self {
        self.weighted_function = Some(Box::new(func));

        self
    }

    /// Calculates the interpolated value at a given position.
    ///
    /// # Arguments
    ///
    /// - `position` - The position to evaluate.
    ///
    /// # Returns
    ///
    /// The interpolated value at the given position.
    pub fn evaluate(&self, position: C) -> N {
        let weight_result = self
            .points
            .iter()
            .enumerate()
            .map(|(index, point)| {
                let distance = point.distance_to(&position);

                if distance.is_zero() {
                    return Err(index);
                }

                let weight = N::one() / distance.powf(self.power_parameter);

                Ok(weight)
            })
            .collect::<Result<Vec<N>, usize>>();

        return match weight_result {
            Ok(weights) => {
                let mut normalized_weights = normalize_weights(&weights);

                if let Some(func) = &self.weighted_function {
                    for weight in &mut normalized_weights {
                        *weight = func(*weight);
                    }

                    normalized_weights = normalize_weights(&normalized_weights);
                }

                normalized_weights
                    .iter()
                    .zip(&self.values)
                    .fold(N::zero(), |acc, (w, v)| acc + *w * *v)
            }
            Err(index) => self.values[index],
        };

        fn normalize_weights<N>(weights: &[N]) -> Vec<N>
        where
            N: Float,
        {
            let weight_sum = weights.iter().fold(N::zero(), |acc, w| acc + *w);

            weights.iter().map(|w| *w / weight_sum).collect::<Vec<N>>()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;

    #[test]
    #[should_panic]
    fn test_empty_points() {
        let points: Vec<f64> = vec![];
        let values = vec![1.0, 2.0];
        IDW::new(points, values);
    }

    #[test]
    #[should_panic]
    fn test_empty_values() {
        let points = vec![1.0, 2.0];
        let values = vec![];
        IDW::new(points, values);
    }

    #[test]
    #[should_panic]
    fn test_different_length() {
        let points = vec![1.0, 2.0];
        let values = vec![1.0];
        IDW::new(points, values);
    }

    #[test]
    fn test_power() {
        let points = vec![1.0, 2.0, 3.0];
        let values = vec![1.0, 2.0, 3.0];
        let idw = IDW::new(points, values).power(0.5);

        assert_relative_eq!(idw.evaluate(0.0), 1.814988, max_relative = 0.000001);
        assert_relative_eq!(idw.evaluate(1.0), 1.0);
        assert_relative_eq!(idw.evaluate(1.001), 1.072458, max_relative = 0.000001);
        assert_relative_eq!(idw.evaluate(1.5), 1.836013, max_relative = 0.000001);
        assert_relative_eq!(idw.evaluate(2.0), 2.0);
        assert_relative_eq!(idw.evaluate(2.5), 2.163986, max_relative = 0.000001);
        assert_relative_eq!(idw.evaluate(3.0), 3.0);
        assert_relative_eq!(idw.evaluate(4.0), 2.185011, max_relative = 0.000001);
    }

    #[test]
    fn test_weighted_function() {
        let points = vec![1.0, 2.0, 3.0];
        let values = vec![1.0, 2.0, 3.0];
        let idw = IDW::new(points, values)
            .weighted_function(|weight| (1.0 + (4.0 * PI * weight).sin()) * 0.5);

        assert_relative_eq!(idw.evaluate(0.0), 2.138717, max_relative = 0.000001);
        assert_relative_eq!(idw.evaluate(1.0), 1.0);
        assert_relative_eq!(idw.evaluate(1.001), 2.000006, max_relative = 0.000001);
        assert_relative_eq!(idw.evaluate(1.5), 2.316685, max_relative = 0.000001);
        assert_relative_eq!(idw.evaluate(2.0), 2.0);
        assert_relative_eq!(idw.evaluate(2.5), 1.683314, max_relative = 0.000001);
        assert_relative_eq!(idw.evaluate(3.0), 3.0);
        assert_relative_eq!(idw.evaluate(4.0), 1.861282, max_relative = 0.000001);
    }

    #[test]
    fn test_idw_1d() {
        let points = vec![1.0, 2.0, 3.0];
        let values = vec![1.0, 2.0, 3.0];
        let idw = IDW::new(points, values);

        assert_relative_eq!(idw.evaluate(0.0), 1.346938, max_relative = 0.000001);
        assert_relative_eq!(idw.evaluate(1.0), 1.0);
        assert_relative_eq!(idw.evaluate(1.001), 1.000001, max_relative = 0.000001);
        assert_relative_eq!(idw.evaluate(1.5), 1.578947, max_relative = 0.000001);
        assert_relative_eq!(idw.evaluate(2.0), 2.0);
        assert_relative_eq!(idw.evaluate(2.5), 2.421053, max_relative = 0.000001);
        assert_relative_eq!(idw.evaluate(3.0), 3.0);
        assert_relative_eq!(idw.evaluate(4.0), 2.653061, max_relative = 0.000001);
    }

    #[test]
    fn test_idw_2d() {
        let points = vec![(1.0, 1.0), (2.0, 2.0), (3.0, 3.0)];
        let values = vec![1.0, 2.0, 3.0];
        let idw = IDW::new(points, values);

        assert_relative_eq!(idw.evaluate((0.0, 0.0)), 1.346938, max_relative = 0.000001);
        assert_relative_eq!(idw.evaluate((1.0, 2.0)), 1.636363, max_relative = 0.000001);
        assert_relative_eq!(
            idw.evaluate((1.001, 0.009)),
            1.274519,
            max_relative = 0.000001
        );
        assert_relative_eq!(idw.evaluate((1.5, 2.5)), 2.0);
        assert_relative_eq!(idw.evaluate((2.0, 2.0)), 2.0);
        assert_relative_eq!(idw.evaluate((2.5, 1.5)), 2.0);
        assert_relative_eq!(idw.evaluate((3.0, 2.0)), 2.363636, max_relative = 0.000001);
        assert_relative_eq!(idw.evaluate((4.0, 4.0)), 2.653061, max_relative = 0.000001);
    }

    #[test]
    fn test_idw_3d() {
        let points = vec![(1.0, 1.0, 1.0), (2.0, 2.0, 2.0), (3.0, 3.0, 3.0)];
        let values = vec![1.0, 2.0, 3.0];
        let idw = IDW::new(points, values);

        assert_relative_eq!(
            idw.evaluate((0.0, 0.0, 0.0)),
            1.346938,
            max_relative = 0.000001
        );
        assert_relative_eq!(idw.evaluate((1.0, 2.0, 3.0)), 2.0);
        assert_relative_eq!(
            idw.evaluate((1.001, 0.009, 1.0)),
            1.229539,
            max_relative = 0.000001
        );
        assert_relative_eq!(
            idw.evaluate((1.5, 2.5, 1.5)),
            1.919732,
            max_relative = 0.000001
        );
        assert_relative_eq!(idw.evaluate((2.0, 2.0, 2.0)), 2.0);
        assert_relative_eq!(
            idw.evaluate((2.5, 1.5, 2.5)),
            2.080267,
            max_relative = 0.000001
        );
        assert_relative_eq!(idw.evaluate((3.0, 2.0, 1.0)), 2.0);
        assert_relative_eq!(
            idw.evaluate((4.0, 4.0, 4.0)),
            2.653061,
            max_relative = 0.000001
        );
    }
}
