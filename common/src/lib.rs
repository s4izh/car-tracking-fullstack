use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CarGeneralData {
    pub matricula: String,
    pub revoluciones: i32,
    pub nivel_gasolina: i32,
    pub nivel_aceite: i32,
}
