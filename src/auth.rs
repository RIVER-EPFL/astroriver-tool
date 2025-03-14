use crate::config::Config;
use oauth2::basic::BasicClient;
use oauth2::reqwest;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, DeviceAuthorizationResponse,
    DeviceAuthorizationUrl, EmptyExtraDeviceAuthorizationFields, PkceCodeChallenge, RedirectUrl,
    Scope, StandardDeviceAuthorizationResponse, TokenResponse, TokenUrl,
};
use oauth2::{DeviceCodeErrorResponseType, RequestTokenError};
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use url::Url;

pub(super) async fn device_flow_token() -> Result<String, Box<dyn std::error::Error + Send + Sync>>
{
    // Setup URLs and client as before

    let config = Config::from_env();
    let auth_url = AuthUrl::new(
        format!(
            "{}/realms/{}/protocol/openid-connect/auth",
            config.keycloak_url, config.keycloak_realm
        )
        .to_string(),
    )?;
    let token_url = TokenUrl::new(
        format!(
            "{}/realms/{}/protocol/openid-connect/token",
            config.keycloak_url, config.keycloak_realm
        )
        .to_string(),
    )?;
    let device_auth_url = DeviceAuthorizationUrl::new(
        format!(
            "{}/realms/{}/protocol/openid-connect/auth/device",
            config.keycloak_url, config.keycloak_realm
        )
        .to_string(),
    )?;

    let client = BasicClient::new(ClientId::new(config.keycloak_client_id.to_string()))
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_device_authorization_url(device_auth_url);

    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");

    // Request device code and verification URI
    let details: StandardDeviceAuthorizationResponse = client
        .exchange_device_code()
        .add_scope(Scope::new("read".to_string()))
        .request_async(&http_client)
        .await?;

    // Build the verification URL properly with URL encoding.
    let mut verification_url = Url::parse(details.verification_uri().as_str())?;
    verification_url
        .query_pairs_mut()
        .append_pair("verification_uri_complete", details.user_code().secret());

    open::that(verification_url.as_str())?;
    println!(
        "Open this URL in your browser:\n{}",
        verification_url.as_str()
    );

    // Now we poll for the token
    println!("Waiting for user to authenticate...");

    loop {
        match client
            .exchange_device_access_token(&details)
            .request_async(
                &http_client,
                |duration| async move {
                    tokio::time::sleep(duration).await;
                },
                None,
            )
            .await
        {
            Ok(token) => {
                println!("Access Token: {}", token.access_token().secret());
                return Ok(token.access_token().secret().to_owned());
            }
            Err(RequestTokenError::ServerResponse(e))
                if e.error() == &oauth2::DeviceCodeErrorResponseType::AuthorizationPending =>
            {
                // Still waiting; retry after sleeping
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
}
