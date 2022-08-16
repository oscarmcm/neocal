use std::env;
use std::error::Error;
use std::path::Path;

use chrono::{DateTime, Datelike, Duration, NaiveDate, Weekday};
use clap::{Parser, Subcommand};
use configparser::ini::Ini;
use home;
use reqwest;
use serde::{Deserialize, Serialize};
use term_size;
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
};
use term_table::{Table, TableStyle};
use url::Url;

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

#[derive(Parser)]
#[clap(author="Oscar Cortez <om.cortez.2010@gmail.com>", version, about="Google Calendar CLI", long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
    #[clap(short, long, value_parser, forbid_empty_values = true, validator = validate_option_value)]
    /// Name of the calendar to use
    r#for: Option<String>,

    #[clap(short, long, value_parser, forbid_empty_values = true, validator = validate_option_value)]
    /// Word to search in the calendar
    search: Option<String>,

    #[clap(short, long, value_parser, forbid_empty_values = true, validator = validate_option_value)]
    /// Name of the Time Zone to return the events
    timezone: Option<String>,

    #[clap(short, long, default_value_t=u32::MIN)]
    /// Number of weeks to return the events starting from the current week
    weeks: u32,

    #[clap(long)]
    /// Get calendar entries for the current week
    week: bool,

    #[clap(long)]
    /// Get calendar entries for today
    today: bool,

    #[clap(long)]
    /// Get calendar entries for tomorrow
    tomorrow: bool,

    #[clap(short, long, parse(from_occurrences))]
    verbosity: usize,
}

#[derive(Subcommand)]
enum Commands {
    /// Shows user calendar in Agenda-like view mode
    Agenda {
        #[clap(short, long, value_parser, forbid_empty_values = true, validator = validate_option_value)]
        /// Name of the calendar to use
        r#for: Option<String>,

        #[clap(short, long, value_parser, forbid_empty_values = true, validator = validate_option_value)]
        /// Word to search in the calendar
        search: Option<String>,

        #[clap(short, long, value_parser, forbid_empty_values = true, validator = validate_option_value)]
        /// Name of the Time Zone to return the events
        timezone: Option<String>,

        #[clap(short, long, default_value_t=u32::MIN)]
        /// Number of weeks to return the events starting from the current week
        weeks: u32,

        #[clap(long)]
        /// Get calendar entries for the current week
        week: bool,

        #[clap(long)]
        /// Get calendar entries for today
        today: bool,

        #[clap(long)]
        /// Get calendar entries for tomorrow
        tomorrow: bool,
    },
    /// Show user calendar in Calendar-like view mode
    Calendar {
        #[clap(short, long, value_parser, forbid_empty_values = true, validator = validate_option_value)]
        /// Name of the calendar to use
        r#for: Option<String>,

        #[clap(short, long, value_parser, forbid_empty_values = true, validator = validate_option_value)]
        /// Word to search in the calendar
        search: Option<String>,

        #[clap(short, long, value_parser, forbid_empty_values = true, validator = validate_option_value)]
        /// Name of the Time Zone to return the events
        timezone: Option<String>,

        #[clap(short, long, default_value_t=u32::MIN)]
        /// Number of weeks to return the events starting from the current week
        weeks: u32,
    },
}

fn week_bounds(weeks_ahead: u32) -> (NaiveDate, NaiveDate) {
    let current_year = chrono::offset::Local::now().year();
    let current_week = chrono::offset::Local::now().iso_week().week();
    let mon = NaiveDate::from_isoywd(current_year, current_week, Weekday::Mon);
    let sun = NaiveDate::from_isoywd(current_year, current_week + weeks_ahead, Weekday::Sun);
    (mon, sun)
}

fn validate_option_value(name: &str) -> Result<(), String> {
    if name.trim().len() != name.len() {
        Err(String::from(
            "Values cannot have leading and trailing space",
        ))
    } else {
        Ok(())
    }
}

fn render_agenda_view(events: Vec<Event>) -> std::io::Result<()> {
    if events.iter().len() == 0 {
        eprintln!("No Events were found.");
    };

    let mut table = Table::new();
    let mut event_date = "";

    table.style = TableStyle::blank();
    table.max_column_width = if let Some((w, _h)) = term_size::dimensions() {
        w - 60
    } else {
        80
    };

    for event in events.iter() {
        let event_time = if &event.start_date_time != "" && &event.end_date_time != "" {
            format!(
                "{} - {}",
                DateTime::parse_from_rfc3339(&event.start_date_time)
                    .unwrap()
                    .format("%H:%M")
                    .to_string(),
                DateTime::parse_from_rfc3339(&event.end_date_time)
                    .unwrap()
                    .format("%H:%M")
                    .to_string(),
            )
        } else {
            "All Day".to_string()
        };

        let mut table_row = [
            TableCell::new(""),
            TableCell::new_with_alignment(event_time, 1, Alignment::Right),
            TableCell::new_with_alignment(
                format!("{}\n{}", &event.summary, &event.call),
                1,
                Alignment::Left,
            ),
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
    }

    println!("{}", table.render());
    return Ok(());
}

fn render_calendar_view(_events: Vec<Event>) -> std::io::Result<()> {
    eprintln!("This command is not yet implemented.");
    return Ok(());
}

async fn get_events(
    view_to_render: &str,
    url: &str,
    search: &str,
    timezone: &str,
    weeks: &u32,
    week: &bool,
    today: &bool,
    tomorrow: &bool,
) -> Result<(), Box<dyn Error>> {
    let mut endpoint = Url::parse(url)?;
    let client = reqwest::Client::new();

    if search.len() != 0 {
        endpoint.query_pairs_mut().append_pair("q", search);
    };
    if timezone.len() != 0 {
        endpoint.query_pairs_mut().append_pair("timeZone", timezone);
    };
    if *weeks != 0 {
        let (mon, sun) = week_bounds(*weeks);
        let time_min = format!("{}T00:00:00.000Z", mon.format("%Y-%m-%d"));
        let time_max = format!("{}T23:59:59.000Z", sun.format("%Y-%m-%d"));
        let query = format!("timeMin={}&timeMax={}", time_min, time_max);
        endpoint.set_query(Some(&query));
    };
    if *week {
        let (mon, sun) = week_bounds(0);
        let time_min = format!("{}T00:00:00.000Z", mon.format("%Y-%m-%d"));
        let time_max = format!("{}T23:59:59.000Z", sun.format("%Y-%m-%d"));
        let query = format!("timeMin={}&timeMax={}", time_min, time_max);
        endpoint.set_query(Some(&query));
    };
    if *today {
        let today = chrono::offset::Local::now();
        let time_min = format!("{}T00:00:00.000Z", today.format("%Y-%m-%d"));
        let time_max = format!("{}T23:59:59.000Z", today.format("%Y-%m-%d"));
        let query = format!("timeMin={}&timeMax={}", time_min, time_max);
        endpoint.set_query(Some(&query));
    };
    if *tomorrow {
        let tomorrow = chrono::offset::Local::now() + Duration::days(1);
        let time_min = format!("{}T00:00:00.000Z", tomorrow.format("%Y-%m-%d"));
        let time_max = format!("{}T23:59:59.000Z", tomorrow.format("%Y-%m-%d"));
        let query = format!("timeMin={}&timeMax={}", time_min, time_max);
        endpoint.set_query(Some(&query));
    };

    let request = client
        .get(endpoint.as_str())
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application/json")
        .send()
        .await
        .unwrap();

    match request.status() {
        reqwest::StatusCode::OK => match request.json::<Vec<Event>>().await {
            Ok(events) => match view_to_render {
                "agenda" => render_agenda_view(events),
                "calendar" => render_calendar_view(events),
                _ => Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Unrecognized view type",
                )),
            },
            Err(err) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("The response didn't match the shape we expected. {}", err),
            )),
        },
        reqwest::StatusCode::NOT_FOUND => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Looks like the Calendar URL does not exists.",
        )),
        reqwest::StatusCode::UNAUTHORIZED => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Looks like the Calendar URL is not public.",
        )),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Uh oh! Something unexpected happened.",
        )),
    };
    return Ok(());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut config = Ini::new();
    let config_path = match env::var("NEOCAL_CONFIG_PATH") {
        Ok(val) => val,
        Err(_e) => match home::home_dir() {
            Some(path) => Path::join(&path, Path::new("./.config/neocal/config.ini"))
                .to_str()
                .unwrap()
                .to_string(),
            None => "".to_string(),
        },
    };
    if config_path.len() == 0 {
        eprintln!("Impossible to get your home dir!");
        panic!();
    };
    config.load(config_path)?;

    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Agenda {
            r#for,
            search,
            timezone,
            weeks,
            week,
            today,
            tomorrow,
        }) => {
            let calendar_to_use = if r#for.is_none() {
                config.get("neocal", "default").unwrap()
            } else {
                r#for.as_ref().unwrap().to_string()
            };
            let url = &config.get(&calendar_to_use, "endpoint").unwrap();

            let mut selected_timezone = timezone.as_ref().unwrap_or(&"".to_string()).to_string();
            if selected_timezone.len() == 0 {
                selected_timezone = config
                    .get(&calendar_to_use, "timezone")
                    .unwrap_or("".to_string())
                    .to_string();
            };
            if selected_timezone.len() == 0 {
                selected_timezone = config
                    .get("neocal", "timezone")
                    .unwrap_or("".to_string())
                    .to_string();
            };
            if selected_timezone.len() == 0 {
                selected_timezone = "".to_string();
            };

            match get_events(
                &String::from("agenda"),
                url,
                &search.as_ref().unwrap_or(&"".to_string()).to_string(),
                &selected_timezone,
                &weeks,
                &week,
                &today,
                &tomorrow,
            )
            .await
            {
                Ok(_) => {}
                Err(err) => eprintln!("Error rendering Agenda - {}", err),
            }
        }
        Some(Commands::Calendar {
            r#for,
            search,
            timezone,
            weeks,
        }) => {
            let calendar_to_use = if r#for.is_none() {
                config.get("neocal", "default").unwrap()
            } else {
                r#for.as_ref().unwrap().to_string()
            };
            let url = &config.get(&calendar_to_use, "endpoint").unwrap();

            let mut selected_timezone = timezone.as_ref().unwrap_or(&"".to_string()).to_string();
            if selected_timezone.len() == 0 {
                selected_timezone = config
                    .get(&calendar_to_use, "timezone")
                    .unwrap_or("".to_string())
                    .to_string();
            };
            if selected_timezone.len() == 0 {
                selected_timezone = config
                    .get("neocal", "timezone")
                    .unwrap_or("".to_string())
                    .to_string();
            };
            if selected_timezone.len() == 0 {
                selected_timezone = "".to_string();
            };

            match get_events(
                &String::from("calendar"),
                url,
                &search.as_ref().unwrap_or(&"".to_string()).to_string(),
                &selected_timezone,
                &weeks,
                &false,
                &false,
                &false,
            )
            .await
            {
                Ok(_) => {}
                Err(err) => eprintln!("Error rendering Calendar - {}", err),
            }
        }
        None => {
            let view_to_use = config.get("neocal", "mode").unwrap();
            let calendar_to_use = cli
                .r#for
                .unwrap_or(config.get("neocal", "default").unwrap());
            let url = &config.get(&calendar_to_use, "endpoint").unwrap();
            let timezone = cli.timezone.unwrap_or(
                config
                    .get(&calendar_to_use, "timezone")
                    .unwrap_or(config.get("neocal", "timezone").unwrap_or("".to_string())),
            );
            match view_to_use.as_str() {
                "agenda" => match get_events(
                    &String::from("agenda"),
                    url,
                    &cli.search.unwrap_or("".to_string()).to_string(),
                    &timezone,
                    &cli.weeks,
                    &cli.week,
                    &cli.today,
                    &cli.tomorrow,
                )
                .await
                {
                    Ok(_) => {}
                    Err(err) => eprintln!("Error rendering Agenda - {}", err),
                },
                "calendar" => match get_events(
                    &String::from("calendar"),
                    url,
                    &cli.search.unwrap_or("".to_string()).to_string(),
                    &timezone,
                    &cli.weeks,
                    &false,
                    &false,
                    &false,
                )
                .await
                {
                    Ok(_) => {}
                    Err(err) => eprintln!("Error rendering Calendar - {}", err),
                },
                _ => eprintln!("Unrecognized view type."),
            }
        }
    };

    Ok(())
}
