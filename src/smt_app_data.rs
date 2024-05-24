use reqwest::Error;
use scraper::{Html, Selector};

struct SpeedwayScoreTable {
    team_name: String,
    match_points: u8,
}

async fn get_speedway_match_report() -> Result<String, Error> {
    let res = reqwest::get("https://sportowefakty.wp.pl/zuzel").await?;
    let text = res.text().await?;
    let document = Html::parse_document(&text);

    let selector = Selector::parse(r"#app > header > div:nth-child(3)").unwrap();

    let inner: String = document.select(&selector).flat_map(|e| e.text()).collect();

    let mut result = String::new();
    for e in inner.split("Przejdź do relacji") {
        let temp = format!("{}\n", e);
        if temp == String::from("  Wszystkie wyniki                                   \n") {
            continue;
        }
        result.push_str(&temp);
    }

    result.push('\n');
    Ok(result)
}

async fn get_speedway_table(url: String) -> Result<String, Error> {
    let res = reqwest::get(url).await?;
    let text = res.text().await?;
    let document = Html::parse_document(&text);

    let selector = Selector::parse(r"body > div.conter > i > div.overflowing > div > div.smallpadding_box > table.tabela_ligowa.striped_table").unwrap();

    let mut team_score: Vec<SpeedwayScoreTable> = Vec::new();
    let mut team_element = SpeedwayScoreTable {
        team_name: String::new(),
        match_points: 0,
    };

    let score_index: usize = 10;
    let mut index: usize = 0;
    let mut start_counting = false;

    let skip: usize = 27;

    for elem in document.select(&selector) {
        for (e, txt) in elem.text().enumerate() {
            if e < 27 {
                continue;
            }
            if txt.chars().nth(0).unwrap().is_alphabetic()
                && txt.chars().last().unwrap().is_alphabetic()
            {
                team_element.team_name = txt.to_string();
                start_counting = true;
                index = e;
            }

            if start_counting {
                if e == index + score_index {
                    team_element.match_points = txt.parse().unwrap();

                    start_counting = false;
                    team_score.push(SpeedwayScoreTable {
                        team_name: team_element.team_name.clone(),
                        match_points: team_element.match_points,
                    });

                    team_element.team_name.clear();
                    team_element.match_points = 0;
                }
            }
        }
    }
    let mut result = String::new();

    for e in team_score.iter().enumerate() {
        let temp = format!("{}. {} - {}\n", e.0 + 1, e.1.team_name, e.1.match_points);
        result.push_str(&temp);
    }
    result.push('\n');
    Ok(result)
}

pub struct AppData {
    pub match_information: String,
    pub table_super_league: String,
    pub table_1_league: String,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            match_information: Default::default(),
            table_super_league: Default::default(),
            table_1_league: Default::default(),
        }
    }
}

impl AppData {
    pub async fn get_data_from_server(&mut self) {
        let data_tasks = vec![
            tokio::spawn(get_speedway_match_report()),
            tokio::spawn(get_speedway_table(
                "https://www.zuzelend.com/tabela-pge-ekstraliga".to_string(),
            )),
            tokio::spawn(get_speedway_table(
                "https://www.zuzelend.com/tabela-1-liga".to_string(),
            )),
        ];

        let mut out_data = Vec::new();

        for task in data_tasks {
            out_data.push(task.await.expect("Data error:").unwrap());
        }

        self.table_1_league = out_data.pop().unwrap();
        self.table_super_league = out_data.pop().unwrap();
        self.match_information = out_data.pop().unwrap();
    }
}
