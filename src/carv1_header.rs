use cid::Cid;
use libipld::prelude::Codec;
use libipld::Ipld;
use libipld_cbor::DagCborCodec;

use crate::error::CarDecodeError;

#[derive(Debug, PartialEq)]
pub(crate) struct CarV1Header {
    pub version: u64,
    pub roots: Option<Vec<Cid>>,
}

pub(crate) fn decode_carv1_header(header: &[u8]) -> Result<CarV1Header, CarDecodeError> {
    let header: Ipld = DagCborCodec.decode(&header).or_else(|e| {
        Err(CarDecodeError::InvalidCarV1Header(format!(
            "header cbor codec error: {:?}",
            e
        )))
    })?;

    // {"roots": [QmUU2HcUBVSXkfWPUc3WUSeCMrWWeEJTuAgR9uyWBhh9Nf], "version": 1}
    let header = if let Ipld::Map(map) = header {
        map
    } else {
        return Err(CarDecodeError::InvalidCarV1Header(format!(
            "header expected cbor Map but got {:#?}",
            header
        )));
    };

    let roots = match header.get("roots") {
        Some(Ipld::List(roots_ipld)) => {
            let mut roots = Vec::with_capacity(roots_ipld.len());
            for root in roots_ipld {
                if let Ipld::Link(cid) = root {
                    roots.push(cid.clone());
                } else {
                    return Err(CarDecodeError::InvalidCarV1Header(format!(
                        "roots key elements expected cbor Link but got {:#?}",
                        root
                    )));
                }
            }
            Some(roots)
        }
        Some(ipld) => {
            return Err(CarDecodeError::InvalidCarV1Header(format!(
                "roots key expected cbor List but got {:#?}",
                ipld
            )))
        }
        // CARv2 does not have 'roots' key, so allow to not be specified
        None => None,
    };

    let version = match header.get("version") {
        Some(Ipld::Integer(int)) => *int as u64,
        Some(ipld) => {
            return Err(CarDecodeError::InvalidCarV1Header(format!(
                "version key expected cbor Integer but got {:#?}",
                ipld
            )))
        }
        None => {
            return Err(CarDecodeError::InvalidCarV1Header(format!(
                "expected header key version, keys: {:?}",
                header.keys().collect::<Vec<&String>>()
            )))
        }
    };

    Ok(CarV1Header { version, roots })
}

#[cfg(test)]
mod tests {
    use crate::carv2_header::CARV2_PRAGMA;

    use super::*;
    use cid::Cid;
    use hex;

    #[test]
    fn decode_carv1_header_basic() {
        let header_buf = hex::decode("a265726f6f747381d82a58230012205b0995ced69229d26009c53c185a62ea805a339383521edbed1028c4966154486776657273696f6e01").unwrap();
        let cid = Cid::try_from("QmUU2HcUBVSXkfWPUc3WUSeCMrWWeEJTuAgR9uyWBhh9Nf").unwrap();

        assert_eq!(
            decode_carv1_header(&header_buf).unwrap(),
            CarV1Header {
                version: 1,
                roots: Some(vec!(cid))
            }
        )
    }

    #[test]
    fn decode_carv1_header_error_cbor_codec() {
        let header_buf = hex::decode("a265726f6f747371d82a58230012205b0995ced69229d26009c53c185a62ea805a339383521edbed1028c4966154486776657273696f6e01").unwrap();

        match decode_carv1_header(&header_buf) {
            Err(CarDecodeError::InvalidCarV1Header(str)) => assert_eq!(
                str,
                "header cbor codec error: invalid utf-8 sequence of 1 bytes from index 0"
            ),
            x => panic!("other result {:?}", x),
        }
    }

    #[test]
    fn decode_carv1_header_error_cbor_type() {
        let header_buf = hex::decode("0000").unwrap();

        match decode_carv1_header(&header_buf) {
            Err(CarDecodeError::InvalidCarV1Header(str)) => {
                assert_eq!(str, "header expected cbor Map but got Integer(0)")
            }
            x => panic!("other result {:?}", x),
        }
    }

    #[test]
    fn decode_carv1_header_v2_pragma() {
        assert_eq!(
            // First byte is the varint length
            decode_carv1_header(&CARV2_PRAGMA[1..]).unwrap(),
            CarV1Header {
                version: 2,
                roots: None
            }
        )
    }
}
