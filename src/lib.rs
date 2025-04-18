#[cfg(feature = "all")]
anchor_gen::generate_cpi_crate!("idl.json");

#[cfg(feature = "all")]
declare_id!("TRSY7YgS3tcDoi6ZgTp2MmPJpXHyCVrGaFhL7HLdQc9");

pub mod simple;
