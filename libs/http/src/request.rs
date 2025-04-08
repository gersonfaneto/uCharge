use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::http::HttpMethod;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub method: HttpMethod,
    pub uri: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmBody {
    pub token: String,
    pub plate: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectBody {
    pub position: (f32, f32),
    pub power: f32,   // kW
    pub price: f32,   // price per kWh
    pub name: String, // franchise name
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBody {
    pub old_position: (f32, f32),
    pub old_power: f32,   // kW
    pub old_price: f32,   // price per kWh
    pub old_name: String, // franchise name

    pub new_position: (f32, f32),
    pub new_power: f32,   
    pub new_price: f32,  
    pub new_name: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginBody {
    pub username: String,
    pub password: String,
}

