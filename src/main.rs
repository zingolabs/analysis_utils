use clap::Parser;
use plotters::{
    prelude::ChartBuilder,
    style::{self, Color},
};
use zingo_testutils::DurationAnnotation;
#[derive(Debug)]
struct Annotations(Vec<DurationAnnotation>);
impl Annotations {
    fn filter_on_testname(&self, name: &str) -> Annotations {
        let matches = self
            .0
            .clone()
            .into_iter()
            .filter(|da| da.test_name == name)
            .collect();
        Annotations(matches)
    }
    fn get_da_roof(&self) -> u128 {
        let durations = self
            .0
            .iter()
            .map(|da| da.duration.as_millis())
            .collect::<Vec<u128>>();
        let duration_max = durations.iter().fold(0, |acc, d| acc.max(*d));
        (duration_max >> 3) + duration_max
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    file: std::path::PathBuf,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get input
    let cli = Args::parse();
    let image_str = cli
        .file
        .to_str()
        .unwrap()
        .trim_end_matches(".json")
        .to_owned()
        + ".png";
    //let image_location = std::path::PathBuf::from(image_str);
    println!("Annotation Source File: {}", cli.file.to_str().unwrap());
    // load annotations
    let duration_annotations = Annotations(zingo_testutils::get_duration_annotations(cli.file));

    // viewonly_client_pu_false section:
    let keyless_client_pu_false = duration_annotations
        .filter_on_testname("sync_1153_baseline_synctimes_keyless_client_pu_false");
    let keyowning_client_pu_false = duration_annotations
        .filter_on_testname("sync_1153_baseline_synctimes_keyowning_client_pu_false");
    let fullviewonly_client_pu_false_das = duration_annotations
        .filter_on_testname("sync_1153_baseline_synctimes_fullviewonly_client_pu_false");
    if dbg!(fullviewonly_client_pu_false_das.0.len()) == 0
        || dbg!(keyless_client_pu_false.0.len()) == 0
        || dbg!(keyowning_client_pu_false.0.len()) == 0
    {
        panic!("Empty list!")
    }
    dbg!(&fullviewonly_client_pu_false_das);
    let keyless_duration_roof = keyless_client_pu_false.get_da_roof();
    let keyowning_duration_roof = keyowning_client_pu_false.get_da_roof();
    let full_duration_roof = fullviewonly_client_pu_false_das.get_da_roof();
    let first_duration_roof = keyless_duration_roof.max(full_duration_roof);
    let duration_roof = keyowning_duration_roof.max(first_duration_roof);
    //let das = viewonly_client_pu_false_das.0.len() as u128;
    // Begin plotting expressions
    use plotters::{backend, drawing};
    let root = drawing::IntoDrawingArea::into_drawing_area(backend::BitMapBackend::new(
        &image_str,
        (1024, 768),
    ));

    root.fill(&style::colors::WHITE)?;
    let areas = root.split_by_breakpoints([80], [688]);

    areas[0].fill(&style::colors::RED)?;
    areas[1].fill(&style::colors::YELLOW)?;
    areas[2].fill(&style::colors::GREEN)?;
    areas[3].fill(&style::full_palette::PURPLE)?;
    let mut scatter_ctx = plotters::chart::ChartBuilder::on(&areas[1])
        .build_cartesian_2d(0u128..4, 0u128..duration_roof)?;

    scatter_ctx.draw_series(keyless_client_pu_false.0.iter().enumerate().map(
        |(_, DurationAnnotation { duration, .. })| {
            plotters::prelude::Circle::new(
                (1, duration.as_millis()),
                2,
                plotters::style::BLUE.filled(),
            )
        },
    ))?;
    scatter_ctx.draw_series(fullviewonly_client_pu_false_das.0.iter().enumerate().map(
        |(_, DurationAnnotation { duration, .. })| {
            plotters::prelude::Circle::new(
                (2, duration.as_millis()),
                2,
                plotters::style::RED.filled(),
            )
        },
    ))?;
    scatter_ctx.draw_series(keyowning_client_pu_false.0.iter().enumerate().map(
        |(_, DurationAnnotation { duration, .. })| {
            plotters::prelude::Circle::new(
                (3, duration.as_millis()),
                2,
                plotters::style::GREEN.filled(),
            )
        },
    ))?;
    ChartBuilder::on(&root)
        .caption(
            "keyless_client, fvk_only_client, keyowning_client",
            ("sans-serif", 30),
        )
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(0..duration_roof as u32, 0f32..1f32)?;

    //let run_by_duration: Vec<usize, Duration> = duration_annotations.iter().menumerate().collect();
    //let areas = root.split_by_breakpoints([944], [80]);

    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    Ok(())
}
