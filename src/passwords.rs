use std::fs::{OpenOptions,File};
use std::io::prelude::*;

use crypto::{scrypt,aes};
use bincode::{serialize,deserialize};
use rand::{rngs::OsRng, Rng,RngCore};




#[derive(Serialize,Deserialize,Debug,PartialEq)]
pub struct Pass {
    pub username : String,
    pub password : Vec<u8>,
}

pub type Key = Vec<u8>;



#[derive(Serialize,Deserialize,Debug,PartialEq)]
pub enum EntryType {
    Pass(Pass),
    Key(Key),
}

#[derive(Serialize,Deserialize,Debug,PartialEq)]
pub struct Entry {
    pub name : String,
    pub value : EntryType,
}
pub struct Entries {
    pub data : Vec<Entry>,
    salt : Vec<u8>,
    key : Vec<u8>,
    rng : OsRng,
}

#[derive(Serialize,Deserialize,Debug,PartialEq)]
struct PasswordFile {
    pub salt : Vec<u8>,
    pub noce : Vec<u8>,
    pub data : Vec<u8>,
}
impl Entries {
    pub fn new(password : String) -> Self {
        let params = scrypt::ScryptParams::new(1,8,1337);
        let mut key = [0u8;32].to_vec();
        let mut rng = OsRng::new().ok().unwrap();
        let mut salt = [0u8;32].to_vec(); //TODO: gensalt on new
        rng.fill_bytes(salt.as_mut_slice());
        let params = scrypt::ScryptParams::new(1,8,1337);
        scrypt::scrypt(password.as_bytes(),salt.as_slice(),&params,key.as_mut_slice());
        Self {
            data : Vec::new(),
            key,
            salt,
            rng,
        }
    }
    pub fn read(path : String, password : String) -> Self {
        let params = scrypt::ScryptParams::new(1,8,1337);
        let mut file = File::open(path).unwrap();
        let mut contents : Vec<u8> = Vec::new();
        file.read_to_end(&mut contents).unwrap();
        let file : PasswordFile = deserialize(contents.as_slice()).unwrap();
        let mut key = [0u8;32].to_vec();
        let mut rng = OsRng::new().ok().unwrap();
        let mut salt = file.salt;
        scrypt::scrypt(password.as_bytes(),salt.as_slice(),&params,key.as_mut_slice());
        let mut cipher = aes::ctr(aes::KeySize::KeySize256,key.as_slice(),file.noce.as_slice());
        let mut output = vec![0u8;file.data.len()];

        cipher.process(file.data.as_slice(),output.as_mut_slice());
        Self {
            data : deserialize(output.as_slice()).unwrap(),
            key,
            salt,
            rng,
        }
    }
    pub fn write(&mut self,path : String) {
        let mut file = OpenOptions::new().write(true).create(true).truncate(false).open(path).unwrap();
        let mut noce = [0u8;32].to_vec();
        self.rng.fill_bytes(noce.as_mut_slice());
        let mut cipher = aes::ctr(aes::KeySize::KeySize256,self.key.as_slice(),noce.as_slice());
        let data = serialize(&self.data).unwrap();
        let mut output = vec![0u8;data.len()];
        cipher.process(data.as_slice(),output.as_mut_slice());
        let out = serialize(&PasswordFile{
            salt : self.salt.clone(),
            noce,
            data : output,
        }).unwrap();
        file.write_all(out.as_slice());
    } 
    fn insert(&mut self, entry : Entry) {
        self.data.push(entry);
    }
}
