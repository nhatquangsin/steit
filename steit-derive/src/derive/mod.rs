#[allow(clippy::module_inception)]
mod derive;
mod r#enum;
mod field;
mod r#struct;
mod tag;
mod variant;

pub use derive::derive as do_it;
