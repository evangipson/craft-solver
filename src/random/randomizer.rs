use rand::Rng;

/// Returns `success` if the `percent` is greater than a random percentage,
/// otherwise will return `default`.
pub fn if_more_than<T>(percent: f32, success: T, default: T) -> T {
    if rand::rng().random_range(0.0..100.0).gt(&percent) {
        success
    } else {
        default
    }
}
