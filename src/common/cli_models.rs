use structopt::StructOpt;
#[derive(StructOpt)]
pub struct Cli {
    #[structopt(parse(from_os_str))]
    pub path: std::path::PathBuf,
}
