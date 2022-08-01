use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "targeter")]
pub struct Opt {
    /// Input file
    #[structopt(short, long, parse(from_os_str), default_value = ".")]
    pub input: std::path::PathBuf,

    /// Number of threads to use for tasks (default: 0, meaning auto detect)
    #[structopt(short, long, default_value = "0")]
    pub jobs: usize
}
