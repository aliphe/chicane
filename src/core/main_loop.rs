pub trait MainLoop {
    fn next(&mut self) -> bool;
}
