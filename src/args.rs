use std::{env, result::Result};

use getopts::{Fail, Matches, Options as OptsOptions};
use thiserror::Error;

use crate::{
    consts,
    location::client::{LocationProvider, URL_LOCATIONS},
    options::Options,
    units::Units,
};

#[derive(Debug, Error)]
pub enum ArgsError {
    #[error(transparent)]
    GetOpts(#[from] Fail),
}

pub struct Args(Options);

pub type ArgsResult = Result<Options, ArgsError>;

impl Args {
    #[inline]
    fn new() -> OptsOptions {
        let mut opts = OptsOptions::new();
        opts.optflag("m", "metric", "Weather in metric units (compatibility)")
            .optflag("i", "imperial", "Weather in imperial units (compatibility)")
            .optopt(
                "u",
                "unit",
                "Unit of measurement",
                "[C]elsius or [F]ahrenheit",
            )
            .optopt("c", "connect-timeout", "Connect timeout (in seconds)", "5")
            .optopt("t", "timeout", "Timeout (in seconds)", "30")
            .optopt(
                "p",
                "location-provider",
                "Location provider",
                format!("0 to {}", URL_LOCATIONS.len() - 1).as_str(),
            )
            .optflag("f", "full-info", "Full weather information")
            .optflag("s", "silent", "Silent mode")
            .optflag("v", "version", "Print program version")
            .optflag("h", "help", "Print this help menu");
        opts
    }

    #[inline]
    fn parse_units(matches: &Matches) -> Option<Units> {
        if matches.opt_present("m") {
            Some(Units::Celsius)
        } else if matches.opt_present("i") {
            Some(Units::Fahrenheit)
        } else {
            let units = matches.opt_str("u").unwrap_or_default();
            if units.eq_ignore_ascii_case("C") {
                Some(Units::Celsius)
            } else if units.eq_ignore_ascii_case("F") {
                Some(Units::Fahrenheit)
            } else {
                None
            }
        }
    }

    #[inline]
    fn parse_connect_timeout(matches: &Matches) -> Option<u64> {
        matches.opt_get("c").unwrap_or_default()
    }

    #[inline]
    fn parse_timeout(matches: &Matches) -> Option<u64> {
        matches.opt_get("t").unwrap_or_default()
    }

    #[inline]
    fn parse_query(matches: &Matches) -> Option<String> {
        if matches.free.is_empty() {
            None
        } else {
            Some(matches.free[0].clone())
        }
    }

    #[inline]
    fn parse_location_provider(matches: &Matches) -> Option<LocationProvider> {
        matches.opt_get("p").unwrap_or_default()
    }

    #[inline]
    fn parse_full_info(matches: &Matches) -> Option<bool> {
        if matches.opt_present("f") {
            Some(true)
        } else {
            None
        }
    }

    #[inline]
    fn parse_silent(matches: &Matches) -> Option<bool> {
        if matches.opt_present("s") {
            Some(true)
        } else {
            None
        }
    }

    #[inline]
    fn parse_version(matches: &Matches) -> Option<String> {
        if matches.opt_present("v") {
            Some(consts::PROGRAM_VERSION.to_owned())
        } else {
            None
        }
    }

    #[inline]
    fn parse_help(opts: &OptsOptions, matches: &Matches) -> Option<String> {
        if matches.opt_present("h") {
            Some(opts.usage(&format!(
                "Usage: {} [options] [city name[,state code][,country code]]",
                consts::PROGRAM_NAME
            )))
        } else {
            None
        }
    }

    pub fn parse(args: &[String]) -> ArgsResult {
        let opts = Self::new();
        let matches = opts.parse(args)?;
        let args = Self {
            0: Options {
                units: Self::parse_units(&matches),
                connect_timeout: Self::parse_connect_timeout(&matches),
                timeout: Self::parse_timeout(&matches),
                query: Self::parse_query(&matches),
                location_provider: Self::parse_location_provider(&matches),
                full_info: Self::parse_full_info(&matches),
                silent: Self::parse_silent(&matches),
                version: Self::parse_version(&matches),
                help: Self::parse_help(&opts, &matches),
            },
        };
        Ok(args.0)
    }

    pub fn parse_from_env() -> ArgsResult {
        let args: Vec<String> = env::args().collect();
        Self::parse(&args[1..])
    }
}

#[cfg(test)]
mod tests {
    use super::Args;
    use crate::units::Units;

    #[test]
    fn args_parse_unit() {
        let opt = Args::parse(&[]).unwrap();
        assert_eq!(opt.units, None);

        let opt = Args::parse(&["--metric".to_string()]).unwrap();
        assert_eq!(opt.units, Some(Units::Celsius));
        let opt = Args::parse(&["-m".to_string()]).unwrap();
        assert_eq!(opt.units, Some(Units::Celsius));

        let opt = Args::parse(&["--imperial".to_string()]).unwrap();
        assert_eq!(opt.units, Some(Units::Fahrenheit));
        let opt = Args::parse(&["-i".to_string()]).unwrap();
        assert_eq!(opt.units, Some(Units::Fahrenheit));

        let opt = Args::parse(&["--unit=C".to_string()]).unwrap();
        assert_eq!(opt.units, Some(Units::Celsius));
        let opt = Args::parse(&["-uC".to_string()]).unwrap();
        assert_eq!(opt.units, Some(Units::Celsius));
        let opt = Args::parse(&["-uc".to_string()]).unwrap();
        assert_eq!(opt.units, Some(Units::Celsius));
        let opt = Args::parse(&["--unit=F".to_string()]).unwrap();
        assert_eq!(opt.units, Some(Units::Fahrenheit));
        let opt = Args::parse(&["-uF".to_string()]).unwrap();
        assert_eq!(opt.units, Some(Units::Fahrenheit));
        let opt = Args::parse(&["-uf".to_string()]).unwrap();
        assert_eq!(opt.units, Some(Units::Fahrenheit));
    }

    #[test]
    fn args_parse_timeouts() {
        let opt = Args::parse(&[]).unwrap();
        assert_eq!(opt.connect_timeout, None);

        let opt = Args::parse(&["--connect-timeout=123".to_string()]).unwrap();
        assert_eq!(opt.connect_timeout, Some(123));
        let opt = Args::parse(&["-c123".to_string()]).unwrap();
        assert_eq!(opt.connect_timeout, Some(123));

        assert_eq!(opt.timeout, None);
        let opt = Args::parse(&["--timeout=123".to_string()]).unwrap();
        assert_eq!(opt.timeout, Some(123));
        let opt = Args::parse(&["-t123".to_string()]).unwrap();
        assert_eq!(opt.timeout, Some(123));
    }

    #[test]
    fn args_parse_query() {
        let opt = Args::parse(&[]).unwrap();
        assert_eq!(opt.query, None);

        let opt = Args::parse(&["".to_string()]).unwrap();
        assert_eq!(opt.query, Some("".to_string()));
        let opt = Args::parse(&["monteiro".to_string()]).unwrap();
        assert_eq!(opt.query, Some("monteiro".to_string()));
        let opt = Args::parse(&["joão pessoa".to_string()]).unwrap();
        assert_eq!(opt.query, Some("joão pessoa".to_string()));
        let opt = Args::parse(&["joão pessoa,paraíba".to_string()]).unwrap();
        assert_eq!(opt.query, Some("joão pessoa,paraíba".to_string()));
        let opt = Args::parse(&["joão pessoa,paraíba,brasil".to_string()]).unwrap();
        assert_eq!(opt.query, Some("joão pessoa,paraíba,brasil".to_string()));
    }

    #[test]
    fn args_parse_location_provider() {
        let opt = Args::parse(&[]).unwrap();
        assert_eq!(opt.location_provider, None);

        let opt = Args::parse(&["-p-1".to_string()]).unwrap();
        assert_eq!(opt.location_provider, Some(-1));
        let opt = Args::parse(&["-p0".to_string()]).unwrap();
        assert_eq!(opt.location_provider, Some(0));
        let opt = Args::parse(&["-p3".to_string()]).unwrap();
        assert_eq!(opt.location_provider, Some(3));
        let opt = Args::parse(&["--location-provider=-1".to_string()]).unwrap();
        assert_eq!(opt.location_provider, Some(-1));
        let opt = Args::parse(&["--location-provider=0".to_string()]).unwrap();
        assert_eq!(opt.location_provider, Some(0));
        let opt = Args::parse(&["--location-provider=3".to_string()]).unwrap();
        assert_eq!(opt.location_provider, Some(3));
    }

    #[test]
    fn args_parse_full_info() {
        let opt = Args::parse(&[]).unwrap();
        assert_eq!(opt.full_info, None);

        let opt = Args::parse(&["--full-info".to_string()]).unwrap();
        assert_eq!(opt.full_info, Some(true));
        let opt = Args::parse(&["-f".to_string()]).unwrap();
        assert_eq!(opt.full_info, Some(true));
    }

    #[test]
    fn args_parse_silent() {
        let opt = Args::parse(&[]).unwrap();
        assert_eq!(opt.silent, None);

        let opt = Args::parse(&["--silent".to_string()]).unwrap();
        assert_eq!(opt.silent, Some(true));
        let opt = Args::parse(&["-s".to_string()]).unwrap();
        assert_eq!(opt.silent, Some(true));
    }

    #[test]
    fn args_parse_version() {
        let opt = Args::parse(&[]).unwrap();
        assert_eq!(opt.version, None);

        let opt = Args::parse(&["--version".to_string()]).unwrap();
        assert_eq!(opt.version, Some(env!("CARGO_PKG_VERSION").to_string()));
        let opt = Args::parse(&["-v".to_string()]).unwrap();
        assert_eq!(opt.version, Some(env!("CARGO_PKG_VERSION").to_string()));
    }

    #[test]
    fn args_parse_help() {
        let opt = Args::parse(&[]).unwrap();
        assert_eq!(opt.help, None);

        let help = "Usage: weather [options] [city name[,state code][,country code]]

Options:
    -m, --metric        Weather in metric units (compatibility)
    -i, --imperial      Weather in imperial units (compatibility)
    -u, --unit [C]elsius or [F]ahrenheit
                        Unit of measurement
    -c, --connect-timeout 5
                        Connect timeout (in seconds)
    -t, --timeout 30    Timeout (in seconds)
    -p, --location-provider 0 to 3
                        Location provider
    -f, --full-info     Full weather information
    -s, --silent        Silent mode
    -v, --version       Print program version
    -h, --help          Print this help menu
";
        let opt = Args::parse(&["--help".to_string()]).unwrap();
        assert_eq!(opt.help, Some(help.to_string()));
        let opt = Args::parse(&["-h".to_string()]).unwrap();
        assert_eq!(opt.help, Some(help.to_string()));
    }
}
