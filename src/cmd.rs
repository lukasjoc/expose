use clap::{
    crate_authors, //
    crate_description,
    crate_name,
    crate_version,
    App,
    AppSettings,
    Arg,
};

// build the command with all options
pub fn build_cmd() -> App<'static, 'static> {
    // adding all the meta data to the cli interface
    let new_app = App::new(crate_name!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!());

    // adding all the arguments
    new_app
        // providing a path to watch files and folders
        // in required mode
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .required(true)
                .takes_value(true)
                .help("Path to dir or file to watch all desired contents recursively"),
        )
        // providing a regex pattern to ignore certain files and folders
        // for examples see readme
        .arg(
            Arg::with_name("ignore")
                .short("i")
                .long("ignore")
                .takes_value(true)
                .default_value(r"^\..*")
                .help(r"Ignore certain files using a defined regex pattern"),
        )
}
