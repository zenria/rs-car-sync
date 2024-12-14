use std::io::{self, Read};

use crate::{block_cid::read_block_cid, error::CarDecodeError, varint::read_varint_u64, Cid};

/// Arbitrary high value to prevent big allocations
const MAX_BLOCK_LEN: u64 = 1073741824;

/// # Returns
///
/// (cid, block buffer, total block byte length including varint)
pub(crate) fn decode_block<R: Read>(
    r: &mut R,
) -> Result<(&mut R, Cid, Vec<u8>, usize), CarDecodeError> {
    let (len, cid, varint_len, cid_len) = decode_block_header(r)?;

    // len from header = block_len - varint_len
    let block_len = len - cid_len;

    let mut block_buf = vec![0u8; block_len];
    r.read_exact(&mut block_buf)?;

    Ok((r, cid, block_buf, len + varint_len))
}

fn decode_block_header<R: Read>(src: &mut R) -> Result<(usize, Cid, usize, usize), CarDecodeError> {
    let (len, varint_len) = match read_varint_u64(src) {
        Ok(Some(len)) => len,
        Ok(None) => {
            return Err(CarDecodeError::InvalidBlockHeader(
                "invalid block header varint".to_string(),
            ))
        }
        Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => {
            return Err(CarDecodeError::BlockStartEOF)
        }
        Err(err) => Err(err)?,
    };

    if len == 0 {
        return Err(CarDecodeError::InvalidBlockHeader(
            "zero length".to_string(),
        ));
    }

    if len > MAX_BLOCK_LEN {
        return Err(CarDecodeError::InvalidBlockHeader(format!(
            "block len too big {}",
            len
        )));
    }

    let (cid, cid_len) = read_block_cid(src)?;

    Ok((len as usize, cid, varint_len, cid_len))
}
