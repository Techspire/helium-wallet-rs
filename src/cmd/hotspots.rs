use crate::{
    cmd::{api_url, collect_addresses, Opts},
    result::Result,
};
use helium_api::{Client, Hotspot};
use prettytable::{format, Table};
use structopt::StructOpt;
use super::CmdRunner;

/// Get the hotspots for a wallet
#[derive(Debug, StructOpt)]
pub struct Cmd {
    /// Addresses to get hotspots for
    #[structopt(short = "a", long = "address")]
    addresses: Vec<String>,
}

impl CmdRunner for Cmd {
    fn run(&self, opts: Opts) -> Result {
        let client = Client::new_with_base_url(api_url());
        let mut results: Vec<(String, Result<Vec<Hotspot>>)> =
            Vec::with_capacity(self.addresses.len());
        for address in collect_addresses(opts.files, self.addresses.clone())? {
            results.push((address.to_string(), client.get_hotspots(&address)));
        }
        print_results(results);
        Ok(())
    }
}

fn print_results(results: Vec<(String, Result<Vec<Hotspot>>)>) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(row![
        "Address", "Name", "Location", "City", "State", "Score"
    ]);

    for (address, result) in results {
        #[allow(clippy::unused_unit)]
        match result {
            Ok(hotspots) if hotspots.is_empty() => {
                table.add_row(row![address, H5 -> "No hotspots found".to_string()]);
                ()
            }
            Ok(hotspots) => {
                for hotspot in hotspots {
                    table.add_row(row![
                        hotspot.address,
                        hotspot.name.unwrap_or_else(|| "unknown".to_string()),
                        hotspot.location.unwrap_or_else(|| "uknnown".to_string()),
                        hotspot
                            .geocode
                            .short_city
                            .unwrap_or_else(|| "unknown".to_string()),
                        hotspot
                            .geocode
                            .short_state
                            .unwrap_or_else(|| "unknown".to_string()),
                        hotspot.score
                    ]);
                }
            }
            Err(err) => {
                table.add_row(row![address, H5 -> err.to_string()]);
                ()
            }
        };
    }
    table.printstd();
}
