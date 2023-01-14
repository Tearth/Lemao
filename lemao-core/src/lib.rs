#![allow(
    clippy::while_immutable_condition,
    clippy::not_unsafe_ptr_arg_deref,
    clippy::identity_op,
    clippy::too_many_arguments,
    clippy::needless_range_loop,
    clippy::uninlined_format_args
)]

pub mod audio;
pub mod renderer;
pub mod utils;
pub mod window;

pub use lemao_common_platform;
pub use lemao_math;
