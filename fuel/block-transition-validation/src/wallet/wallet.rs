use fuel_core::types::{ fuel_vm::SecretKey, fuel_types::Address, fuel_crypto::PublicKey, fuel_tx::Input };

#[derive(Debug, Clone)]
pub struct Wallet {
  pub secret: SecretKey,
  pub address: Address,
  pub pk: PublicKey,
}

impl Wallet {
  pub fn new(secret: SecretKey) -> Self {
    let pk: PublicKey = (&secret).into();
    let address: Address = Input::owner(&pk);

    Self {
      secret,
      address,
      pk,
    }
  }
}

impl Into<Address> for Wallet {
  fn into(self) -> Address {
    self.address
  }
}

impl Into<SecretKey> for Wallet {
  fn into(self) -> SecretKey {
    self.secret
  }
}
