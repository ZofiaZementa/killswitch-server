mod config;
mod kill;

use anyhow::Result;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

fn kill_s(cfg: &config::Config, sub_matches: &ArgMatches) -> Result<()> {
    if sub_matches.is_present("system") {
        kill::kill_hosts(
            sub_matches.values_of("system").unwrap(),
            &cfg.hosts,
            &cfg.ssh_config,
        )
    } else {
        kill::kill_all(cfg.hosts.keys().map(|hn| hn.as_str()), &cfg.ssh_config)
    }
}

fn main() -> Result<()> {
    let matches = App::new("Killswitch server")
        .author("Maximilian Fischer")
        .about("Provides a remote killswitch server for your devices")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(true)
                .default_value("/etc/killswitch/server.yaml")
                .help("Location of the config file"),
        )
        .subcommand(
            SubCommand::with_name("kill")
                .author("Maximilian Fischer")
                .about("Kill all or some systems")
                .arg(
                    Arg::with_name("system")
                        .multiple(true)
                        .takes_value(true)
                        .help("System(s) which to kill. If none is provided, all are killed"),
                ),
        )
        .setting(AppSettings::SubcommandRequired)
        .get_matches();

    let cfg = config::get_config(matches.value_of("config").unwrap())?;

    match matches.subcommand() {
        ("kill", Some(sub_matches)) => kill_s(&cfg, sub_matches),
        _ => panic!("No subcommand"),
    }
}
