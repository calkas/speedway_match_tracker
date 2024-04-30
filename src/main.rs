use speedway_match_tracker::smt_app_data::AppData;

#[tokio::main]
async fn main() {
    let mut app = AppData::default();
    app.get_data_from_server().await;

    println!("{}", app.match_information);
    println!("{}", app.table_super_league);
    println!("{}", app.table_1_league);
}
