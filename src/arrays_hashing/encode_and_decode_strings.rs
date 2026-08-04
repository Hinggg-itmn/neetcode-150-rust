//! encode_and_decode_strings
//! Time: O(N) | Space: O(N)

struct Codec;
impl Codec {
    fn new() -> Self {
        Codec
    }    
    pub fn encode(&self, strs: Vec<String>) -> String {
        let mut encoded= String::new();
        for s in strs{
            encoded.push_str(&format!("{}#{}", s.len(), s));
        }
        encoded
    }
    pub fn decode(&self, s: String)-> Vec<String> {
        let mut res= Vec::new();
        let bytes=s.as_bytes();
        let mut i = 0;
        while i < bytes.len()
        {
            let mut j = i;
            while bytes[j] !=b'#'{
                j+=i; 
            }
        
        let len_str= std::str::from_utf8(&bytes[i..j]).unwrap();
        let len:usize = len_str.parse().unwrap();
        i=j+1;
        let  str_item=std::str::from_utf8(&bytes[i..i+len]).unwrap();
        res.push(str_item.to_string());

        i+=len;
        }
        res

    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_and_decode_normal() {
        let codec = Codec::new();
        let input = vec![
            "Hello".to_string(),
            "World".to_string(),
        ];
        
        let encoded = codec.encode(input.clone());
        let decoded = codec.decode(encoded);
        
        assert_eq!(decoded, input);
    }

    #[test]
    fn test_encode_and_decode_empty_and_special_chars() {
        let codec = Codec::new();
        let input = vec![
            "".to_string(),
            "#!@#$".to_string(),
            "こんにちは".to_string(), // Tiếng Nhật / Unicode
            "a#b".to_string(),
        ];
        
        let encoded = codec.encode(input.clone());
        let decoded = codec.decode(encoded);
        
        assert_eq!(decoded, input);
    }

    #[test]
    fn test_encode_and_decode_empty_vector() {
        let codec = Codec::new();
        let input: Vec<String> = vec![];
        
        let encoded = codec.encode(input.clone());
        let decoded = codec.decode(encoded);
        
        assert_eq!(decoded, input);
    }
}
