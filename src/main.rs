mod commands;
mod list_ports;
mod programmer;

use crate::commands::Commands;
use crate::list_ports::list_ports;
use crate::programmer::program_pic::program_pic;
use clap::Parser;

fn main() {
    let cli = commands::Cli::parse();

    match &cli.command {
        Commands::ListPorts => {
            list_ports()
        }
        Commands::Program { input_file_path, port_name, baud_rate, timeout } => {
            program_pic(input_file_path, port_name, baud_rate, timeout);
        }
    }
}
