use utoipa::{openapi::schema, ToSchema};

#[derive(Debug, ToSchema)]
pub struct UploadPhoto {
    #[schema(value_type = String, format = Binary)]
    pub photo: Vec<u8>,
}
