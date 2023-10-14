use clap::Parser;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None,)]
// add `disable_help_flag = true` to command so that `horz_overlap_mm` and `help` do not have the same short flag
pub struct Args {
    /// Name of the person to greet
    #[arg(long, default_value_t = 20.0)]
    pub horz_overlap_mm: f64,

    #[arg(short, long, default_value_t = 20.0)]
    pub vert_overlap_mm: f64,
}
