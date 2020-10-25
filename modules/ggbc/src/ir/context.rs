/// Ir-generation context.
///
/// (In a future refactor it should allow for more O(N)-ish Ir code generation)
#[derive(Default)]
pub struct Context {
    fn_level: usize,
}

impl Context {
    pub fn is_main(&self) -> bool {
        self.fn_level == 0
    }

    pub fn begin_fn(&mut self) {
        self.fn_level += 1;
    }

    pub fn end_fn(&mut self) {
        self.fn_level -= 1
    }
}
