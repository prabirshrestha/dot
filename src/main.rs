extern crate ansi_term;
extern crate clap;
extern crate shellexpand;
extern crate toml;

mod cli;
mod config;
mod entry;
mod util;

use std::env;
use config::Config;
use entry::EntryStatus;


#[cfg(windows)]
fn startup() {
  if env::var("HOME").is_err() {
    env::set_var("HOME", env::home_dir().unwrap().to_str().unwrap());
  }
}

#[cfg(not(windows))]
fn startup() {}


pub fn main() {
  startup();

  let matches = cli::build_cli().get_matches();

  let dry_run = matches.is_present("dry-run");
  let verbose = matches.is_present("verbose");

  let ref mut config = Config::new();

  let exitcode = match matches.subcommand() {
    ("check", Some(m)) => command_check(config, m, verbose),
    ("link", Some(m)) => command_link(config, m, dry_run, verbose),
    ("clean", Some(m)) => command_clean(config, m, dry_run, verbose),
    (_, _) => unreachable!(),
  };
  std::process::exit(exitcode);
}


pub fn command_check(config: &mut Config, _: &clap::ArgMatches, verbose: bool) -> i32 {
  let mut num_unhealth = 0;
  for (linkfile, entries) in config.read_linkfiles() {
    if verbose {
      println!("{}",
               ansi_term::Style::new()
                 .bold()
                 .fg(ansi_term::Colour::Blue)
                 .paint(format!("Loading {} ...", linkfile)));
    }

    for ref entry in entries {
      let status = entry.status();
      if status != EntryStatus::Healthy {
        println!("{} {} ({:?})",
                 ansi_term::Style::new().bold().fg(ansi_term::Colour::Red).paint("✘"),
                 entry.dst.display(),
                 status);
        num_unhealth += 1;
      } else {
        if verbose {
          println!("{} {}\n  => {}",
                   ansi_term::Style::new().bold().fg(ansi_term::Colour::Green).paint("✓"),
                   entry.dst.display(),
                   entry.src.display());
        }
      }
    }
  }

  num_unhealth
}

pub fn command_link(config: &mut Config,
                    _: &clap::ArgMatches,
                    dry_run: bool,
                    verbose: bool)
                    -> i32 {
  for (linkfile, content) in config.read_linkfiles() {
    if verbose {
      println!("{}",
               ansi_term::Style::new()
                 .bold()
                 .fg(ansi_term::Colour::Blue)
                 .paint(format!("Loading {} ...", linkfile)));
    }

    for ref entry in content {
      if entry.status() == EntryStatus::Healthy {
        if verbose {
          println!("{}\n  the link has already existed.", entry.dst.display());
        }
        continue;
      }
      if verbose {
        println!("{}\n  => {}", entry.dst.display(), entry.src.display());
      }
      util::make_link(&entry.src, &entry.dst, dry_run).unwrap();
    }
  }

  0
}

pub fn command_clean(config: &mut Config,
                     _: &clap::ArgMatches,
                     dry_run: bool,
                     verbose: bool)
                     -> i32 {
  for (linkfile, content) in config.read_linkfiles() {
    if verbose {
      println!("{}",
               ansi_term::Style::new()
                 .bold()
                 .fg(ansi_term::Colour::Blue)
                 .paint(format!("Loading {} ...", linkfile)));
    }

    for ref entry in content {
      if verbose {
        println!("unlink {}", entry.dst.display());
      }
      util::remove_link(&entry.dst, dry_run).unwrap();
    }
  }

  0
}
