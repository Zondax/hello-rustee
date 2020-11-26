#![no_std]

#[cfg(feature = "framework")]
pub mod framework;
pub mod wrapper;

/// Trait that must be implemented by types that can process commands from Ta
pub trait HandleTaCommand {
    fn handle_command(
        cmd_id: u32,
        param_types: u32,
        parameters: &mut wrapper::Parameters,
    ) -> Result<(), wrapper::TaErrorCode>;
}
