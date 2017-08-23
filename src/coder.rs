use json;

pub struct Coder;

impl Coder {

    pub fn decode_to_str(query: Vec<u8>) -> String {
        String::from_utf8(query).unwrap().to_string()
    }

    pub fn to_json(query: &str) -> json::JsonValue {
        json::parse(query).unwrap_or(json::parse("{ \"error\": \"parsing error, not a json response\" }").unwrap())
    }

    pub fn to_json2(query: &Vec<u8>) -> json::JsonValue {
        let tmp = Coder::decode_to_str(query.to_vec());
        Coder::to_json(&tmp)
    }

}
