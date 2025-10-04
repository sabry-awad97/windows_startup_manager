pub mod process;
pub mod registry;

#[cfg(test)]
mod process_test;

pub use process::ProcessManager;
pub use registry::WindowsRegistryRepository;
