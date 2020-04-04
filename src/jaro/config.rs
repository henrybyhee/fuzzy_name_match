pub struct JaroWinklerConfigOptions {
    pub similarity_threshold: f64,
    pub max_prefix_length: i32,
    pub scaling_factor: f64,
}

impl JaroWinklerConfigOptions {
    pub fn default() -> JaroWinklerConfigOptions {
        JaroWinklerConfigOptions {
            similarity_threshold: 0.7,
            max_prefix_length: 4,
            scaling_factor: 0.1,
        }
    }

    pub fn new(
        similarity_threshold: f64,
        max_prefix_length: i32,
        scaling_factor: f64,
    ) -> JaroWinklerConfigOptions {
        JaroWinklerConfigOptions {
            similarity_threshold,
            max_prefix_length,
            scaling_factor,
        }
    }
}
