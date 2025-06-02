use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoginImageResponse {
    pub image_url: String,
}

impl LoginImageResponse {
    pub fn new(image_url: String) -> Self {
        Self { image_url }
    }

    pub fn none() -> Self {
        Self {
            image_url: "".to_string(),
        }
    }
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct EditSettingParam {
    pub value: String,
}
