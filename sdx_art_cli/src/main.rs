use anyhow::{Context, Result};
use clap::{App, AppSettings, Arg};

use generative_artistry::circle_packing::CirclePackingDescriptor;

fn main() -> Result<()> {
    let matches = App::new("sdx_art_cli")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(App::new("sdx").about("My own stuff"))
        .subcommand(
            App::new("ga")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .about("Generative Artistry Tutorial implementations")
                .subcommand(
                    App::new("tiled_lines")
                        .about("Tiled lines tutorial implementation of generative artistry"),
                )
                .subcommand(
                    App::new("circle_packing")
                        .about("Circle packing tutorial implementation of generative artistry")
                        .arg(
                            Arg::new("line_width")
                                .help("Line width of circles")
                                .short('l')
                                .long("line")
                                .value_name("LINE_WIDTH")
                                .takes_value(true)
                                .default_value("2.0"),
                        )
                        .arg(
                            Arg::new("min_radius")
                                .help("Minimum circle radius")
                                .short('f')
                                .long("min")
                                .value_name("MIN_RADIUS")
                                .takes_value(true)
                                .default_value("2"),
                        )
                        .arg(
                            Arg::new("max_radius")
                                .help("Maximum circle radius")
                                .short('t')
                                .long("max")
                                .value_name("MAX_RADIUS")
                                .takes_value(true)
                                .default_value("250"),
                        )
                        .arg(
                            Arg::new("total_circles")
                                .help("Total amount of circles")
                                .short('c')
                                .long("circles")
                                .value_name("TOTAL_CIRCLES")
                                .takes_value(true)
                                .default_value("1000"),
                        )
                        .arg(
                            Arg::new("create_circle_attempts")
                                .help("Attempts to create circles in empty space")
                                .short('a')
                                .long("attempts")
                                .value_name("CREATE_CIRCLE_ATTEMPTS")
                                .takes_value(true)
                                .default_value("500"),
                        ),
                ),
        )
        .get_matches();

    if let Some(args) = matches.subcommand_matches("ga") {
        if let Some(args) = args.subcommand_matches("circle_packing") {
            let line_width = args
                .value_of("line_width")
                .unwrap()
                .parse::<f32>()
                .context("Failed to parse line_width arg into a f32 type.")?;
            let min_radius = args
                .value_of("min_radius")
                .unwrap()
                .parse::<usize>()
                .context("Failed to parse min_radius arg into a usize type.")?;
            let max_radius = args
                .value_of("max_radius")
                .unwrap()
                .parse::<usize>()
                .context("Failed to parse max_radius arg into a usize type.")?;
            let total_circles = args
                .value_of("total_circles")
                .unwrap()
                .parse::<usize>()
                .context("Failed to parse total_circles arg into a usize type.")?;
            let create_circle_attempts = args
                .value_of("create_circle_attempts")
                .unwrap()
                .parse::<usize>()
                .context("Failed to parse create_circle_attempts arg into a usize type.")?;

            let descriptor = CirclePackingDescriptor::new(
                line_width,
                min_radius,
                max_radius,
                total_circles,
                create_circle_attempts,
            );

            generative_artistry::circle_packing::present(descriptor);
        }

        if let Some(_args) = args.subcommand_matches("tiled_lines") {
            generative_artistry::tiled_lines::present();
        }
    }

    if let Some(_args) = matches.subcommand_matches("sdx") {
        sdx_art::first_sketch::present();
    }

    Ok(())
}
