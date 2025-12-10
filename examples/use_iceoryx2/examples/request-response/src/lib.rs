use iceoryx2::prelude::ZeroCopySend;

pub const SERVICE: &str = "I/D/";
pub const SERVICE_EVENT: &str = "I/D/Event";

#[derive(Debug, Clone, Copy, ZeroCopySend)]
#[repr(C)]
pub enum Frontend {
    Add,
    Minus,
}

#[derive(Debug, Clone, Copy, ZeroCopySend)]
#[repr(C)]
pub enum Backend {
    Data(i32),
}
