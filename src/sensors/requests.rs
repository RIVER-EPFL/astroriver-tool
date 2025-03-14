use super::models::Sensor;
use oauth2::reqwest;

pub(crate) async fn request_sensors(token: String) -> Result<Vec<Sensor>, String> {
    let client = reqwest::Client::new();
    let url = "https://astroriver-dev.epfl.ch/api/sensors?filter=%7B%7D&range=%5B0%2C24%5D&sort=%5B%22id%22%2C%22ASC%22%5D";
    let response = client
        .get(url)
        .header("authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let json = response.text().await.map_err(|e| e.to_string())?;
    let sensors: Vec<Sensor> = serde_json::from_str(&json).map_err(|e| e.to_string())?;

    Ok(sensors)
}
