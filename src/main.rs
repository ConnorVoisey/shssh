use std::process::{Command, ExitCode, Stdio};

use clap::Parser;
use shssh::{
    config::{get_config, Config, Server},
    args::{Args, LsArgs},
};

fn main() -> ExitCode {
    let args = Args::parse();
    let config = match get_config() {
        Some(val) => val,
        None => {
            return ExitCode::FAILURE;
        }
    };
    match args.subcommand {
        shssh::args::Command::Ssh(server_name) => match server_name.name {
            Some(server_name) => ssh(&server_name, &config),
            None => return ExitCode::FAILURE,
        },
        shssh::args::Command::Ls(args)=> {
            dbg!(args);
        }
    };
    ExitCode::SUCCESS
}

fn ssh(server_name: &str, config: &Config) {
    let server = search_server(server_name, &config.servers).expect("server name is not valid");
    let user = &server.users[0];
    let ssh_command = format!(
        "sshpass -p {} ssh {}@{}",
        user.password, user.name, server.host
    );

    let mut child = Command::new("sh")
        .arg("-c")
        .arg(&ssh_command)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to execute SSH command");

    let status = child.wait().expect("Failed to wait for SSH command");

    if !status.success() {
        eprintln!(
            "Error: SSH command failed with exit code {:?}",
            status.code().unwrap()
        );
    }
}

fn search_server<'a>(name: &'a str, servers: &'a [Server]) -> Option<&'a Server> {
    servers.iter().find(|&server| server.key == name)
}

fn ls(args: &LsArgs, config: &Config){

}
