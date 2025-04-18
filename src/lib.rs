//#[cfg(feature = "all")]
//anchor_gen::generate_cpi_crate!("idl.json");

//#[cfg(feature = "all")]
//declare_id!("TRSY7YgS3tcDoi6ZgTp2MmPJpXHyCVrGaFhL7HLdQc9");

pub fn controller_discriminator() -> [u8; 8] {
    [184, 79, 171, 0, 183, 43, 113, 110]
}

pub fn delegation_discriminator() -> [u8; 8] {
    [237, 90, 140, 159, 124, 255, 243, 80]
}

pub fn ruleaccumulator_discriminator() -> [u8; 8] {
    [127, 132, 189, 170, 68, 38, 206, 135]
}

pub fn spendrequest_discriminator() -> [u8; 8] {
    [71, 251, 215, 71, 98, 153, 90, 25]
}
