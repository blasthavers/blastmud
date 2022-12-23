use std::fs;
use std::error::Error;
use serde::Deserialize;
use ring::signature;
use base64;
use crate::DResult;

#[derive(Deserialize)]
struct AV {
    copyright: String,
    serial: u64,
    cn: String,
    assertion: String,
    sig: String
}

static KEY_BYTES: [u8;65] = [
    0x04, 0x4f, 0xa0, 0x8b, 0x32, 0xa7, 0x7f, 0xc1, 0x0a, 0xfc, 0x51, 0x95, 0x93, 0x57, 0x05,
    0xb3, 0x0f, 0xad, 0x16, 0x05, 0x3c, 0x7c, 0xfc, 0x02, 0xd2, 0x7a, 0x63, 0xff, 0xd3, 0x09,
    0xaa, 0x5b, 0x78, 0xfe, 0xa8, 0xc2, 0xc3, 0x02, 0xc2, 0xe6, 0xaf, 0x81, 0xc7, 0xa3, 0x03,
    0xfa, 0x4d, 0xf1, 0xf9, 0xfc, 0x0a, 0x36, 0xef, 0x6b, 0x1e, 0x9d, 0xce, 0x6e, 0x60, 0xc6,
    0xa8, 0xb3, 0x02, 0x35, 0x7e
];

pub fn check() -> DResult<()> {
    let av: AV = serde_yaml::from_str(&fs::read_to_string("age-verification.yml")?).
        map_err(|error| Box::new(error) as Box<dyn Error + Send + Sync>)?;
    if av.copyright != "This file is protected by copyright and may not be used or reproduced except as authorised by the copyright holder. All rights reserved." ||
        av.assertion != "age>=18" {
            Err(Box::<dyn Error + Send + Sync>::from("Invalid age-verification.yml"))?;
        }

    let sign_text = format!("cn={};{};serial={}", av.cn, av.assertion, av.serial);
    let key: signature::UnparsedPublicKey<&[u8]> =
        signature::UnparsedPublicKey::new(&signature::ECDSA_P256_SHA256_ASN1, &KEY_BYTES);
    key.verify(&sign_text.as_bytes(), &base64::decode(av.sig)?)
        .map_err(|_| Box::<dyn Error + Send + Sync>::from("Invalid age-verification.yml signature"))
}
