use crate::coordinate::Coordinate;

use serde::ser::{Serialize, SerializeStruct};
use serde_json::{to_string, to_value};

struct Response {
    adjacents: Vec<Vec<usize>>,
    coordinates: Vec<Coordinate>,
}

impl Serialize for Response {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Response", 2)?;
        s.serialize_field("adjacents", &self.adjacents)?;
        s.serialize_field("coordinates", &self.coordinates)?;
        s.end()
    }
}

pub fn vec_to_json(points: Vec<Coordinate>, adjacent_matrix: Vec<Vec<usize>>) -> String {
    let response = Response {
        coordinates: points,
        adjacents: adjacent_matrix,
    };
    let ret = to_value(&response).unwrap().to_string();
    ret
}
