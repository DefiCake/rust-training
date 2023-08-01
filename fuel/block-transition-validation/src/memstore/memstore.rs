use std::{ collections::BTreeMap, sync::{ Mutex, Arc }, fs::File, io::{ Read, Write } };
use anyhow::Result;
use fuel_core::{
  database::{ Column, Error as DatabaseError, Result as DatabaseResult },
  state::{ BatchOperations, KVItem, KeyValueStore, TransactableStorage, Value },
};
use fuel_core_storage::iter::{ BoxedIter, IntoBoxedIter, IterDirection };
use crate::serialization::lib::BinFileSerde;

#[derive(serde::Serialize, serde::Deserialize, Default, Debug)]
struct MemStore {
  inner: [Mutex<BTreeMap<Vec<u8>, Value>>; Column::COUNT],
}

impl MemStore {
  pub fn iter_all(
    &self,
    column: Column,
    prefix: Option<&[u8]>,
    start: Option<&[u8]>,
    direction: IterDirection
  ) -> impl Iterator<Item = KVItem> {
    let lock = self.inner[column.as_usize()].lock().expect("poisoned");

    fn clone<K: Clone, V: Clone>(kv: (&K, &V)) -> (K, V) {
      (kv.0.clone(), kv.1.clone())
    }

    let collection: Vec<_> = match (prefix, start) {
      (None, None) => {
        if direction == IterDirection::Forward {
          lock.iter().map(clone).collect()
        } else {
          lock.iter().rev().map(clone).collect()
        }
      }
      (Some(prefix), None) => {
        if direction == IterDirection::Forward {
          lock
            .range(prefix.to_vec()..)
            .take_while(|(key, _)| key.starts_with(prefix))
            .map(clone)
            .collect()
        } else {
          let mut vec: Vec<_> = lock
            .range(prefix.to_vec()..)
            .into_boxed()
            .take_while(|(key, _)| key.starts_with(prefix))
            .map(clone)
            .collect();

          vec.reverse();
          vec
        }
      }
      (None, Some(start)) => {
        if direction == IterDirection::Forward {
          lock
            .range(start.to_vec()..)
            .map(clone)
            .collect()
        } else {
          lock
            .range(..=start.to_vec())
            .rev()
            .map(clone)
            .collect()
        }
      }
      (Some(prefix), Some(start)) => {
        if direction == IterDirection::Forward {
          lock
            .range(start.to_vec()..)
            .take_while(|(key, _)| key.starts_with(prefix))
            .map(clone)
            .collect()
        } else {
          lock
            .range(..=start.to_vec())
            .rev()
            .take_while(|(key, _)| key.starts_with(prefix))
            .map(clone)
            .collect()
        }
      }
    };

    collection.into_iter().map(Ok)
  }
}

impl KeyValueStore for MemStore {
  fn get(&self, key: &[u8], column: Column) -> DatabaseResult<Option<Value>> {
    Ok(self.inner[column.as_usize()].lock().expect("poisoned").get(&key.to_vec()).cloned())
  }

  fn put(&self, key: &[u8], column: Column, value: Value) -> DatabaseResult<Option<Value>> {
    Ok(self.inner[column.as_usize()].lock().expect("poisoned").insert(key.to_vec(), value))
  }

  fn delete(&self, key: &[u8], column: Column) -> DatabaseResult<Option<Value>> {
    Ok(self.inner[column.as_usize()].lock().expect("poisoned").remove(&key.to_vec()))
  }

  fn exists(&self, key: &[u8], column: Column) -> DatabaseResult<bool> {
    Ok(self.inner[column.as_usize()].lock().expect("poisoned").contains_key(&key.to_vec()))
  }

  fn iter_all(
    &self,
    column: Column,
    prefix: Option<&[u8]>,
    start: Option<&[u8]>,
    direction: IterDirection
  ) -> BoxedIter<KVItem> {
    self.iter_all(column, prefix, start, direction).into_boxed()
  }

  fn size_of_value(&self, key: &[u8], column: Column) -> DatabaseResult<Option<usize>> {
    Ok(
      self.inner[column.as_usize()]
        .lock()
        .expect("poisoned")
        .get(&key.to_vec())
        .map(|v| v.len())
    )
  }

  fn read(&self, key: &[u8], column: Column, mut buf: &mut [u8]) -> DatabaseResult<Option<usize>> {
    self.inner[column.as_usize()]
      .lock()
      .expect("poisoned")
      .get(&key.to_vec())
      .map(|value| {
        let read = value.len();
        std::io::Write::write_all(&mut buf, value.as_ref()).map_err(|e| DatabaseError::Other(anyhow::anyhow!(e)))?;
        DatabaseResult::Ok(read)
      })
      .transpose()
  }

  fn read_alloc(&self, key: &[u8], column: Column) -> DatabaseResult<Option<Value>> {
    Ok(self.inner[column.as_usize()].lock().expect("poisoned").get(&key.to_vec()).cloned())
  }

  fn write(&self, key: &[u8], column: Column, buf: &[u8]) -> DatabaseResult<usize> {
    let len = buf.len();
    self.inner[column.as_usize()].lock().expect("poisoned").insert(key.to_vec(), Arc::new(buf.to_vec()));
    Ok(len)
  }

  fn replace(&self, key: &[u8], column: Column, buf: &[u8]) -> DatabaseResult<(usize, Option<Value>)> {
    let len = buf.len();
    let existing = self.inner[column.as_usize()].lock().expect("poisoned").insert(key.to_vec(), Arc::new(buf.to_vec()));
    Ok((len, existing))
  }

  fn take(&self, key: &[u8], column: Column) -> DatabaseResult<Option<Value>> {
    Ok(self.inner[column.as_usize()].lock().expect("poisoned").remove(&key.to_vec()))
  }
}

impl BatchOperations for MemStore {}

impl TransactableStorage for MemStore {}

impl BinFileSerde for MemStore {
  fn to_bincode_file(&self, path: String) -> Result<()> {
    let bin_data = bincode::serialize(&self)?;
    let mut file = File::create(&path)?;
    file.write_all(&bin_data)?;

    Ok(())
  }

  fn from_bincode_file(path: String) -> Result<Self> where Self: Sized {
    let mut file = File::open(&path)?;
    let mut bin_data = Vec::new();
    file.read_to_end(&mut bin_data)?;

    let database: MemStore = bincode::deserialize(&bin_data)?;

    Ok(database)
  }
}
