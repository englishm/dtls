use super::*;

use std::io::{BufReader, BufWriter};

use util::Error;

#[test]
fn test_change_cipher_spec_round_trip() -> Result<(), Error> {
    let c = ChangeCipherSpec {};
    let mut raw = vec![];
    {
        let mut writer = BufWriter::<&mut Vec<u8>>::new(raw.as_mut());
        c.marshal(&mut writer)?;
    }

    let mut reader = BufReader::new(raw.as_slice());
    let cnew = ChangeCipherSpec::unmarshal(&mut reader)?;
    assert_eq!(
        c, cnew,
        "ChangeCipherSpec round trip: got {:?}, want {:?}",
        cnew, c
    );

    Ok(())
}

#[test]
fn test_change_cipher_spec_invalid() -> Result<(), Error> {
    let data = vec![0x00];

    let mut reader = BufReader::new(data.as_slice());
    let result = ChangeCipherSpec::unmarshal(&mut reader);

    match result {
        Ok(_) => assert!(false, "must be error"),
        Err(err) => assert_eq!(err, *ERR_INVALID_CIPHER_SPEC),
    };

    Ok(())
}
