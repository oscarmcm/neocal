# NeoCal - Google Calendar Command Line Interface

**NeoCal** is a Rust application that allows you to access your Google Calendar(s) from a command line. It uses Google Apps Script platform to build his own data source, making
it less complicated to setup than using OAUTH config tokens.

## Installation

You can either install it via `cargo` or download the binaries from GitHub releases.

If you go the `cargo` route, you need to have it installed (usually using [rustup](https://rustup.rs)). In a terminal, run this command to install `neocal`:

```
cargo install neocal
```
Then you'll be able to run `neocal` from whichever directory you in.

### SetUp

Once you have it installed you need to follow 3 steps in order to complete the configuration:

1. [Get Calendar IDs](https://github.com/oscarmcm/neocal/wiki/Obtain-your-Google-Calendar’s-ID:)
2. [Setup Google Apps Script](https://github.com/oscarmcm/neocal/wiki/Setup-Google-Apps-Script)
3. [Create Config File](https://github.com/oscarmcm/neocal/wiki/NeoCal-Config-File)

## How-To

**NeoCal** provides a series of subcommands with the following functionality:

```
list     - list available calendars
agenda   - get all the events for the current month in agenda format
calendar - get all the events for the current month in calendar format
```

And also comes with the following options:

```
-c / --calendar <NAME> - Name of the calendar to use
```

Run with `--help/-h` for detailed usage.

## To-Do

List of things to add support in order of importance:

### High

1. Support for using a custom TimeZone [ ]
2. Allow number of weeks to view [ ]
  - Should work for "Agenda" and "Calendar" mode
3. Agenda able to filter from Start to End [ ]
4. Produce builds for Linux and Windows [ ]

### Medium

1. Agenda support "Today" and "Tomorrow" modes [ ]
2. Allow table style choices [ ]
3. List User Calendar [ ]
4. Add Calendar view [ ]
5. Colorized output [ ]

### Low important tools

1. Create events from the CLI [ ]
2. Edit events from the CLI [ ]
3. Delete events from the CLI [ ]
4. Event Popup Reminders [ ]
5. Theme Support [ ]

