#[derive(Debug, Default)]
pub struct ConnectInfo {
    pub hex_id: Option<String>,
    pub angel_key: Option<String>,
    pub channel_id: u32,
    pub map_id: u32,
}