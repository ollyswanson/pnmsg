use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "pnmsg")]
pub enum Command {
    #[structopt(name = "encode")]
    Encode {
        file: PathBuf,
        chunk_type: String,
        message: String,
        output_file: Option<PathBuf>,
    },
    #[structopt(name = "decode")]
    Decode { file: PathBuf, chunk_type: String },
    #[structopt(name = "remove")]
    Remove { file: PathBuf, chunk_type: String },
    #[structopt(name = "print")]
    Print { file: PathBuf, chunk_type: String },
}
