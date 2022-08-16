# NeoCal - Google Calendar Command Line Interface

**NeoCal** is an application that allows you to access your Google Calendar(s) from a command line. It uses Google Apps Script platform to build his own data source, making
it less complicated to setup than using OAUTH config tokens.

## Installation

You can either install it via `cargo` or download the binaries from GitHub releases.

If you go the `cargo` route, you need to have it installed (usually using [rustup](https://rustup.rs)). In a terminal, run this command to install `neocal`:

```
cargo install neocal
```

Then you'll be able to run `neocal` from whichever directory you're in.

If you want other installation option, please go to the [Wiki Page](https://github.com/oscarmcm/neocal/wiki/Installing-Options)

### SetUp

Once you have it installed you need to follow 3 steps in order to complete the configuration:

1. [Get Calendar IDs](https://github.com/oscarmcm/neocal/wiki/Obtain-your-Google-Calendar’s-ID)
2. [Setup Google Apps Script](https://github.com/oscarmcm/neocal/wiki/Setup-Google-Apps-Script)
3. [Create Config File](https://github.com/oscarmcm/neocal/wiki/NeoCal-Config-File)

## How-To

**NeoCal** provides a series of subcommands with the following functionality:

```
agenda      Shows user calendar in Agenda-like view mode
calendar    Shows user calendar in Calendar-like view mode
help        Print this message or the help of the given subcommand(s)
```

And also comes with the following options:

```
-f, --for <FOR>              Name of the calendar to use
-h, --help                   Print help information
-s, --search <SEARCH>        Word to search in the calendar
-t, --timezone <TIMEZONE>    Name of the Time Zone to return the events
    --today                  Get calendar entries for today
    --tomorrow               Get calendar entries for tomorrow
-V, --version                Print version information
    --week                   Get calendar entries for the current week
```

Run with `--help/-h` for detailed usage.

## To-Do

List of things to add support in order of importance:

### High

1. Add Calendar view

### Medium

1. Allow table style choices
2. List User Calendar

### Low

1. Create events from the CLI
2. Edit events from the CLI
3. Delete events from the CLI
4. Event Popup Reminders
5. Theme Support
6. Colorized output

