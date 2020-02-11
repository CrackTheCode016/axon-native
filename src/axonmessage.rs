pub mod axonmessage {
    use serde::{Deserialize, Serialize};
    use serde::de::DeserializeOwned;
    use serde_json::Error as SerdeError;

    // marker trait for sendable objects.
    pub trait Sendable {}
    pub trait Recievable {}

    pub trait AxonMessage: Serialize + DeserializeOwned {
        fn to_json_string(&self) -> Result<String, SerdeError>
        where
            Self: Serialize,
        {
            serde_json::to_string(&self)
        }
    }

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Message<T> {
        AxonMessage(T),
        Empty,
        Log { status: i8, data: String },
    }

    impl<T: Serialize + DeserializeOwned> AxonMessage for Message<T> {}
}
