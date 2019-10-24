use crate::types::{AccessControl, UserAction};

/// SignatureFS
///
/// It gets signature of the program & user that do syscall to filesystem.
/// check their signature whether it matches with allowed signature in filesystem or not
/// if it's matches, then continue.
///
/// It loads policy based file from embedded database and fetch trusted signature from it.
///
pub struct SignatureFS {}

impl SignatureFS {}

impl FileSystem for SignatureFS {}
