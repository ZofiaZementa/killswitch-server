use anyhow::{anyhow, bail, Context, Error, Result};
use std::{collections::BTreeMap, fmt, process::Command};

#[derive(Debug)]
pub struct VecError {
    errs: Vec<Error>,
}

impl VecError {
    fn new(errs: Vec<Error>) -> Self {
        VecError { errs }
    }
}

impl fmt::Display for VecError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for e in self.errs.iter() {
            writeln!(f, "{:?}", e)?;
        }
        Ok(())
    }
}

impl std::error::Error for VecError {}

pub fn kill_host(hostname: &str, ssh_config: &str) -> Result<()> {
    let res = Command::new("ssh")
        .args(["-F", ssh_config, hostname])
        .output()?;
    if res.status.success() {
        Ok(())
    } else {
        bail!(String::from_utf8(res.stderr)?)
    }
}

pub fn kill_hosts<'a, T>(hostnames: T, hosts: &BTreeMap<String, ()>, ssh_config: &str) -> Result<()>
where
    T: IntoIterator<Item = &'a str>,
{
    let ress: Vec<Error> = hostnames
        .into_iter()
        .map(|hostname| {
            hosts
                .get(hostname)
                .ok_or(anyhow!("{} not in hosts", hostname))?;
            kill_host(hostname, ssh_config)
                .with_context(|| format!("Kill failed for host {}", hostname))
        })
        .filter_map(|res| res.err())
        .collect();
    if ress.is_empty() {
        Ok(())
    } else {
        Err(VecError::new(ress).into())
    }
}

pub fn kill_all<'a, T>(hosts: T, ssh_config: &str) -> Result<()>
where
    T: IntoIterator<Item = &'a str>,
{
    let ress: Vec<Error> = hosts
        .into_iter()
        .filter_map(|hostname| {
            kill_host(hostname, ssh_config)
                .with_context(|| format!("Kill failed for host {}", hostname))
                .err()
        })
        .collect();
    if ress.is_empty() {
        Ok(())
    } else {
        Err(VecError::new(ress).into())
    }
}
