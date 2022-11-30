#![allow(warnings, clippy::all)]
pub mod opengl;

#[cfg(windows)]
pub mod wgl;

#[cfg(windows)]
pub mod winapi;

#[cfg(unix)]
pub mod glx;
