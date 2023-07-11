use clap::Parser;
use plotters::{
    prelude::ChartBuilder,
    style::{self, Color},
};
use zingo_testutils::DurationAnnotation;
struct Annotations(Vec<DurationAnnotation>);
impl Annotations {
    fn filter_on_testname(&self, name: String) -> Annotations {
        let matches = self
            .0
            .clone()
            .into_iter()
            .filter(|da| da.test_name.contains(&name[..]))
            .collect();
        Annotations(matches)
    }
}

fn get_test_name(duration_annotations: &Vec<DurationAnnotation>) -> String {
    let DurationAnnotation { test_name, .. } = &duration_annotations[0];
    test_name.clone()
}
fn get_da_roof(duration_annotations: &Vec<DurationAnnotation>) -> u128 {
    let durations = duration_annotations
        .iter()
        .map(|da| da.duration.as_millis())
        .collect::<Vec<u128>>();
    let duration_max = durations.iter().fold(0, |acc, d| acc.max(*d));
    (duration_max >> 3) + duration_max
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
    dbg!(&image_str);
    //let image_location = std::path::PathBuf::from(image_str);
    println!("Hello, world! {}", cli.file.to_str().unwrap());
    // load annotations
    let duration_annotations = zingo_testutils::get_duration_annotations(cli.file);
    let duration_roof = get_da_roof(&duration_annotations);
    let das = duration_annotations.len() as u128;
    dbg!(&duration_annotations);
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
        .build_cartesian_2d(0u128..das + 1, 0u128..duration_roof)?;

    scatter_ctx.draw_series(duration_annotations.iter().enumerate().map(
        |(x, DurationAnnotation { duration, .. })| {
            plotters::prelude::Circle::new(
                ((x as u128 + 1), duration.as_millis()),
                2,
                plotters::style::BLUE.filled(),
            )
        },
    ))?;
    ChartBuilder::on(&root)
        .caption(get_test_name(&duration_annotations), ("sans-serif", 30))
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(0..duration_roof as u32, 0f32..1f32)?;

    //let run_by_duration: Vec<usize, Duration> = duration_annotations.iter().menumerate().collect();
    //let areas = root.split_by_breakpoints([944], [80]);

    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    Ok(())
}
