use ::rng::Rng;

/// The parameters for evaluating a property several time.
#[derive(Debug, Clone)]
pub struct EvalSeriesParams {
    /// The random number generator for calling `Prop::eval`.
    pub rng: Rng,
    /// The size of the first evaluation used for intializing `gen::Result::size`. The next
    /// evaluations use an interpolated size between `start_size` and `end_size`.
    pub start_size: u64,
    /// The size of the last evaluation used for intializing `gen::Result::size`. The previous
    /// evaluations use an interpolated size between `start_size` and `end_size`.
    pub end_size: u64,
    /// The upper limit for the number of property evaluations.
    pub min_passed: u64,
}
