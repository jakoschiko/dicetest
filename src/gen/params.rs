/// These parameters configure the random value generation of generators.
/// They will be passed to `GenOnce::gen_once` or `Gen::gen`.
#[derive(Debug, Clone)]
pub struct Params {
    // The maxmimum size of generated dynamic data structures.
    //
    // This parameter exists because the hardware of the testing machine is limited. For example
    // a very big list could not fit in the memory or its generation could take too much time.
    //
    // The generator implementation is allowed to freely interpret this value, but the
    // complexity of the value generation must be in `O(size)`.
    pub size: u64
}

impl Default for Params {
    fn default() -> Self {
        Params {
            size: 100,
        }
    }
}
