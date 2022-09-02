use crate::coordinate::Coordinate;

/* Format Sample
{ "coordinates": [ { "x": 12, "y": 128 }, { "x": 12, "y": 128 }, ] }
 */

pub fn vec_to_json(points: &Vec<Coordinate>) -> String {
    let mut ret = String::new();

    ret.push_str("{\"coordinates\":[");

    for p in points {
        ret.push_str(format!("{{\"x\":{},\"y\":{}}},", (*p).x, (*p).y).as_str());
    }
    ret.push_str("]}");
    ret
}
