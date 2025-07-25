//! Contains the end-point querying logic.  This is where you need to
//! add any custom end-point related logic.

use std::{
    collections::HashMap,
    num::{NonZeroU32, NonZeroU64},
    ops::Deref,
    time::Duration,
};

use derive_more::{DebugCustom, Display};
use eyre::{eyre, Result, WrapErr};
use futures_util::{pin_mut, Sink, SinkExt as _, Stream, StreamExt};
pub use iroha_config::client_api::ConfigGetDTO;
use iroha_config::client_api::ConfigUpdateDTO;
use iroha_logger::prelude::*;
pub use iroha_telemetry::metrics::{Status, Uptime};
use iroha_torii_shared::{uri as torii_uri, Version};
use iroha_version::prelude::*;
use parity_scale_codec::DecodeAll;
use rand::Rng;
use tungstenite::client::IntoClientRequest;
use url::Url;

pub use crate::query::QueryError;
use crate::{
    config::Config,
    crypto::{HashOf, KeyPair},
    data_model::{
        block::SignedBlock,
        events::pipeline::{
            BlockEventFilter, BlockStatus, PipelineEventBox, PipelineEventFilterBox,
            TransactionEventFilter, TransactionStatus,
        },
        isi::Instruction,
        prelude::*,
        transaction::TransactionBuilder,
        ChainId,
    },
    http::{Method as HttpMethod, RequestBuilder, Response, StatusCode},
    http_default::DefaultRequestBuilder,
};

const APPLICATION_JSON: &str = "application/json";

/// `Result` with [`QueryError`] as an error
pub type QueryResult<T> = core::result::Result<T, QueryError>;

/// Phantom struct that handles Transaction API HTTP response
#[derive(Clone, Copy)]
struct TransactionResponseHandler;

impl TransactionResponseHandler {
    fn handle(resp: &Response<Vec<u8>>) -> Result<()> {
        if resp.status() == StatusCode::OK {
            Ok(())
        } else {
            Err(
                ResponseReport::with_msg("Unexpected transaction response", resp)
                    .unwrap_or_else(core::convert::identity)
                    .into(),
            )
        }
    }
}

/// Phantom struct that handles status check HTTP response
#[derive(Clone, Copy)]
pub struct StatusResponseHandler;

impl StatusResponseHandler {
    fn handle(resp: &Response<Vec<u8>>) -> Result<&Vec<u8>> {
        if resp.status() != StatusCode::OK {
            return Err(ResponseReport::with_msg("Unexpected status response", resp)
                .unwrap_or_else(core::convert::identity)
                .into());
        }
        Ok(resp.body())
    }
}

/// Private structure to incapsulate error reporting for HTTP response.
pub(crate) struct ResponseReport(pub(crate) eyre::Report);

impl ResponseReport {
    /// Constructs report with provided message
    ///
    /// # Errors
    /// If response body isn't a valid utf-8 string
    pub(crate) fn with_msg<S: AsRef<str>>(
        msg: S,
        response: &Response<Vec<u8>>,
    ) -> Result<Self, Self> {
        let status = response.status();
        let body = std::str::from_utf8(response.body());
        let msg = msg.as_ref();

        body.map_err(|_| {
            Self(eyre!(
                "{msg}; status: {status}; body isn't a valid utf-8 string"
            ))
        })
        .map(|body| Self(eyre!("{msg}; status: {status}; response body: {body}")))
    }
}

impl From<ResponseReport> for eyre::Report {
    #[inline]
    fn from(report: ResponseReport) -> Self {
        report.0
    }
}

/// Iroha client
#[derive(Clone, DebugCustom, Display)]
#[debug(
    fmt = "Client {{ torii: {torii_url}, public_key: {} }}",
    "key_pair.public_key()"
)]
#[display(fmt = "{}@{torii_url}", "key_pair.public_key()")]
pub struct Client {
    /// Unique id of the blockchain. Used for simple replay attack protection.
    pub chain: ChainId,
    /// Url for accessing Iroha node
    pub torii_url: Url,
    /// Accounts keypair
    pub key_pair: KeyPair,
    /// Transaction time to live in milliseconds
    pub transaction_ttl: Option<Duration>,
    /// Transaction status timeout
    pub transaction_status_timeout: Duration,
    /// Current account
    pub account: AccountId,
    /// Http headers which will be appended to each request
    pub headers: HashMap<String, String>,
    /// If `true` add nonce, which makes different hashes for
    /// transactions which occur repeatedly and/or simultaneously
    pub add_transaction_nonce: bool,
}

/// Handle holding a constructed transaction and providing methods to submit it
pub struct TransactionHandle {
    transaction: SignedTransaction,
    client: Client,
    verify: bool,
    verify_timeout: Duration,
}

impl Deref for TransactionHandle {
    type Target = SignedTransaction;

    fn deref(&self) -> &Self::Target {
        &self.transaction
    }
}

impl TransactionHandle {
    fn new(transaction: SignedTransaction, client: Client) -> Self {
        todo!()
    }

    /// Override `transaction_status_timeout` from the client config
    pub fn with_verify_timeout(self, value: Duration) -> Self {
        todo!()
    }

    /// Submit the transaction
    pub async fn submit(&self) -> Result<()> {
        todo!()
    }

    /// Submit the transaction and verify its status from Iroha by listening to events
    pub async fn submit_and_verify(&self) -> Result<()> {
        todo!()
    }

    // pub fn submit_blocking(&self) -> Result<()> {
    //
    // }
}

/// Representation of `Iroha` client.
impl Client {
    /// Constructor for client from configuration
    #[inline]
    pub fn new(configuration: Config) -> Self {
        Self::with_headers(configuration, HashMap::new())
    }

    /// Constructor for client from configuration and headers
    ///
    /// *Authorization* header will be added if `basic_auth` is presented
    #[inline]
    pub fn with_headers(
        Config {
            chain,
            account,
            torii_api_url,
            key_pair,
            basic_auth,
            transaction_add_nonce,
            transaction_ttl,
            transaction_status_timeout,
        }: Config,
        mut headers: HashMap<String, String>,
    ) -> Self {
        if let Some(basic_auth) = basic_auth {
            let credentials = format!(
                "{}:{}",
                basic_auth.web_login,
                basic_auth.password.expose_secret()
            );
            let engine = base64::engine::general_purpose::STANDARD;
            let encoded = base64::engine::Engine::encode(&engine, credentials);
            headers.insert(String::from("Authorization"), format!("Basic {encoded}"));
        }

        Self {
            chain,
            torii_url: torii_api_url,
            key_pair,
            transaction_ttl: Some(transaction_ttl),
            transaction_status_timeout,
            account,
            headers,
            add_transaction_nonce: transaction_add_nonce,
        }
    }

    /// Build a transaction and return a handle to submit it.
    pub fn transaction<F>(&self, f: F) -> TransactionHandle
    where
        F: FnOnce(TransactionBuilder) -> TransactionBuilder,
    {
        let mut builder = TransactionBuilder::new(self.chain.clone(), self.account.clone());

        if let Some(ttl) = self.transaction_ttl {
            builder = builder.time_to_live(Some(ttl));
        }
        if self.add_transaction_nonce {
            let nonce = rand::thread_rng().gen::<NonZeroU32>();
            builder = builder.nonce(Some(nonce));
        }
        builder = f(builder);
        let tx = self.sign_transaction(builder);
        TransactionHandle::new(tx, self.clone())
    }

    /// Signs transaction
    ///
    /// # Errors
    /// Fails if signature generation fails
    pub fn sign_transaction(&self, transaction: TransactionBuilder) -> SignedTransaction {
        transaction.sign(self.key_pair.private_key())
    }

    async fn wait_tx_confirmation(&self, hash: HashOf<SignedTransaction>) -> Result<()> {
        let filters = vec![
            TransactionEventFilter::default().for_hash(hash).into(),
            PipelineEventFilterBox::from(
                BlockEventFilter::default().for_status(BlockStatus::Applied),
            ),
        ];

        let events = self.listen_for_events(filters).await?;
        pin_mut!(events);

        let confirmation_loop = async {
            let mut block_height = None;
            while let Some(event) = events.next().await {
                if let EventBox::Pipeline(this_event) = event? {
                    match this_event {
                        PipelineEventBox::Transaction(transaction_event) => match transaction_event
                            .status()
                        {
                            TransactionStatus::Queued => {}
                            TransactionStatus::Approved => {
                                block_height = transaction_event.block_height();
                            }
                            TransactionStatus::Rejected(reason) => {
                                return Err((Clone::clone(&**reason)).into());
                            }
                            TransactionStatus::Expired => return Err(eyre!("Transaction expired")),
                        },
                        PipelineEventBox::Block(block_event) => {
                            if Some(block_event.header().height()) == block_height {
                                if let BlockStatus::Applied = block_event.status() {
                                    return Ok(hash);
                                }
                            }
                        }
                    }
                }
            }

            Err(eyre!(
                "Connection dropped without `Committed` or `Rejected` event"
            ))
        };

        tokio::time::timeout(self.transaction_status_timeout, confirmation_loop).await??;

        Ok(())
    }

    /// Connect (through `WebSocket`) to listen for `Iroha` `pipeline` and `data` events.
    ///
    /// # Errors
    ///
    /// TODO
    pub async fn listen_for_events(
        &self,
        event_filters: impl IntoIterator<Item = impl Into<EventFilterBox>>,
    ) -> Result<impl Stream<Item = Result<EventBox>>> {
        async fn next<S>(ws: &mut S) -> Result<Option<EventBox>>
        where
            S: Stream<Item = Result<tungstenite::Message, tungstenite::Error>>
                + Sink<tungstenite::Message, Error = tungstenite::Error>
                + Unpin,
        {
            let msg = ws
                .next()
                .await
                .transpose()?
                .map(|x| match x {
                    tungstenite::Message::Binary(bin) => EventBox::decode_all(&mut &bin[..]),
                    _ => unreachable!(),
                })
                .transpose()?;
            Ok(msg)
        }

        let request = build_ws_request(
            join_torii_url(&self.torii_url, torii_uri::SUBSCRIPTION),
            &self.headers,
        )?;

        let (mut socket, _response) = tokio_tungstenite::connect_async(request).await?;

        socket
            .send(tungstenite::Message::Binary(
                EventSubscriptionRequest::new(event_filters.into_iter().map(Into::into).collect())
                    .encode(),
            ))
            .await?;

        let stream = async_fn_stream::try_fn_stream(|emitter| async move {
            while let Some(item) = next(&mut socket).await? {
                emitter.emit(item).await;
            }
            Ok(())
        });

        Ok(stream)
    }

    /// Receive blocks from Iroha as they occur (via WebSocket).
    ///
    /// # Errors
    ///
    /// TODO:
    pub async fn listen_for_blocks(
        &self,
        height: NonZeroU64,
    ) -> Result<impl Stream<Item = Result<SignedBlock>>> {
        async fn next<S>(ws: &mut S) -> Result<Option<SignedBlock>>
        where
            S: Stream<Item = Result<tungstenite::Message, tungstenite::Error>>
                + Sink<tungstenite::Message, Error = tungstenite::Error>
                + Unpin,
        {
            ws.send(tungstenite::Message::Binary(
                BlockStreamMessage::Next.encode(),
            ))
            .await?;
            let msg = ws
                .next()
                .await
                .transpose()?
                .map(|x| match x {
                    tungstenite::Message::Binary(bin) => {
                        BlockStreamMessage::decode_all(&mut &bin[..])
                    }
                    _ => unreachable!(),
                })
                .transpose()?
                .map(|message| match message {
                    BlockStreamMessage::Block(block) => Ok(block),
                    _ => Err(eyre!("Unexpected message from server")),
                })
                .transpose()?;
            Ok::<_, eyre::Report>(msg)
        }

        let request = build_ws_request(
            join_torii_url(&self.torii_url, torii_uri::BLOCKS_STREAM),
            &self.headers,
        )?;

        let (mut socket, _response) = tokio_tungstenite::connect_async(request)
            .await
            .wrap_err("Failed to establish WebSocket connection")?;

        socket
            .send(tungstenite::Message::Binary(
                BlockStreamMessage::Subscribe(BlockSubscriptionRequest::new(height)).encode(),
            ))
            .await?;

        let stream = async_fn_stream::try_fn_stream(|emitter| async move {
            while let Some(item) = next(&mut socket).await? {
                emitter.emit(item).await;
            }
            Ok(())
        });

        Ok(stream)
    }

    /// Get value of config on peer
    ///
    /// # Errors
    /// Fails if sending request or decoding fails
    pub fn get_config(&self) -> Result<ConfigGetDTO> {
        let resp = DefaultRequestBuilder::new(
            HttpMethod::GET,
            join_torii_url(&self.torii_url, torii_uri::CONFIGURATION),
        )
        .headers(&self.headers)
        .header(http::header::CONTENT_TYPE, APPLICATION_JSON)
        .build()?
        .send()?;

        if resp.status() != StatusCode::OK {
            return Err(eyre!(
                "Failed to get configuration with HTTP status: {}. {}",
                resp.status(),
                std::str::from_utf8(resp.body()).unwrap_or(""),
            ));
        }
        serde_json::from_slice(resp.body()).wrap_err("Failed to decode body")
    }

    /// Send a request to change the configuration of a specified field.
    ///
    /// # Errors
    /// If sending request or decoding fails
    pub fn set_config(&self, dto: &ConfigUpdateDTO) -> Result<()> {
        let body = serde_json::to_vec(&dto).wrap_err(format!("Failed to serialize {dto:?}"))?;
        let url = join_torii_url(&self.torii_url, torii_uri::CONFIGURATION);
        let resp = DefaultRequestBuilder::new(HttpMethod::POST, url)
            .headers(&self.headers)
            .header(http::header::CONTENT_TYPE, APPLICATION_JSON)
            .body(body)
            .build()?
            .send()?;

        if resp.status() != StatusCode::ACCEPTED {
            return Err(eyre!(
                "Failed to post configuration with HTTP status: {}. {}",
                resp.status(),
                std::str::from_utf8(resp.body()).unwrap_or(""),
            ));
        }

        Ok(())
    }

    /// Gets network status seen from the peer
    ///
    /// # Errors
    /// Fails if sending request or decoding fails
    pub fn get_status(&self) -> Result<Status> {
        let req = self
            .prepare_status_request::<DefaultRequestBuilder>()
            .header(http::header::ACCEPT, "application/x-parity-scale");
        let resp = req.build()?.send()?;
        let scaled_resp = StatusResponseHandler::handle(&resp).cloned()?;
        DecodeAll::decode_all(&mut scaled_resp.as_slice()).map_err(|err| eyre!("{err}"))
    }

    /// Prepares http-request to implement [`Self::get_status`] on your own.
    ///
    /// # Errors
    /// Fails if request build fails
    pub fn prepare_status_request<B: RequestBuilder>(&self) -> B {
        B::new(
            HttpMethod::GET,
            join_torii_url(&self.torii_url, torii_uri::STATUS),
        )
        .headers(self.headers.clone())
    }

    /// Get the server version
    ///
    /// # Errors
    /// Fails if sending request or decoding fails
    pub fn get_server_version(&self) -> Result<Version> {
        let resp = DefaultRequestBuilder::new(
            HttpMethod::GET,
            join_torii_url(&self.torii_url, torii_uri::SERVER_VERSION),
        )
        .headers(&self.headers)
        .header(http::header::CONTENT_TYPE, APPLICATION_JSON)
        .build()?
        .send()?;

        if resp.status() != StatusCode::OK {
            return Err(eyre!(
                "Failed to get server version with HTTP status: {}. {}",
                resp.status(),
                std::str::from_utf8(resp.body()).unwrap_or(""),
            ));
        }
        Ok(serde_json::from_slice(resp.body())?)
    }
}

pub(crate) fn join_torii_url(url: &Url, path: &str) -> Url {
    // This is needed to prevent "https://iroha-peer.jp/peer1/".join("/query") == "https://iroha-peer.jp/query"
    let path = path.strip_prefix('/').unwrap_or(path);

    // This is needed to prevent "https://iroha-peer.jp/peer1".join("query") == "https://iroha-peer.jp/query"
    // Note: trailing slash is added to url at config user layer if needed
    assert!(
        url.path().ends_with('/'),
        "Torii url must end with trailing slash"
    );

    url.join(path).expect("Valid URI")
}

fn transform_ws_url(mut url: Url) -> Result<Url> {
    match url.scheme() {
        "https" => url.set_scheme("wss").expect("Valid substitution"),
        "http" => url.set_scheme("ws").expect("Valid substitution"),
        _ => {
            return Err(eyre!(
                "Provided URL scheme is neither `http` nor `https`: {}",
                url
            ))
        }
    }

    Ok(url)
}

fn build_ws_request(url: Url, headers: &HashMap<String, String>) -> Result<http::Request<()>> {
    transform_ws_url(url)?
        .into_client_request()
        .map_err(eyre::Report::from)
        .and_then(|mut req| {
            let req_headers = req.headers_mut();
            for (name, value) in headers {
                req_headers.insert(http::HeaderName::try_from(name)?, value.try_into()?);
            }
            Ok(req)
        })
}

#[cfg(test)]
mod tests {
    use iroha_test_samples::gen_account_in;

    use super::*;
    use crate::{
        config::{BasicAuth, Config},
        secrecy::SecretString,
    };

    const LOGIN: &str = "mad_hatter";
    const PASSWORD: &str = "ilovetea";
    // `mad_hatter:ilovetea` encoded with base64
    const ENCRYPTED_CREDENTIALS: &str = "bWFkX2hhdHRlcjppbG92ZXRlYQ==";

    fn config_factory() -> Config {
        let (account_id, key_pair) = gen_account_in("wonderland");
        Config {
            chain: ChainId::from("00000000-0000-0000-0000-000000000000"),
            key_pair,
            account: account_id,
            torii_api_url: "http://127.0.0.1:8080".parse().unwrap(),
            basic_auth: None,
            transaction_add_nonce: false,
            transaction_ttl: Duration::from_secs(5),
            transaction_status_timeout: Duration::from_secs(10),
        }
    }

    #[test]
    fn txs_same_except_for_nonce_have_different_hashes() {
        let client = Client::new(Config {
            transaction_add_nonce: true,
            ..config_factory()
        });

        let build_transaction = || client.transaction(|builder| builder);
        let tx1 = build_transaction();
        let tx2 = build_transaction();
        assert_ne!(tx1.hash(), tx2.hash());

        let tx2 = {
            let tx = TransactionBuilder::new(client.chain.clone(), client.account.clone())
                .instructions(tx1.instructions().clone())
                .metadata(tx1.metadata().clone())
                .creation_time(tx1.creation_time())
                .nonce(tx1.nonce())
                .time_to_live(tx1.time_to_live());
            client.sign_transaction(tx)
        };
        assert_eq!(tx1.hash(), tx2.hash());
    }

    #[test]
    fn authorization_header() {
        let client = Client::new(Config {
            basic_auth: Some(BasicAuth {
                web_login: LOGIN.parse().expect("Failed to create valid `WebLogin`"),
                password: SecretString::new(PASSWORD.to_owned()),
            }),
            ..config_factory()
        });

        let value = client
            .headers
            .get("Authorization")
            .expect("Expected `Authorization` header");
        let expected_value = format!("Basic {ENCRYPTED_CREDENTIALS}");
        assert_eq!(value, &expected_value);
    }

    #[cfg(test)]
    mod join_torii_url {
        use url::Url;

        use super::*;

        fn do_test(url: &str, path: &str, expected: &str) {
            let url = Url::parse(url).unwrap();
            let actual = join_torii_url(&url, path);
            assert_eq!(actual.as_str(), expected);
        }

        #[test]
        fn path_with_slash() {
            do_test("https://iroha.jp/", "/query", "https://iroha.jp/query");
            do_test(
                "https://iroha.jp/peer-1/",
                "/query",
                "https://iroha.jp/peer-1/query",
            );
        }

        #[test]
        fn path_without_slash() {
            do_test("https://iroha.jp/", "query", "https://iroha.jp/query");
            do_test(
                "https://iroha.jp/peer-1/",
                "query",
                "https://iroha.jp/peer-1/query",
            );
        }

        #[test]
        #[should_panic(expected = "Torii url must end with trailing slash")]
        fn panic_if_url_without_trailing_slash() {
            do_test("https://iroha.jp/peer-1", "query", "should panic");
        }
    }
}
