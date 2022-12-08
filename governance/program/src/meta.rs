#![allow(missing_docs)]

#[cfg(not(feature = "no-entrypoint"))]
use solana_security_txt::security_txt;

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    // Required fields
    name: "Solana Program Library Governance Program",
    project_url: "https://realms.today",
    contacts: "email:security@solana.com",
    policy: "https://github.com/solana-labs/solana/blob/master/SECURITY.md",

    // Optional Fields
    source_code: "https://github.com/solana-labs/solana-program-library",
    source_revision: env!("GITHUB_SHA"),
    source_release: env!("GITHUB_REF_NAME"),
    auditors: "Neodyme,OtterSec"
}
