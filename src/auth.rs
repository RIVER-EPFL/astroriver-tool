use crate::config::Config;
use oauth2::basic::BasicClient;
use oauth2::reqwest;
use oauth2::{
    AuthUrl, ClientId, DeviceAuthorizationUrl, Scope, StandardDeviceAuthorizationResponse,
    TokenResponse, TokenUrl,
};
use oauth2::{DeviceCodeErrorResponseType, RequestTokenError};

pub async fn start_device_auth_flow()
-> Result<StandardDeviceAuthorizationResponse, Box<dyn std::error::Error + Send + Sync>> {
    let config = Config::from_env();
    let client = BasicClient::new(ClientId::new(config.keycloak_client_id.to_string()))
        .set_auth_uri(AuthUrl::new(format!(
            "{}/realms/{}/protocol/openid-connect/auth",
            config.keycloak_url, config.keycloak_realm
        ))?)
        .set_token_uri(TokenUrl::new(format!(
            "{}/realms/{}/protocol/openid-connect/token",
            config.keycloak_url, config.keycloak_realm
        ))?)
        .set_device_authorization_url(DeviceAuthorizationUrl::new(format!(
            "{}/realms/{}/protocol/openid-connect/auth/device",
            config.keycloak_url, config.keycloak_realm
        ))?);

    let http_client = reqwest::Client::new();
    let details = client
        .exchange_device_code()
        .add_scope(Scope::new("read".to_string()))
        .request_async(&http_client)
        .await?;

    let verification_uri = details.verification_uri_complete();
    open::that(verification_uri.unwrap().secret())?;

    Ok(details)
}

pub async fn poll_for_device_token(
    details: StandardDeviceAuthorizationResponse,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let config = Config::from_env();
    let client = BasicClient::new(ClientId::new(config.keycloak_client_id.to_string()))
        .set_token_uri(TokenUrl::new(format!(
            "{}/realms/{}/protocol/openid-connect/token",
            config.keycloak_url, config.keycloak_realm
        ))?);

    let http_client = reqwest::Client::new();

    loop {
        match client
            .exchange_device_access_token(&details)
            .request_async(
                &http_client,
                |d| async move {
                    tokio::time::sleep(d).await;
                },
                None,
            )
            .await
        {
            Ok(token) => return Ok(token.access_token().secret().to_owned()),
            Err(RequestTokenError::ServerResponse(e))
                if e.error() == &DeviceCodeErrorResponseType::AuthorizationPending =>
            {
                continue;
            }
            Err(e) => return Err(e.into()),
        }
    }
}

pub async fn login_flow() -> Result<String, String> {
    match start_device_auth_flow().await {
        Ok(details) => {
            if let Err(e) = open::that(details.verification_uri_complete().unwrap().secret()) {
                return Err(e.to_string());
            }
            match poll_for_device_token(details).await {
                Ok(token) => Ok(token),
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
