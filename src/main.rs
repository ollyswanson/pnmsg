use structopt::StructOpt;
mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = args::Command::from_args();

    println!("{:?}", opt);
    match opt {
        args::Command::Encode {
            file,
            chunk_type,
            message,
            output_file,
        } => commands::encode(file, chunk_type, message, output_file)?,
        args::Command::Decode { file, chunk_type } => commands::decode(file, chunk_type)?,
        _ => Err("Unknown command")?,
    }

    Ok(())
}
