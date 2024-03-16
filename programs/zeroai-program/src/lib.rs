use anchor_lang::prelude::*;
use instructions::add::*;
use instructions::close::*;
use instructions::initialize::*;
use solana_security_txt::security_txt;
use state::Vault;

pub mod instructions;
pub mod state;

mod error;
use error::ErrorCode;

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "ZEROAI Program",
    project_url: "https://0ai.ai",
    contacts: "email:business@0ai.ai,link:https://t.me/ZEROAI_business",
    policy: "https://github.com/zeroai-official/zeroai-solana/blob/main/POLICY.md",

    // Optional Fields
    preferred_languages: "en",
    source_code: "https://github.com/zeroai-official/zeroai-solana",
    encryption: "
-----BEGIN PGP PUBLIC KEY BLOCK-----
Comment: Yafet's OpenPGP certificate

mDMEZfM5MRYJKwYBBAHaRw8BAQdAEQdK6Jx6aRW67639XStinPbZz2w8A+yo7jO+
SBLJaw60HFlhZmV0IDx5YWZldC50Njk5QGdtYWlsLmNvbT6ImQQTFgoAQRYhBFGW
4ff4llffcGQaZQdBysLCbEVzBQJl8zkxAhsDBQkFo5qABQsJCAcCAiICBhUKCQgL
AgQWAgMBAh4HAheAAAoJEAdBysLCbEVzvCwBALVxnLQiydpEc0NmxJ3uQjqwAXPY
e/cyfPBJxJ9zcpvQAQCvqdq/XN+7G703AYNHq1FHN1nkVa6EbsCaeFjdqt0ECbg4
BGXzOTESCisGAQQBl1UBBQEBB0AY+QRzqVbQNceXGC5BbtDIcieLSz2vzIE3VZ2y
3SJVFQMBCAeIfgQYFgoAJhYhBFGW4ff4llffcGQaZQdBysLCbEVzBQJl8zkxAhsM
BQkFo5qAAAoJEAdBysLCbEVzBWUBAJ1eOWq/ZE6NbJZcFdVUcRjdM43QeEEIFCfK
RZSmAFOsAP9ahFP4ZYi6sWb6l1gq2cVsB4OcUWnabI7WBLsP3IYYBA==
=8coH
-----END PGP PUBLIC KEY BLOCK-----
",
auditors: "Yafet",
acknowledgements: "
We are grateful to the security researchers and ethical hackers 
who help us identify and address vulnerabilities in our systems. 
While we do not currently offer monetary rewards, we sincerely 
appreciate and acknowledge the efforts of those who responsibly 
disclose security issues to us.

Your contributions play a vital role in enhancing the security 
and integrity of our platform. We welcome you to join our mission 
of building a secure and reliable environment for our users.

If you discover any potential security vulnerabilities, please 
report them to us through the channels listed in this file. We will 
thoroughly investigate all reports and provide appropriate 
recognition for confirmed issues.

Thank you for your dedication and support in making our systems 
more resilient.
"
}

declare_id!("FDkZRiRJapBGTmcr9u8dQtHEk9VbDsb4E9dY4NYPLkJ3");

#[program]
mod zeroai_program {

    use self::state::Vault;

    use super::*;

    pub fn initvault(ctx: Context<InitVault>, vault: Vault) -> Result<()> {
        init_vault(ctx, vault)
    }

    pub fn initgames(ctx: Context<InitGames>) -> Result<()> {
        init_games(ctx)
    }

    pub fn addgame(ctx: Context<AddGame>, symbol: Vec<String>, price: Vec<u64>) -> Result<()> {
        add_game(ctx, symbol, price)
    }

    pub fn closegame(ctx: Context<CloseGame>) -> Result<()> {
        close_game(ctx)
    }
}
