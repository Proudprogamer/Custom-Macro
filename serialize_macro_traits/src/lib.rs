ues std::fmt::Error;

pub trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}

pub trait Deserialize : Sized {
    fn desrialize(base : &[u8]) -> Result<OK, std::fmt::Error>;
}