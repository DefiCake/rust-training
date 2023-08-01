use anyhow::Result;

pub trait BinFileSerde {
  fn to_bincode_file(&self, path: String) -> Result<()>;
  fn from_bincode_file(path: String) -> Result<Self> where Self: Sized;
}
