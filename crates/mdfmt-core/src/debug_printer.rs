pub trait DebugPrinter {
    type Options;
    fn debug_print(&self, options: Self::Options) -> String;
}
