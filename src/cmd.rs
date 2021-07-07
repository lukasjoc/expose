use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

pub fn build_cmd() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("path")
                .short("path")
                .long("path")
                .takes_value(true)
                .help("File/Path to watch"),
        )
}
