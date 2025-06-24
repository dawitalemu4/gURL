use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};
use serde_with::skip_serializing_none;
use validator::Validate;

pub fn serialize_favorites<S>(value: &Option<Vec<i32>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(favorites) => {
            let csv = favorites
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(",");
            serializer.serialize_str(&csv)
        }
        None => serializer.serialize_none(),
    }
}

pub fn deserialize_favorites<'de, D>(deserializer: D) -> Result<Option<Vec<i32>>, D::Error>
where
    D: Deserializer<'de>,
{
    match Option::<String>::deserialize(deserializer)? {
        Some(s) if !s.trim().is_empty() => {
            let vec = s
                .split(',')
                .map(|v| v.trim().parse::<i32>().map_err(D::Error::custom))
                .collect::<Result<Vec<_>, _>>()?;

            Ok(Some(vec))
        }
        _ => Ok(None),
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct User {
    #[validate(length(min = 1))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub password: String,
    #[serde(
        serialize_with = "serialize_favorites",
        deserialize_with = "deserialize_favorites"
    )]
    pub favorites: Option<Vec<i32>>,
    #[validate(length(min = 1))]
    pub date: Option<String>,
    #[serde(rename = "oldPassword")]
    #[validate(length(min = 1))]
    pub old_pw: String,
    pub deleted: bool,
}
