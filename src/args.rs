use clap::{App, Arg};

#[derive(Default)]
pub struct AppArgs {
    pub target: String,
    pub no_css: bool,
    pub no_fonts: bool,
    pub no_frames: bool,
    pub no_images: bool,
    pub no_js: bool,
    pub insecure: bool,
    pub isolate: bool,
    pub output: String,
    pub silent: bool,
    pub timeout: u64,
    pub user_agent: String,
}

const DEFAULT_NETWORK_TIMEOUT: u64 = 120;
const DEFAULT_USER_AGENT: &str =
    "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:73.0) Gecko/20100101 Firefox/73.0";

impl AppArgs {
    pub fn get() -> AppArgs {
        let app = App::new(env!("CARGO_PKG_NAME"))
            .version(crate_version!())
            .author(crate_authors!("\n"))
            .about(crate_description!())
            .arg(
                Arg::with_name("target")
                    .required(true)
                    .takes_value(true)
                    .index(1)
                    .help("URL or file path"),
            )
            // .args_from_usage("-a, --include-audio 'Removes audio sources'")
            .args_from_usage("-c, --no-css 'Removes CSS'")
            .args_from_usage("-f, --no-frames 'Removes frames and iframes'")
            .args_from_usage("-F, --no-fonts 'Removes fonts'")
            .args_from_usage("-i, --no-images 'Removes images'")
            .args_from_usage("-I, --isolate 'Cuts off document from the Internet'")
            .args_from_usage("-j, --no-js 'Removes JavaScript'")
            .args_from_usage("-k, --insecure 'Allows invalid X.509 (TLS) certificates'")
            .args_from_usage("-o, --output=[document.html] 'Writes output to <file>'")
            .args_from_usage("-s, --silent 'Suppresses verbosity'")
            .args_from_usage("-t, --timeout=[60] 'Adjusts network request timeout'")
            .args_from_usage("-u, --user-agent=[Firefox] 'Sets custom User-Agent string'")
            // .args_from_usage("-v, --include-video 'Removes video sources'")
            .get_matches();
        let mut app_args = AppArgs::default();
        // Process the command
        app_args.target = app
            .value_of("target")
            .expect("please set target")
            .to_string();
        app_args.no_css = app.is_present("no-css");
        app_args.no_fonts = app.is_present("no-fonts");
        app_args.no_frames = app.is_present("no-frames");
        app_args.no_images = app.is_present("no-images");
        app_args.no_js = app.is_present("no-js");
        app_args.insecure = app.is_present("insecure");
        app_args.isolate = app.is_present("isolate");
        app_args.silent = app.is_present("silent");
        app_args.timeout = app
            .value_of("timeout")
            .unwrap_or(&DEFAULT_NETWORK_TIMEOUT.to_string())
            .parse::<u64>()
            .unwrap();
        app_args.output = app.value_of("output").unwrap_or("").to_string();
        app_args.user_agent = app
            .value_of("user-agent")
            .unwrap_or(DEFAULT_USER_AGENT)
            .to_string();
        app_args
    }
}
