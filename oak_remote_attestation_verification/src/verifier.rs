//
// Copyright 2023 The Project Oak Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use alloc::{string::String, vec::Vec};
use oak_remote_attestation::proto::oak::session::v1::{
    AttestationEndorsement, AttestationEvidence, BinaryAttestation,
};

use crate::rekor::{equal_keys, get_rekor_log_entry_body, verify_rekor_log_entry};
use anyhow::Context;
use base64::{prelude::BASE64_STANDARD, Engine as _};
use oak_transparency_claims::claims::{
    parse_endorsement_statement, validate_endorsement, verify_validity_duration,
};

/// Reference values used by the verifier to appraise the attestation evidence.
/// <https://www.rfc-editor.org/rfc/rfc9334.html#name-reference-values>
pub struct ReferenceValue {
    pub binary_hash: Vec<u8>,
}

/// A trait implementing the functionality of a verifier that appraises the attestation evidence and
/// produces an attestation result.
/// <https://www.rfc-editor.org/rfc/rfc9334.html#name-verifier>
pub trait AttestationVerifier: Clone + Send + Sync {
    /// Verify that the provided evidence was endorsed and contains specified reference values.
    fn verify(
        evidence: &AttestationEvidence,
        endorsement: &AttestationEndorsement,
        reference_value: &ReferenceValue,
    ) -> anyhow::Result<()>;
}

/// An instance of [`AttestationVerifier`] that succeeds iff the provided attestation is empty.
///
/// Useful when no attestation is expected to be generated by the other side of a remotely
/// attested connection.
#[derive(Clone)]
pub struct InsecureAttestationVerifier;

impl AttestationVerifier for InsecureAttestationVerifier {
    fn verify(
        evidence: &AttestationEvidence,
        _endorsement: &AttestationEndorsement,
        _reference_value: &ReferenceValue,
    ) -> anyhow::Result<()> {
        // We check that the attestation report is empty in order to avoid accidentally ignoring a
        // real attestation from the other side, although in principle a more lenient
        // implementation of this struct could be used that always ignores also non-empty
        // attestations.
        if evidence.attestation.is_empty() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "expected empty attestation report, got {:?}",
                evidence.attestation
            ))
        }
    }
}

const PEM_HEADER: &str = "-----BEGIN PUBLIC KEY-----";
const PEM_FOOTER: &str = "-----END PUBLIC KEY-----";

/// Makes a plausible guess whether the public key is in PEM format.
pub fn looks_like_pem(maybe_pem: &str) -> bool {
    let p = maybe_pem.trim();
    p.starts_with(PEM_HEADER) && p.ends_with(PEM_FOOTER)
}

/// Converts a pem key to raw. Will panic if it does not like like pem.
pub fn convert_pem_to_raw(public_key_pem: &str) -> anyhow::Result<Vec<u8>> {
    let stripped = public_key_pem
        .trim()
        .strip_prefix(PEM_HEADER)
        .expect("could not find expected header")
        .strip_suffix(PEM_FOOTER)
        .expect("could not find expected footer");
    let remove_newlines = stripped.replace('\n', "");

    Ok(BASE64_STANDARD.decode(remove_newlines)?)
}

pub fn convert_raw_to_pem(public_key: &[u8]) -> String {
    let mut pem = String::from(PEM_HEADER);
    for (i, c) in BASE64_STANDARD.encode(public_key).chars().enumerate() {
        if i % 64 == 0 {
            pem.push('\n');
        }
        pem.push(c);
    }
    pem.push('\n');
    pem.push_str(PEM_FOOTER);
    pem.push('\n');
    pem
}

/// Verifies the binary endorsement for a given measurement. More precisely:
///
/// 1. Verifies that the endorsement statement in the `BinaryAttestation` contains a
/// single subject with a digest measured using the given measurement algorithm, equal to
/// `measurement_from_evidence`.
/// 1. Verifies that the log entry is valid (cf. `verify_rekor_log_entry`),
/// 1. Verifies the Rekor public key in `BinaryAttestation` against the specified one,
/// 1. Verifies the endorser public key from the log entry against the specified one.
pub fn verify_binary_endorsement(
    measurement_from_evidence: &[u8],
    measurement_alg: &str,
    binary_attestation: &BinaryAttestation,
    endorser_public_key: &[u8],
    rekor_public_key: &[u8],
) -> anyhow::Result<()> {
    verify_endorsement_statement(
        &binary_attestation.endorsement_statement,
        measurement_from_evidence,
        measurement_alg,
    )?;

    verify_rekor_log_entry(
        &binary_attestation.rekor_log_entry,
        rekor_public_key,
        &binary_attestation.endorsement_statement,
    )?;
    verify_endorser_public_key(binary_attestation, endorser_public_key)?;
    verify_rekor_public_key(binary_attestation, rekor_public_key)?;

    Ok(())
}

/// Verifies endorsement against the given reference values.
pub fn verify_endorsement_statement(
    endorsement: &[u8],
    binary_digest: &[u8],
    measurement_alg: &str,
) -> anyhow::Result<()> {
    let claim = parse_endorsement_statement(endorsement)?;
    if let Err(err) = validate_endorsement(&claim) {
        anyhow::bail!("validating endorsement: {err:?}");
    }
    verify_validity_duration(&claim)?;
    if claim.subject.len() != 1 {
        anyhow::bail!(
            "expected 1 subject in the endorsement, found {}",
            claim.subject.len()
        );
    }

    let binary_digest = core::str::from_utf8(binary_digest)?;
    match claim.subject[0].digest.get(measurement_alg) {
        Some(found_digest) => {
            if found_digest != binary_digest {
                anyhow::bail!(
                    "unexpected binary {} digest: expected {}, got {}",
                    measurement_alg,
                    binary_digest,
                    found_digest
                )
            }
        }
        None => anyhow::bail!("missing {measurement_alg} digest in the endorsement statement"),
    }

    Ok(())
}

/// Verifies that the Rekor public key coincides with the one contained in the attestation.
pub fn verify_rekor_public_key(
    binary_attestation: &BinaryAttestation,
    rekor_public_key: &[u8],
) -> anyhow::Result<()> {
    // TODO(#4231): Currently, we only check that the public keys are the same. Once Rekor starts
    // using rolling keys, the verification logic will have to be updated.

    let actual_pem_vec = BASE64_STANDARD
        .decode(&binary_attestation.base64_pem_encoded_rekor_public_key)
        .context("couldn't decode public key in binary attestation")?;
    let actual_pem = core::str::from_utf8(&actual_pem_vec)?;
    let actual = convert_pem_to_raw(actual_pem)?;

    if !equal_keys(rekor_public_key, &actual)? {
        anyhow::bail!(
            "Rekor public key mismatch: expected {:?} found {:?}",
            rekor_public_key,
            actual
        )
    }

    Ok(())
}

/// Verifies that the endorser public key coincides with the one contained in the attestation.
pub fn verify_endorser_public_key(
    binary_attestation: &BinaryAttestation,
    endorser_public_key: &[u8],
) -> anyhow::Result<()> {
    // TODO(#4231): Currently, we only check that the public keys are the same. Should be updated to
    // support verifying rolling keys.

    let body = get_rekor_log_entry_body(&binary_attestation.rekor_log_entry)?;

    let actual_pem_vec = BASE64_STANDARD
        .decode(body.spec.signature.public_key.content)
        .context("couldn't base64-decode public key bytes from server")?;
    let actual_pem = core::str::from_utf8(&actual_pem_vec)?;
    let actual = convert_pem_to_raw(actual_pem)?;

    if !equal_keys(endorser_public_key, &actual)? {
        anyhow::bail!(
            "endorser public key mismatch: expected {:?} found {:?}",
            endorser_public_key,
            actual,
        )
    }

    Ok(())
}