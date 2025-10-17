use axum::{routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use starknet_types_core::felt::Felt;
use starknet_signers::{LocalWallet, SigningKey, Signer};
use derive_builder::Builder;
use starknet_crypto as sn_crypto;

use crate::{config::AppConfig, error::AppError};

// Resource-centric routes under /signatures
// - POST /signatures          -> sign a hash
// - POST /signatures/verify   -> verify signature and return service pubkey
pub fn sign_routes() -> Router<AppConfig> {
    Router::new()
        .route("/signatures", post(sign_handler))
        .route("/signatures/verify", post(verify_handler))
}

#[derive(Deserialize)]
pub struct StarknetSignRequest {
    // 0x-prefixed or plain hex felt hash
    pub hash: String,
}

#[derive(Serialize, Builder, Clone, Debug)]
#[builder(pattern = "owned", build_fn(name = "_build"))]
pub struct SignResponse {
    pub r: String,
    pub s: String,
}

impl SignResponseBuilder {
    pub fn build(self) -> Result<SignResponse, &'static str> {
        self._build().map_err(|_| "invalid response")
    }
}

async fn sign_handler(
    axum::extract::State(config): axum::extract::State<AppConfig>,
    Json(body): Json<StarknetSignRequest>,
) -> Result<Json<SignResponse>, AppError> {
    let hash = parse_felt_hex(&body.hash).map_err(|e| AppError::BadRequest(e))?;
    let wallet = LocalWallet::from_signing_key(SigningKey::from_secret_scalar(config.starknet_private_key.clone()));
    let sig = wallet.sign_hash(&hash).await.map_err(|_| AppError::Internal)?;
    let resp = SignResponseBuilder::default()
        .r(to_hex_prefixed(&sig.r))
        .s(to_hex_prefixed(&sig.s))
        .build()
        .map_err(|_| AppError::Internal)?;
    Ok(Json(resp))
}

fn parse_felt_hex(s: &str) -> Result<Felt, String> {
    let trimmed = s.trim();
    let hex = trimmed.strip_prefix("0x").unwrap_or(trimmed);
    Felt::from_hex(hex).map_err(|_| "invalid felt hex".to_string())
}

fn to_hex_prefixed(f: &Felt) -> String {
    format!("{:#x}", f)
}

#[derive(Deserialize)]
pub struct PublicKeyRequest {
    pub hash: String,
    pub r: String,
    pub s: String,
}

#[derive(Serialize, Builder, Clone, Debug)]
#[builder(pattern = "owned", build_fn(name = "_build"))]
pub struct PublicKeyResponse {
    pub is_valid: bool,
    pub public_key: String,
}

impl PublicKeyResponseBuilder {
    pub fn build(self) -> Result<PublicKeyResponse, &'static str> {
        self._build().map_err(|_| "invalid response")
    }
}

// Signers-related routes
// - GET /signers/self -> returns the service public key
pub fn signers_routes() -> Router<AppConfig> {
    Router::new().route("/signers/self", get(self_signer_handler))
}

async fn verify_handler(
    axum::extract::State(config): axum::extract::State<AppConfig>,
    Json(body): Json<PublicKeyRequest>,
) -> Result<Json<PublicKeyResponse>, AppError> {
    let hash_felt = parse_felt_hex(&body.hash).map_err(|e| AppError::BadRequest(e))?;
    let r_felt = parse_felt_hex(&body.r).map_err(|e| AppError::BadRequest(e))?;
    let s_felt = parse_felt_hex(&body.s).map_err(|e| AppError::BadRequest(e))?;

    // Derive the public key corresponding to our configured private key
    let verifying_key = SigningKey::from_secret_scalar(config.starknet_private_key.clone()).verifying_key();
    let pk: Felt = verifying_key.scalar();

    // Verify signature against this public key
    let is_valid = sn_crypto::verify(&verifying_key.scalar(), &hash_felt, &r_felt, &s_felt)
        .map_err(|_| AppError::Internal)?;

    let resp = PublicKeyResponseBuilder::default()
        .is_valid(is_valid)
        .public_key(to_hex_prefixed(&pk))
        .build()
        .map_err(|_| AppError::Internal)?;

    Ok(Json(resp))
}

#[derive(Serialize, Builder, Clone, Debug)]
#[builder(pattern = "owned", build_fn(name = "_build"))]
pub struct SelfSignerResponse {
    pub public_key: String,
}

impl SelfSignerResponseBuilder {
    pub fn build(self) -> Result<SelfSignerResponse, &'static str> {
        self._build().map_err(|_| "invalid response")
    }
}

async fn self_signer_handler(
    axum::extract::State(config): axum::extract::State<AppConfig>,
) -> Result<Json<SelfSignerResponse>, AppError> {
    let verifying_key = SigningKey::from_secret_scalar(config.starknet_private_key.clone()).verifying_key();
    let pk: Felt = verifying_key.scalar();
    let resp = SelfSignerResponseBuilder::default()
        .public_key(to_hex_prefixed(&pk))
        .build()
        .map_err(|_| AppError::Internal)?;
    Ok(Json(resp))
}
