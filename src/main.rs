use analysis_utils::duration_annotation::get_duration_annotations;
use analysis_utils::duration_annotation::Annotations;
use analysis_utils::duration_annotation::DurationAnnotation;
use clap::Parser;
use plotters::{
    prelude::Circle,
    style::{self, full_palette::WHITE, Color},
};
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    file: std::path::PathBuf,
}
macro_rules! graph_durations {
    ($chart:tt, $annotations:tt, $color:tt, $position:ident) => {
        $position += 1;
        $chart
            .draw_series($annotations.0.iter().enumerate().map(
                |(_, DurationAnnotation { duration, .. })| {
                    plotters::prelude::Circle::new(
                        ($position, duration.as_millis() as u64),
                        2,
                        plotters::style::$color.filled(),
                    )
                },
            ))?
            .label(&$annotations.get_testname())
            .legend(|(x, y)| Circle::new((x + 15, y), 2, plotters::style::$color.filled()));
        //let mean = $annotations.get_da_mean();
    };
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
    let duration_annotations = Annotations(get_duration_annotations(cli.file));

    // viewonly_client_pu_false section:
    let keyless_client_pu_false =
        duration_annotations.filter_on_testname("keyless_client_pu_false");
    let keyowning_client_pu_false =
        duration_annotations.filter_on_testname("keyowning_client_pu_false");
    let fullviewonly_client_pu_false =
        duration_annotations.filter_on_testname("fullviewonly_client_pu_false");
    let keyless_duration_roof = keyless_client_pu_false.get_da_roof();
    let keyowning_duration_roof = keyowning_client_pu_false.get_da_roof();
    let full_duration_roof = fullviewonly_client_pu_false.get_da_roof();
    let first_duration_roof = keyless_duration_roof.max(full_duration_roof);
    let duration_roof = keyowning_duration_roof.max(first_duration_roof);
    // Begin plotting expressions
    use plotters::{backend, drawing};
    let root = drawing::IntoDrawingArea::into_drawing_area(backend::BitMapBackend::new(
        &image_str,
        (1024, 768),
    ));

    root.fill(&style::colors::WHITE)?;

    let git_description = &keyowning_client_pu_false.0[0].git_description;
    let caption = format!("1153 block sync times, version: {}", &git_description);
    let mut chart = plotters::chart::ChartBuilder::on(&root)
        .x_label_area_size(30)
        .y_label_area_size(60)
        .caption(caption.to_string(), ("Calibri", 30.0))
        .build_cartesian_2d(0u64..4, 0u64..duration_roof)?;

    chart
        .configure_mesh()
        .bold_line_style(WHITE.mix(0.3))
        .y_desc("Milliseconds To Sync")
        .x_desc("Benchmark Scenarios")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;
    let mut position = 0;
    graph_durations!(chart, keyless_client_pu_false, BLUE, position);
    graph_durations!(chart, fullviewonly_client_pu_false, RED, position);
    graph_durations!(chart, keyowning_client_pu_false, GREEN, position);
    chart
        .configure_series_labels()
        .label_font(("Calibri", 20))
        .background_style(plotters::style::WHITE.mix(0.8))
        .border_style(plotters::style::BLACK)
        .draw()?;

    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    Ok(())
}
