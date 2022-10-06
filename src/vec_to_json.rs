use crate::coordinate::Coordinate;
use serde::ser::{Serialize, SerializeStruct};
use serde_json::Value;

struct Points {
    coordinates: Vec<Coordinate>,
    adjacents: Vec<Vec<usize>>,
}

impl Serialize for Points {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Points", 3)?;
        s.serialize_field("coordinates", &self.coordinates)?;
        s.serialize_field("adjacents", &self.adjacents)?;
        s.end()
    }
}

/* Format Sample
{ "coordinates": [ { "x": 12, "y": 128 }, { "x": 12, "y": 128 }, ] }
 */

pub fn vec_to_json(points: Vec<Coordinate>, adjacent_matrix: Vec<Vec<usize>>) -> Value {
    let response = Points {
        coordinates: points,
        adjacents: adjacent_matrix,
    };

    let r = serde_json::to_value(&response).unwrap();
    let mut ret = String::new();

    r
}
