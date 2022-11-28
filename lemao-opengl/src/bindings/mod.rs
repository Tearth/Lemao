#![allow(warnings, clippy::all)]
pub mod opengl;

#[cfg(windows)]
pub mod wgl;

#[cfg(unix)]
pub mod glx;
