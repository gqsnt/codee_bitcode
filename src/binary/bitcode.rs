use bitcode::DecodeOwned;
use crate::{Decoder, Encoder};

/// A codec that relies on `bitcode` to encode decode data.
///
/// This is only available with the **`bitcode` feature** enabled.
pub struct BitcodeCodec;

impl<T: bitcode::Encode> Encoder<T> for BitcodeCodec {
    type Error = bitcode::Error;
    type Encoded = Vec<u8>;

    fn encode(val: &T) -> Result<Self::Encoded, Self::Error> {
        Ok(bitcode::encode(val))
    }
}

impl< T: DecodeOwned> Decoder<T> for BitcodeCodec {
    type Error = bitcode::Error;
    type Encoded = [u8];

    fn decode(val: &Self::Encoded) -> Result<T, Self::Error> { bitcode::decode(val) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitcode_codec() {
        #[derive(Clone, Debug, PartialEq, bitcode::Encode, bitcode::Decode)]
        struct Test {
            s: String,
            i: i32,
        }
        let t = Test {
            s: String::from("party time 🎉"),
            i: 42,
        };
        let enc = BitcodeCodec::encode(&t).unwrap();
        let dec: Test = BitcodeCodec::decode(&enc).unwrap();
        assert_eq!(dec, t);
    }
}
