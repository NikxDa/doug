#[derive(Debug, Clone)]
pub struct EdnsOption {
    pub option_code: u16,
    pub option_length: u16,
    pub option_data: Vec<u8>,
}