use clap::ArgMatches;

pub(crate) struct SharedArgs<'a> {
    pub width: usize,
    pub height: usize,
    pub tilesize: Option<&'a usize>,
    pub tilewidth: Option<&'a usize>,
    pub tileheight: Option<&'a usize>,
    pub use_weights: bool,
    pub use_transforms: bool,
    pub max_contradictions: Option<&'a usize>,
}

impl<'a> From<&'a ArgMatches> for SharedArgs<'a> {
    fn from(matches: &'a ArgMatches) -> SharedArgs<'a> {
        SharedArgs {
            width: *matches.get_one::<usize>("width").unwrap(),
            height: *matches.get_one::<usize>("height").unwrap(),
            tilesize: matches.get_one::<usize>("tilesize"),
            tilewidth: matches.get_one::<usize>("tilewidth"),
            tileheight: matches.get_one::<usize>("tileheight"),
            use_weights: !matches.get_flag("noweights"),
            use_transforms: !matches.get_flag("notransforms"),
            max_contradictions: matches.get_one::<usize>("attempts"),
        }
    }
}

/// Applies several shared arguments for different subcommands to the given `clap::Command` provided.
macro_rules! expand_shared_args {
    ($e:expr) => {
        $e
            .arg(Arg::new("width")
                .required(true)
                .value_parser(value_parser!(usize)))
            .arg(Arg::new("height")
                .required(true)
                .value_parser(value_parser!(usize)))
            .arg(arg!( -m --tilesize <number> "Specify the tile size used in the analysis and result. By default this value is 1." )
                .value_parser(value_parser!(usize)))
            .arg(arg!( -j --tilewidth <number> "Specify the tile size width (precedent over --tilesize)." )
                .value_parser(value_parser!(usize)))
            .arg(arg!( -k --tileheight <number> "Specify the tile size height (precedent over --tilesize)." )
                .value_parser(value_parser!(usize)))
            .arg(arg!( -a --attempts <number> "The maximum number of contradictions (attempts) that can be reached before the program quits.")
                .value_parser(value_parser!(usize)))
            .arg(arg!( -w --noweights "Disables using weights in when calculating superposition entropy."))
            .arg(arg!( -t --notransforms "Disables using transforms in rule analysis."))
    }
}

pub(crate) use expand_shared_args;
