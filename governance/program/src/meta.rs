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
    source_revision: "d9d4214341041ac2bf6880f5d3342e1dccda380a",
    source_release: "governance-v3.1.0",
    auditors: "Neodyme,OtterSec"
}
