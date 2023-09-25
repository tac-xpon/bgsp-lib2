pub mod bgsp_common;
mod bg_lib;
mod bg_resources;
pub mod bg_plane;
mod sp_lib;
mod classic_sprite;
pub mod sp_resources;
mod sp_texture_bank;

#[macro_export]
macro_rules! x {($e:expr) => { $e.0 }}
#[macro_export]
macro_rules! y {($e:expr) => { $e.1 }}
