/// The maxmimum size of generated dynamic data structures.
///
/// It will be passed to `GenOnce::gen_once` or `Gen::gen`. The generator implementation is
/// allowed to freely interpret this value, but the complexity of the value generation
/// should be in `O(size)`.
///
/// This parameter exists because the hardware of the testing machine is limited. For example
/// a very big list could not fit in the memory or its generation could take too much time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Size(pub u64);

impl Default for Size {
    fn default() -> Self {
        Size(100)
    }
}