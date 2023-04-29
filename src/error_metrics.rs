/// A trait that provides methods for calculating different types of error metrics
/// between two sets of values.
pub trait ErrorMetrics<T: AsRef<[f64]>> {
    /// Calculates the root mean square deviation (RMSE) between two sets of values.
    /// Returns `None` if the two sets of values have different lengths.
    fn root_mean_square_dev(&self, other: &T) -> Option<f64>;

    /// Calculates the mean squared error (MSE) between two sets of values.
    /// Returns `None` if the two sets of values have different lengths.
    fn mean_squared_error(&self, other: &T) -> Option<f64>;

    /// Calculates the mean absolute error (MAE) between two sets of values.
    /// Returns `None` if the two sets of values have different lengths.
    fn mean_absolute_error(&self, other: &T) -> Option<f64>;
}

impl<T: AsRef<[f64]>> ErrorMetrics<T> for T {
    fn root_mean_square_dev(&self, other: &T) -> Option<f64> {
        if self.as_ref().len() != other.as_ref().len() {
            return None;
        }

        let sum_of_squares =
            self.as_ref()
                .iter()
                .zip(other.as_ref().iter())
                .fold(0.0, |acc, (s, o)| {
                    let diff = s - o;
                    acc + diff * diff
                });

        Some((sum_of_squares / self.as_ref().len() as f64).sqrt())
    }

    fn mean_squared_error(&self, other: &T) -> Option<f64> {
        if self.as_ref().len() != other.as_ref().len() {
            return None;
        }

        if self.as_ref().is_empty() {
            return None;
        }

        let sum = self
            .as_ref()
            .iter()
            .zip(other.as_ref().iter())
            .fold(0.0, |sum, (s, o)| {
                let diff = s - o;
                sum + (diff * diff)
            });

        Some(sum / (self.as_ref().len() as f64))
    }

    fn mean_absolute_error(&self, other: &T) -> Option<f64> {
        if self.as_ref().len() != other.as_ref().len() {
            return None;
        }

        if self.as_ref().is_empty() {
            return None;
        }

        let sum = self
            .as_ref()
            .iter()
            .zip(other.as_ref().iter())
            .fold(0.0, |sum, (s, o)| sum + (s - o).abs());

        Some(sum / (self.as_ref().len() as f64))
    }
}
