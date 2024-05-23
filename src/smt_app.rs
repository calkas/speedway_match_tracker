use super::smt_app_data::AppData;
pub struct App {
    pub is_running: bool,
    pub data: AppData,
}

impl Default for App {
    fn default() -> Self {
        Self {
            is_running: true,
            data: AppData::default(),
        }
    }
}

impl App {
    pub fn run(&self) {}
    pub async fn fetch_data(&mut self) {
        self.data.get_data_from_server().await;
    }
}
