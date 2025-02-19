use anyhow::*;
use async_trait::async_trait;
use kbs_types::Tee;

pub mod sample;

#[cfg(feature = "az-snp-vtpm-verifier")]
pub mod az_snp_vtpm;

#[cfg(feature = "snp-verifier")]
pub mod snp;

#[cfg(feature = "tdx-verifier")]
pub mod tdx;

#[cfg(feature = "sgx-verifier")]
pub mod sgx;

#[cfg(feature = "csv-verifier")]
pub mod csv;

#[cfg(feature = "cca-verifier")]
pub mod cca;

pub fn to_verifier(tee: &Tee) -> Result<Box<dyn Verifier + Send + Sync>> {
    match tee {
        Tee::Sev => todo!(),
        Tee::AzSnpVtpm => {
            cfg_if::cfg_if! {
                if #[cfg(feature = "az-snp-vtpm-verifier")] {
                    let verifier = az_snp_vtpm::AzSnpVtpm::new()?;
                    Ok(Box::new(verifier) as Box<dyn Verifier + Send + Sync>)
                } else {
                    bail!("feature `az-snp-vtpm-verifier` is not enabled for `verifier` crate.")
                }
            }
        }
        Tee::Tdx => {
            cfg_if::cfg_if! {
                if #[cfg(feature = "tdx-verifier")] {
                    Ok(Box::<tdx::Tdx>::default() as Box<dyn Verifier + Send + Sync>)
                } else {
                    bail!("feature `tdx-verifier` is not enabled for `verifier` crate.")
                }
            }
        }
        Tee::Snp => {
            cfg_if::cfg_if! {
                if #[cfg(feature = "snp-verifier")] {
                    let verifier = snp::Snp::new()?;
                    Ok(Box::new(verifier) as Box<dyn Verifier + Send + Sync>)
                } else {
                    bail!("feature `snp-verifier` is not enabled for `verifier` crate.")
                }
            }
        }
        Tee::Sample => Ok(Box::<sample::Sample>::default() as Box<dyn Verifier + Send + Sync>),
        Tee::Sgx => {
            cfg_if::cfg_if! {
                if #[cfg(feature = "sgx-verifier")] {
                    Ok(Box::<sgx::SgxVerifier>::default() as Box<dyn Verifier + Send + Sync>)
                } else {
                    bail!("feature `sgx-verifier` is not enabled for `verifier` crate.")
                }
            }
        }

        Tee::Csv => {
            cfg_if::cfg_if! {
                if #[cfg(feature = "csv-verifier")] {
                    Ok(Box::<csv::CsvVerifier>::default() as Box<dyn Verifier + Send + Sync>)
                } else {
                    bail!("feature `csv-verifier` is not enabled for `verifier` crate.")
                }
            }
        }

        Tee::Cca => {
            cfg_if::cfg_if! {
                if #[cfg(feature = "cca-verifier")] {
                    Ok(Box::<cca::CCA>::default() as Box<dyn Verifier + Send + Sync>)
                } else {
                    bail!("feature `cca-verifier` is not enabled for `verifier` crate.")
                }
            }
        }
    }
}

pub type TeeEvidenceParsedClaim = serde_json::Value;

pub enum ReportData<'a> {
    Value(&'a [u8]),
    NotProvided,
}

pub enum InitDataHash<'a> {
    Value(&'a [u8]),
    NotProvided,
}

#[async_trait]
pub trait Verifier {
    /// Verify the hardware signature.
    ///
    ///
    /// `evidence` is a bytes slice of TEE evidence. Please note that
    /// it might not be the raw attestation quote/evidence from hardware.
    /// On some platforms they are wrapped by some extra context information.
    /// Please see the concrete verifier implementations to check the format.
    ///
    ///
    /// If `report_data` is given, the binding of the `report_data`
    /// against the `report_data` inside the hardware evidence will
    /// be checked. So do `init_data_hash`.
    ///
    ///
    /// Semantically, a `report_data` is a byte slice given when
    /// a hardware evidence is generated. The `report_data` will be
    /// included inside the hardware evidence, thus its integrity will
    /// be protected by the signature of the hardware.
    ///
    ///
    /// A `init_data_hash` is another byte slice given when the TEE
    /// instance is created. It is always provided by untrusted host,
    /// but its integrity will be protected by the tee evidence.
    /// Typical `init_data_hash` is `HOSTDATA` for SNP.
    async fn evaluate(
        &self,
        evidence: &[u8],
        expected_report_data: &ReportData,
        expected_init_data_hash: &InitDataHash,
    ) -> Result<TeeEvidenceParsedClaim>;
}
