use std::env;
use std::error::Error;
use std::path::Path;

use chrono::DateTime;
use configparser::ini::Ini;
use clap::Parser;
use home;
use reqwest;
use serde::{Deserialize, Serialize};
use term_table::TableStyle;
use term_table::Table;
use term_table::row::Row;
use term_table::table_cell::{TableCell, Alignment};
use term_size;


#[derive(Serialize, Deserialize, Debug)]
struct Event {
    summary: String,
    description: String,
    start_date: String,
    start_date_time: String,
    end_date: String,
    end_date_time: String,
    call: String,
}

#[derive(Parser, Debug)]
#[clap(author="Oscar Cortez <om.cortez.2010@gmail.com>", version, about="Google Calendar CLI", long_about = None)]
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
    let mut event_date = "";

    table.style = TableStyle::blank();
    table.max_column_width =
        if let Some((w, _h)) = term_size::dimensions() {
            w - 60
        } else {
            80
        };

    for event in events.iter() {
        let event_time =
            if &event.start_date_time != "" && &event.end_date_time != "" {
                format!(
                    "{} - {}",
                    DateTime::parse_from_rfc3339(&event.start_date_time).unwrap().format("%H:%M").to_string(),
                    DateTime::parse_from_rfc3339(&event.end_date_time).unwrap().format("%H:%M").to_string(),
                )
            } else {
                "All Day".to_string()
            };

        let mut table_row = [
            TableCell::new(""),
            TableCell::new_with_alignment(event_time, 1, Alignment::Right),
            TableCell::new_with_alignment(&event.summary, 1, Alignment::Left),
        ];
        if event_date != &event.start_date {
            table.add_row(Row::new(vec![
                TableCell::new(""),
                TableCell::new(""),
                TableCell::new(""),
            ]));
            table_row[0] = TableCell::new(&event.start_date);
        };
        table.add_row(Row::new(table_row));
        event_date = &event.start_date;
    };

    println!("{}", table.render());
}

//fn calendar_view(events: Vec<Event>) {
//    println!("Calendar View");
//}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let client = reqwest::Client::new();
    let mut config = Ini::new();

    let config_path = match env::var("NEOCAL_CONFIG_PATH") {
        Ok(val) => val,
        Err(_e) => {
            match home::home_dir() {
                Some(path) => Path::join(&path, Path::new("./.config/neocal/config.ini")).to_str().unwrap().to_string(),
                None => panic!("Impossible to get your home dir!"),
            }
        },
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
                        "calendar" => println!("Not implemented at this moment."),
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
            println!("Uh oh! Something unexpected happened.");
        },
    };

    Ok(())
}
