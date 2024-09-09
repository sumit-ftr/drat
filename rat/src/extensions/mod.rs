// all the modules in the extensions module are private
// the modules created in the extensions module
// contains types that are shared across handlers
mod password;
mod shellstate;

// re-exports
pub use password::Password;
pub use shellstate::ShellState;
