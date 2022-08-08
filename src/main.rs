use configparser::ini::Ini;
use std::error::Error;
use std::env;
use clap::Parser;
use serde::{Deserialize, Serialize};
use reqwest;
use term_table::TableStyle;
use term_table::Table;
use term_table::row::Row;
use term_table::table_cell::{TableCell, Alignment};

#[derive(Serialize, Deserialize, Debug)]
struct Event {
    summary: String,
    description: String,
    start: String,
    end: String,
    call: String,
}

#[derive(Parser, Debug)]
#[clap(author="Oscar Cortez", version, about="Google Calendar CLI", long_about = None)]
struct Args {
   /// Type of the calendar view to use
   view: Option<String>,

   /// Name of the calendar to use
   #[clap(short, long, value_parser, forbid_empty_values = true, validator = validate_calendar_name)]
   calendar: Option<String>,
}



fn validate_calendar_name(name: &str) -> Result<(), String> {
    if name.trim().len() != name.len() {
        Err(String::from(
            "Values cannot have leading and trailing space",
        ))
    } else {
        Ok(())
    }
}

fn agenda_view(events: Vec<Event>) {
    let mut table = Table::new();
    table.style = TableStyle::extended();
    table.max_column_width = 40;
    for event in events.iter() {
        table.add_row(Row::new(vec![
            TableCell::new(&event.summary),
            TableCell::new(&event.description)
        ]));
    };
    println!("{}", table.render());
}

fn calendar_view(events: Vec<Event>) {
    println!("Calendar View");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let client = reqwest::Client::new();
    let mut config = Ini::new();

    let config_path = match env::var("NEOCAL_CONFIG_PATH") {
        Ok(val) => val,
        Err(_e) => ".config/neocal/config.ini".to_string(),
    };
    config.load(config_path)?;

    let calendar_to_use = args.calendar.unwrap_or(
        config.get("neocal", "default").unwrap()
    );
    let view_to_use = args.view.unwrap_or(
        config.get("neocal", "mode").unwrap()
    );

    let endpoint = config.get(&calendar_to_use, "endpoint").unwrap();
    let request = client
        .get(endpoint)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application/json")
        .send()
        .await
        .unwrap();

    match request.status() {
        reqwest::StatusCode::OK => {
            match request.json::<Vec<Event>>().await {
                Ok(events) => {
                    match view_to_use.as_str() {
                        "agenda" => agenda_view(events),
                        "calendar" => calendar_view(events),
                        _ => println!("Unrecognized view type"),
                    };
                },
                Err(err) => println!("The response didn't match the shape we expected. {:?}", err),
            };
        },
        reqwest::StatusCode::NOT_FOUND => {
            println!("Looks like the Calendar URL does not exists.");
        },
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Looks like the Calendar URL is not public.");
        },
        _ => {
            panic!("Uh oh! Something unexpected happened.");
        },
    };

    Ok(())
}
