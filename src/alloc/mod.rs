pub mod api;
pub mod immix;
mod blocks;
mod constants;

#[derive(Debug, PartialEq)]
pub enum BlockError {
    BadRequest,
    OutOfMemory,
}
