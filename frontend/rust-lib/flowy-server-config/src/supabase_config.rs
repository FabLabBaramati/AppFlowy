use serde::{Deserialize, Serialize};

use flowy_error::{ErrorCode, FlowyError};

pub const ENABLE_SUPABASE_SYNC: &str = "ENABLE_SUPABASE_SYNC";
pub const SUPABASE_URL: &str = "SUPABASE_URL";
pub const SUPABASE_ANON_KEY: &str = "SUPABASE_ANON_KEY";
pub const SUPABASE_JWT_SECRET: &str = "SUPABASE_JWT_SECRET";

pub const SUPABASE_DB: &str = "SUPABASE_DB";
pub const SUPABASE_DB_USER: &str = "SUPABASE_DB_USER";
pub const SUPABASE_DB_PASSWORD: &str = "SUPABASE_DB_PASSWORD";
pub const SUPABASE_DB_PORT: &str = "SUPABASE_DB_PORT";

/// The configuration for the postgres database. It supports deserializing from the json string that
/// passed from the frontend application. [AppFlowyEnv::parser]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SupabaseConfiguration {
  /// The url of the supabase server.
  pub url: String,
  /// The key of the supabase server.
  pub anon_key: String,
  /// The secret used to sign the JWT tokens.
  pub jwt_secret: String,
  /// Whether to enable the supabase sync.
  /// User can disable it by injecting the environment variable ENABLE_SUPABASE_SYNC=false
  pub enable_sync: bool,
}

impl SupabaseConfiguration {
  pub fn from_env() -> Result<Self, FlowyError> {
    Ok(Self {
      enable_sync: std::env::var(ENABLE_SUPABASE_SYNC)
        .map(|v| v == "true")
        .unwrap_or(false),
      url: std::env::var(SUPABASE_URL)
        .map_err(|_| FlowyError::new(ErrorCode::InvalidAuthConfig, "Missing SUPABASE_URL"))?,
      anon_key: std::env::var(SUPABASE_ANON_KEY)
        .map_err(|_| FlowyError::new(ErrorCode::InvalidAuthConfig, "Missing SUPABASE_ANON_KEY"))?,
      jwt_secret: std::env::var(SUPABASE_JWT_SECRET).map_err(|_| {
        FlowyError::new(ErrorCode::InvalidAuthConfig, "Missing SUPABASE_JWT_SECRET")
      })?,
    })
  }

  /// Write the configuration to the environment variables.
  pub fn write_env(&self) {
    if self.enable_sync {
      std::env::set_var(ENABLE_SUPABASE_SYNC, "true");
    } else {
      std::env::set_var(ENABLE_SUPABASE_SYNC, "false");
    }
    std::env::set_var(SUPABASE_URL, &self.url);
    std::env::set_var(SUPABASE_ANON_KEY, &self.anon_key);
    std::env::set_var(SUPABASE_JWT_SECRET, &self.jwt_secret);
  }
}
