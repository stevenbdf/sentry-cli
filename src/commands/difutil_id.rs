use std::io;
use std::path::Path;

use clap::{App, AppSettings, Arg, ArgMatches};
use failure::Error;

use crate::utils::dif::DifFile;
use crate::utils::system::QuietExit;

pub fn make_app(app: App) -> App {
    // this command is hidden.  It only returns debug ids which is not super useful.
    // it's recommended to use difutil check instead.
    app.about("Print debug identifier(s) from a debug info file.")
        .setting(AppSettings::Hidden)
        .alias("uuid")
        .arg(
            Arg::new("type")
                .long("type")
                .short('t')
                .value_name("TYPE")
                .possible_values(&["dsym", "elf", "proguard", "breakpad"])
                .about(
                    "Explicitly set the type of the debug info file. \
                     This should not be needed as files are auto detected.",
                ),
        )
        .arg(
            Arg::new("json")
                .long("json")
                .about("Format outputs as JSON."),
        )
        .arg(
            Arg::new("path")
                .index(1)
                .required(true)
                .about("The path to the debug info file."),
        )
}

pub fn execute(matches: &ArgMatches) -> Result<(), Error> {
    let path = Path::new(matches.value_of("path").unwrap());

    // which types should we consider?
    let ty = matches.value_of("type").map(|t| t.parse().unwrap());
    let f = DifFile::open_path(path, ty)?;

    if !f.is_usable() {
        eprintln!(
            "error: debug info file is not usable: {}",
            f.get_problem().unwrap_or("unknown error")
        );
        return Err(QuietExit(1).into());
    }

    if !matches.is_present("json") {
        for id in f.ids() {
            println!("{}", id);
        }
    } else if matches.is_present("json") {
        serde_json::to_writer_pretty(&mut io::stdout(), &f.ids())?;
        println!();
    }

    Ok(())
}
