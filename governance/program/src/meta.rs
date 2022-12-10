#![allow(missing_docs)]

#[cfg(not(feature = "no-entrypoint"))]
use solana_security_txt::security_txt;

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    // Required fields
    name: "Mango DAO Governance Program",
    project_url: "https://dao.mango.markets/",
    contacts: "email:blockworks@protonmail.com",
    policy: "https://docs.mango.markets/mango-markets/bug-bounty",

    // Optional Fields
    source_code: "https://github.com/blockworks-foundation/solana-program-library",
    source_revision: env!("GITHUB_SHA"),
    source_release: env!("GITHUB_REF_NAME"),
    auditors: "Neodyme,OtterSec"
}
