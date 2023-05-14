use super::types::{ErrorResponse, User, UserLoginResponse, UserResponse, CarGeneralData, UserData2, Trip, UserTrip};
use reqwasm::http;
use gloo::console;
use serde_json::Value;

const BACKEND_URL: &'static str = std::env!("BACKEND_URL");

// Devuelve un Result ocn un JSONResponse o un String 


pub async fn api_car() -> Result<CarGeneralData, String> {

    let response = match http::Request::get("http://localhost:8080/api/frontend/test",)
        .header("Content-Type", "application/json")
     //   .header("Acces-Control-Request-Method","GET")
        
        .send()
        .await
    {
        Ok(res) => {
            res
        },
        Err(e ) => {
            console::log!("{}", e.to_string());
            return Err("Failed to make request".to_string())
        },
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
} 
    
pub async fn api_get_trip(user_data: &str) -> Result<UserTrip, String> {
    let response = match http::Request::post(format!("{}{}", BACKEND_URL, "/api/frontend/get-trips").as_str())
        .header("Content-Type", "application/json")
        .body(user_data)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn api_register_user(user_data: &str) -> Result<String, String> {
    let response = match http::Request::post(format!("{}{}", BACKEND_URL, "/api/create-user").as_str())
        .header("Content-Type", "application/json")
        .body(user_data)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_body = response.text().await;
    match res_body {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}



pub async fn api_login_user(credentials: &str) -> Result<UserData2, String> {
    let response = match http::Request::post(format!("{}{}", BACKEND_URL, "/api/login").as_str())
        .header("Content-Type", "application/json")
        .credentials(http::RequestCredentials::Include)
        .body(credentials)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

   // let res_body = response.text().await;
    let res_json = response.json::<UserData2>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn api_user_info() -> Result<User, String> {
    let response = match http::Request::get("http://localhost:8000/api/users/me")
        .credentials(http::RequestCredentials::Include)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json::<UserResponse>().await;
    match res_json {
        Ok(data) => Ok(data.data.user),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn api_certificate(credentials: &str) -> Result<String, String> {
    let response = match http::Request::post(format!("{}{}", BACKEND_URL, "/api/frontend/certificate").as_str())
        .header("Content-Type", "application/json")
        .credentials(http::RequestCredentials::Include)
        .body(credentials)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_body = response.text().await;
    match res_body {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}
