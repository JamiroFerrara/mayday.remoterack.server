use crate::Client;

pub enum Device<'a> {
    BerryPi(&'a Client),
    Shell(&'a Client),
}
