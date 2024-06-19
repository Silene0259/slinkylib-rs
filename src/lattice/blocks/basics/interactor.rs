/// # Interactors
/// 
/// Interactors are different messages that are sent using the lattice.

pub struct InteractorType(u16);

pub enum InteractionType {
    Activate, // Connect to chain and activate service
    FinalRevocation, // Deactivate Account (the block is still kept)
    
    
    Message, // Send a message of 512 bytes.
}