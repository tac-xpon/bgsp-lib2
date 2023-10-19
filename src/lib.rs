pub mod bgsp_common;
mod bg_resources;
pub mod bg_plane;
mod classic_sprite;
pub mod sp_resources;
mod texture_bank;

#[macro_export]
macro_rules! x {($e:expr) => { $e.0 }}
#[macro_export]
macro_rules! y {($e:expr) => { $e.1 }}
