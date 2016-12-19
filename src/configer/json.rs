extern crate tlib_core as core;
extern crate rustc_serialize as serialize;

pub struct Json {
    type Error serialize::json::DecoderError;
}

impl core::Configer for Json {
    pub fn config<T: serialize::Decodable, S>(dataSource: &S) -> Result<T, Self::Error>
        where S: str
    {
        // TODO
    }
}
