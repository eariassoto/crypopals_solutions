pub trait Rmse<T: AsRef<[f64]>> {
    fn root_mean_square_dev(&self, other: &T) -> Option<f64>;
}

impl<T: AsRef<[f64]>> Rmse<T> for T {
    fn root_mean_square_dev(&self, other: &T) -> Option<f64> {
        if self.as_ref().len() != other.as_ref().len() {
            return None;
        }

        let sum_of_squares =
            other
                .as_ref()
                .iter()
                .zip(self.as_ref().iter())
                .fold(0.0, |acc, (&exp, &act)| {
                    let diff = exp - act;
                    acc + diff * diff
                });
        let rmse = (sum_of_squares / self.as_ref().len() as f64).sqrt();
        Some(rmse)
    }
}
