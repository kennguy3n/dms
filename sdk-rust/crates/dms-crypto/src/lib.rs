use std::fs::File;
use std::io::{Read, Result as IoResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataEncryptionKey(pub Vec<u8>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncryptedDek {
    pub principal_type: PrincipalType,
    pub principal_id: String,
    pub key_wrap_alg: String,
    pub ciphertext: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrincipalType {
    User,
    Group,
}

#[derive(Debug)]
pub enum CryptoError {
    Io(std::io::Error),
    InvalidKeyLength,
    Wrapper(String),
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io error: {e}"),
            Self::InvalidKeyLength => write!(f, "invalid DEK length"),
            Self::Wrapper(msg) => write!(f, "wrapper error: {msg}"),
        }
    }
}

impl std::error::Error for CryptoError {}

impl From<std::io::Error> for CryptoError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

pub trait KeyWrapper {
    fn algorithm(&self) -> &'static str;
    fn wrap(&self, dek: &[u8], principal_public_key: &[u8]) -> Result<Vec<u8>, CryptoError>;
    fn unwrap(
        &self,
        encrypted_dek: &[u8],
        principal_private_key: &[u8],
    ) -> Result<Vec<u8>, CryptoError>;
}

pub fn generate_dek(size_bytes: usize) -> Result<DataEncryptionKey, CryptoError> {
    if !(size_bytes == 16 || size_bytes == 32) {
        return Err(CryptoError::InvalidKeyLength);
    }

    let mut file = File::open("/dev/urandom")?;
    let mut key = vec![0u8; size_bytes];
    read_exact(&mut file, &mut key)?;
    Ok(DataEncryptionKey(key))
}

fn read_exact(file: &mut File, buf: &mut [u8]) -> IoResult<()> {
    file.read_exact(buf)
}

pub fn wrap_dek_for_principal(
    wrapper: &dyn KeyWrapper,
    dek: &DataEncryptionKey,
    principal_type: PrincipalType,
    principal_id: impl Into<String>,
    principal_public_key: &[u8],
) -> Result<EncryptedDek, CryptoError> {
    let ciphertext = wrapper.wrap(&dek.0, principal_public_key)?;
    Ok(EncryptedDek {
        principal_type,
        principal_id: principal_id.into(),
        key_wrap_alg: wrapper.algorithm().to_string(),
        ciphertext,
    })
}

pub fn unwrap_dek_for_principal(
    wrapper: &dyn KeyWrapper,
    encrypted: &EncryptedDek,
    principal_private_key: &[u8],
) -> Result<DataEncryptionKey, CryptoError> {
    let plaintext = wrapper.unwrap(&encrypted.ciphertext, principal_private_key)?;
    Ok(DataEncryptionKey(plaintext))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct XorWrapper;

    impl KeyWrapper for XorWrapper {
        fn algorithm(&self) -> &'static str {
            "test-xor"
        }

        fn wrap(&self, dek: &[u8], principal_public_key: &[u8]) -> Result<Vec<u8>, CryptoError> {
            if principal_public_key.is_empty() {
                return Err(CryptoError::Wrapper("empty key".into()));
            }
            Ok(dek
                .iter()
                .enumerate()
                .map(|(i, b)| b ^ principal_public_key[i % principal_public_key.len()])
                .collect())
        }

        fn unwrap(
            &self,
            encrypted_dek: &[u8],
            principal_private_key: &[u8],
        ) -> Result<Vec<u8>, CryptoError> {
            self.wrap(encrypted_dek, principal_private_key)
        }
    }

    #[test]
    fn roundtrip_wrap_unwrap() {
        let wrapper = XorWrapper;
        let dek = DataEncryptionKey(vec![1, 2, 3, 4]);
        let encrypted =
            wrap_dek_for_principal(&wrapper, &dek, PrincipalType::User, "u1", &[9, 9]).unwrap();
        let unwrapped = unwrap_dek_for_principal(&wrapper, &encrypted, &[9, 9]).unwrap();
        assert_eq!(dek, unwrapped);
    }
}
