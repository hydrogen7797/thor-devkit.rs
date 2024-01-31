use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
pub enum SignInMethod {
    LocalStorageSignIn,
    ExtensionSignIn,
}

impl Default for SignInMethod {
    fn default() -> Self {
        SignInMethod::LocalStorageSignIn
    }
}

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct LocalStore {
    pub sign_in_method: SignInMethod,
}
