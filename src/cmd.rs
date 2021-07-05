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
                .default_value(".")
                .help("File/Path to watch"),
        )
    //.arg(
    //    Arg::with_name("ignore")
    //        .short("ignore")
    //        .long("ignore")
    //        .takes_value(true)
    //        .help("File/Path contained in the path provided to ignore"),
    //)
    //.arg(
    //    Arg::with_name("host")
    //        .short("host")
    //        .long("host")
    //        .takes_value(true)
    //        .help("Hostname for the http server"),
    //)
    //.arg(
    //    Arg::with_name("port")
    //        .short("p")
    //        .long("port")
    //        .takes_value(true)
    //        .default_value("80")
    //        .help("Port for the http server"),
    //)
}
