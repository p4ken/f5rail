pub mod api;
pub mod app;

pub trait BveMap {
    fn create() -> Self;
}
