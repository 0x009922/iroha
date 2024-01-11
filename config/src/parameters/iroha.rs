//! Basic parameters like the key pair and p2p address

use std::{error::Error, str::FromStr};

use eyre::{eyre, Context, Report};
use iroha_crypto::{Algorithm, KeyPair, PrivateKey, PublicKey};
use iroha_primitives::addr::SocketAddr;
use merge::Merge;
use serde::{Deserialize, Serialize};

use crate::{
    Complete, CompleteError, CompleteResult, Emitter, FromEnv, FromEnvResult, ParseEnvResult,
    ReadEnv, UserField,
};

#[derive(Deserialize, Serialize, Debug, Default, Merge)]
#[serde(deny_unknown_fields, default)]
pub struct UserLayer {
    pub public_key: UserField<PublicKey>,
    pub private_key: UserField<PrivateKey>,
    pub p2p_address: UserField<SocketAddr>,
}

impl Complete for UserLayer {
    type Output = Config;

    fn complete(self) -> CompleteResult<Config> {
        let mut emitter = Emitter::<CompleteError>::new();

        let key_pair = match (self.public_key.get(), self.private_key.get()) {
            (Some(public_key), Some(private_key)) => {
                KeyPair::new(public_key, private_key)
                    .map(Some)
                    .wrap_err("failed to construct a key pair from `iroha.public_key` and `iroha.private_key` configuration parameters")
                    .unwrap_or_else(|report| {
                        emitter.emit(CompleteError::Custom(report));
                        None
                    })
            },
            (public_key, private_key) => {
                if public_key.is_none() {
                    emitter.emit_missing_field("iroha.public_key");
                }
                if private_key.is_none() {
                    emitter.emit_missing_field("iroha.private_key");
                }
                None
            }
        };

        if self.p2p_address.is_none() {
            emitter.emit_missing_field("iroha.p2p_address");
        }

        emitter.finish()?;

        Ok(Config {
            key_pair: key_pair.unwrap(),
            p2p_address: self.p2p_address.get().unwrap(),
        })
    }
}

pub(crate) fn private_key_from_env(
    emitter: &mut Emitter<Report>,
    env: &impl ReadEnv,
    env_key_base: impl AsRef<str>,
    name_base: impl AsRef<str>,
) -> ParseEnvResult<PrivateKey> {
    let digest_env = format!("{}_DIGEST", env_key_base.as_ref());
    let digest_name = format!("{}.digest_function", name_base.as_ref());
    let payload_env = format!("{}_PAYLOAD", env_key_base.as_ref());
    let payload_name = format!("{}.payload", name_base.as_ref());

    let digest_function = ParseEnvResult::parse_simple(emitter, env, &digest_env, &digest_name);

    let payload = env.get(&payload_env).map(ToOwned::to_owned);

    match (digest_function, payload) {
        (ParseEnvResult::Value(digest_function), Some(payload)) => {
            PrivateKey::from_hex(digest_function, &payload)
                .wrap_err_with(|| {
                    eyre!(
                        "failed to construct `{}` from `{}` and `{}` environment variables",
                        name_base.as_ref(),
                        &digest_env,
                        &payload_env
                    )
                })
                .map_or_else(
                    |report| {
                        emitter.emit(report);
                        ParseEnvResult::ParseError
                    },
                    ParseEnvResult::Value,
                )
        }
        (ParseEnvResult::None, None) | (ParseEnvResult::ParseError, _) => ParseEnvResult::None,
        (ParseEnvResult::Value(_), None) => {
            emitter.emit(eyre!(
                "`{}` env was provided, but `{}` was not",
                &digest_env,
                &payload_env
            ));
            ParseEnvResult::ParseError
        }
        (ParseEnvResult::None, Some(_)) => {
            emitter.emit(eyre!(
                "`{}` env was provided, but `{}` was not",
                &payload_env,
                &digest_env
            ));
            ParseEnvResult::ParseError
        }
    }
}

impl FromEnv for UserLayer {
    fn from_env(env: &impl ReadEnv) -> FromEnvResult<Self>
    where
        Self: Sized,
    {
        let mut emitter = Emitter::new();

        let public_key =
            ParseEnvResult::parse_simple(&mut emitter, env, "PUBLIC_KEY", "iroha.public_key")
                .into();
        let private_key =
            private_key_from_env(&mut emitter, env, "PRIVATE_KEY", "iroha.private_key").into();
        let p2p_address =
            ParseEnvResult::parse_simple(&mut emitter, env, "P2P_ADDRESS", "iroha.p2p_address")
                .into();

        emitter.finish()?;

        Ok(Self {
            public_key,
            private_key,
            p2p_address,
        })
    }
}

#[derive(Debug)]
pub struct Config {
    pub key_pair: KeyPair,
    pub p2p_address: SocketAddr,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TestEnv;

    #[test]
    fn parses_private_key_from_env() {
        let env = TestEnv::new()
            .set("PRIVATE_KEY_DIGEST", "ed25519")
            .set("PRIVATE_KEY_PAYLOAD", "8f4c15e5d664da3f13778801d23d4e89b76e94c1b94b389544168b6cb894f84f8ba62848cf767d72e7f7f4b9d2d7ba07fee33760f79abe5597a51520e292a0cb");

        let private_key = UserLayer::from_env(&env)
            .expect("input is valid, should not fail")
            .private_key
            .get()
            .expect("private key is provided, should not fail");

        assert_eq!(private_key.digest_function(), "ed25519".parse().unwrap());
        assert_eq!(hex::encode( private_key.payload()), "8f4c15e5d664da3f13778801d23d4e89b76e94c1b94b389544168b6cb894f84f8ba62848cf767d72e7f7f4b9d2d7ba07fee33760f79abe5597a51520e292a0cb");
    }

    #[test]
    fn fails_to_parse_private_key_in_env_without_digest() {
        let env = TestEnv::new().set("PRIVATE_KEY_DIGEST", "ed25519");
        let error = UserLayer::from_env(&env).expect_err("private key is incomplete, should fail");
        let expected = expect_test::expect![[r#"
            `PRIVATE_KEY_DIGEST` env was provided, but `PRIVATE_KEY_PAYLOAD` was not

            Location:
                config/src/parameters/iroha.rs:100:26"#]];
        expected.assert_eq(&format!("{error:?}"));
    }

    #[test]
    fn fails_to_parse_private_key_in_env_without_payload() {
        let env = TestEnv::new().set("PRIVATE_KEY_PAYLOAD", "8f4c15e5d664da3f13778801d23d4e89b76e94c1b94b389544168b6cb894f84f8ba62848cf767d72e7f7f4b9d2d7ba07fee33760f79abe5597a51520e292a0cb");
        let error = UserLayer::from_env(&env).expect_err("private key is incomplete, should fail");
        let expected = expect_test::expect![[r#"
            `PRIVATE_KEY_PAYLOAD` env was provided, but `PRIVATE_KEY_DIGEST` was not

            Location:
                config/src/parameters/iroha.rs:108:26"#]];
        expected.assert_eq(&format!("{error:?}"));
    }

    #[test]
    fn fails_to_parse_private_key_from_env_with_invalid_payload() {
        let env = TestEnv::new()
            .set("PRIVATE_KEY_DIGEST", "ed25519")
            .set("PRIVATE_KEY_PAYLOAD", "foo");

        let error = UserLayer::from_env(&env).expect_err("input is invalid, should fail");

        let expected = expect_test::expect![[r#"
            failed to construct `iroha.private_key` from `PRIVATE_KEY_DIGEST` and `PRIVATE_KEY_PAYLOAD` environment variables

            Caused by:
                Key could not be parsed. Odd number of digits

            Location:
                config/src/parameters/iroha.rs:82:18"#]];
        expected.assert_eq(&format!("{error:?}"));
    }

    #[test]
    fn when_payload_provided_but_digest_is_invalid() {
        let env = TestEnv::new()
            .set("PRIVATE_KEY_DIGEST", "foo")
            .set("PRIVATE_KEY_PAYLOAD", "8f4c15e5d664da3f13778801d23d4e89b76e94c1b94b389544168b6cb894f84f8ba62848cf767d72e7f7f4b9d2d7ba07fee33760f79abe5597a51520e292a0cb");

        let error = UserLayer::from_env(&env).expect_err("input is invalid, should fail");

        // TODO: print the bad value and supported ones
        let expected = expect_test::expect![[r#"
            failed to parse `iroha.private_key.digest_function` field from `PRIVATE_KEY_DIGEST` env variable

            Caused by:
                Algorithm not supported

            Location:
                config/src/lib.rs:237:14"#]];
        expected.assert_eq(&format!("{error:?}"));
    }
}
