use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum Message {
    Login,
    LoginCompleted(Result<String, String>),
    Logout,
    SensorsRequested(Result<Vec<crate::sensors::Sensor>, String>),
    ShowSensorsPage,
    BackToMain,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Page {
    #[default]
    Main,
    Sensors,
}

#[derive(Default)]
pub struct AppState {
    pub login_token: Option<String>,
    pub login_status: LoginStatus,
    pub login_payload: Option<Claims>,
    pub sensors: Vec<crate::sensors::Sensor>,
    pub current_page: Page,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LoginStatus {
    #[default]
    LoggedOut,
    LoggingIn,
    LoggedIn,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub auth_time: usize,
    pub sub: String,
    pub email: String,
    pub family_name: String,
    pub given_name: String,
    pub preferred_username: String,
    pub scope: String,
}
