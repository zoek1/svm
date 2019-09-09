/// Holds settings for using the runtime.
pub struct Opts {
    /// maximum pages required by the contract pages storage
    pub max_pages: usize,

    /// maximum pages required by the contract page-cache slice
    pub max_pages_slices: usize,
}
