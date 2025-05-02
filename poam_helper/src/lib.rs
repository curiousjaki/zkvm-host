#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct VerificationMetadata {
    pub image_id: [u32; 8],
    pub journal_data: (String, String), 
}