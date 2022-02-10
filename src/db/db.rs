use crate::db::types::NftEntry;
use bincode;
use ethers::core::types::Address;
use sled::{self, IVec};
use std::error::Error;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Db {
    imp: sled::Db,
}

impl Db {
    pub fn open(path: &str) -> Result<Self, Box<dyn Error>> {
        match sled::open(path) {
            Ok(db) => Ok(Self { imp: db }),
            Err(e) => Err(Box::new(e)),
        }
    }
    pub fn get_addresses_or_absent(
        &self,
        addresses: &Vec<Address>,
    ) -> (Vec<NftEntry>, Vec<Address>) {
        let mut entries: Vec<NftEntry> = vec![];
        let mut absent: Vec<Address> = vec![];
        for address in addresses {
            match self.get_address(&address) {
                Some(entry) => {
                    println!("Building from cache: {:?}", address.clone());
                    entries.push(entry);
                }
                None => {
                    println!("Cache miss: {:?}", address.clone());
                    absent.push(address.to_owned());
                }
            };
        }
        (entries, absent)
    }

    pub fn get_address(&self, address: &Address) -> Option<NftEntry> {
        match self.get(address) {
            Some(data) => {
                //...
                match bincode::deserialize::<NftEntry>(&data) {
                    Ok(ac) => Some(ac),
                    Err(e) => {
                        println!("Failed to decode key from db: {:?}", e);
                        None
                    }
                }
            }
            None => None,
        }
    }

    pub fn save_address(
        &self,
        address: &Address,
        entry: &NftEntry,
    ) -> Result<bool, Box<dyn Error>> {
        match bincode::serialize(entry) {
            Ok(data) => match self.insert(address, data) {
                Ok(was_set) => Ok(was_set),
                Err(e) => {
                    println!("Failure saving address: {:?}", e.clone());
                    Err(Box::new(e))
                }
            },
            Err(e) => {
                println!("Failed to encode address cache: {:?}", e);
                Err(e)
            }
        }
    }

    pub fn get<K>(&self, key: K) -> Option<IVec>
    where
        K: AsRef<[u8]> + Debug,
    {
        match self.imp.get(&key) {
            Ok(res) => match res {
                Some(data) => Some(data),
                None => None,
            },
            Err(e) => {
                println!("Failed to get key ({:?}) from cache db: {:?}", key, e);
                None
            }
        }
    }
    pub fn insert<K>(&self, key: K, data: Vec<u8>) -> Result<bool, sled::Error>
    where
        K: AsRef<[u8]> + Debug,
    {
        // returns last value if set
        match self.imp.insert(&key, data) {
            Ok(res) => {
                match res {
                    Some(_last_value) => {
                        //...
                        println!("Saved. Last value: {:?}", _last_value);
                        Ok(true)
                    }
                    None => {
                        //...
                        println!("Saved. No last value returned");
                        Ok(false)
                    }
                }
            }
            Err(e) => Err(e),
        }
    }
}
