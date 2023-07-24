use clap::Parser;
use plotters::{
    prelude::Circle,
    style::{self, full_palette::WHITE, Color},
};
use zingo_testutils::DurationAnnotation;
#[derive(Debug)]
struct Annotations(Vec<DurationAnnotation>);
struct ToDisplay(String);
impl std::fmt::Display for ToDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::write!(f, "{}", self.0)
    }
}
impl Annotations {
    #[allow(unused)]
    fn truncate(&self) -> Annotations {
        let trunced = self.0[..self.0.len() - 1].to_vec();
        Annotations(trunced)
    }
    fn filter_on_testname(&self, name: &str) -> Annotations {
        let matches = self
            .0
            .clone()
            .into_iter()
            .filter(|da| da.test_name == name)
            .collect();
        Annotations(matches)
    }
    #[allow(unused)]
    fn filter_on_git_description(&self, git_description: &str) -> Annotations {
        let matches = self
            .0
            .clone()
            .into_iter()
            .filter(|da| da.git_description == git_description)
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
    fn get_testname(&self) -> String {
        self.0[0]
            .test_name
            .clone()
            .trim_start_matches("sync_1153_baseline_synctimes_")
            .trim_end_matches("_client_pu_false")
            .to_string()
    }
}

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
                        ($position, duration.as_millis()),
                        2,
                        plotters::style::$color.filled(),
                    )
                },
            ))?
            .label($annotations.get_testname())
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
    let duration_annotations = Annotations(zingo_testutils::get_duration_annotations(cli.file));

    // viewonly_client_pu_false section:
    let keyless_client_pu_false =
        duration_annotations.filter_on_testname("keyless_client_pu_false");
    let keyowning_client_pu_false =
        duration_annotations.filter_on_testname("keyowning_client_pu_false");
    let fullviewonly_client_pu_false =
        duration_annotations.filter_on_testname("fullviewonly_client_pu_false");
    if dbg!(fullviewonly_client_pu_false.0.len()) == 0
        || dbg!(keyless_client_pu_false.0.len()) == 0
        || dbg!(keyowning_client_pu_false.0.len()) == 0
    {
        panic!("Empty list!")
    }
    dbg!(&fullviewonly_client_pu_false);
    let keyless_duration_roof = keyless_client_pu_false.get_da_roof();
    //let keyowning_duration_roof = keyowning_client_pu_false.get_da_roof();
    let full_duration_roof = fullviewonly_client_pu_false.get_da_roof();
    //let first_duration_roof = keyless_duration_roof.max(full_duration_roof);
    let duration_roof = keyless_duration_roof.max(full_duration_roof);
    //let das = viewonly_client_pu_false_das.0.len() as u128;
    // Begin plotting expressions
    use plotters::{backend, drawing};
    let root = drawing::IntoDrawingArea::into_drawing_area(backend::BitMapBackend::new(
        &image_str,
        (1024, 768),
    ));

    root.fill(&style::colors::WHITE)?;

    let mut chart = plotters::chart::ChartBuilder::on(&root)
        .x_label_area_size(30)
        .y_label_area_size(60)
        .caption("1153 Block Chain, Sync Times", ("Calibri", 30.0))
        .build_cartesian_2d(0u128..3, 0u128..duration_roof)?;

    chart
        .configure_mesh()
        .bold_line_style(WHITE.mix(0.3))
        .y_desc("MilliSeconds To Sync")
        .x_desc("Benchmark Scenarios")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;
    let mut position = 0;
    graph_durations!(chart, keyless_client_pu_false, BLUE, position);
    graph_durations!(chart, fullviewonly_client_pu_false, RED, position);
    //graph_durations!(chart, keyowning_client_pu_false, GREEN, position);
    chart
        .configure_series_labels()
        .label_font(("Calibri", 20))
        .background_style(plotters::style::WHITE.mix(0.8))
        .border_style(plotters::style::BLACK)
        .draw()?;
    /*
        ChartBuilder::on(&root)
            .caption(
                "keyless_client, fvk_only_client, keyowning_client",
                ("sans-serif", 30),
            )
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(0..duration_roof as u32, 0f32..1f32)?;
    */
    //let run_by_duration: Vec<usize, Duration> = duration_annotations.iter().menumerate().collect();
    //let areas = root.split_by_breakpoints([944], [80]);

    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    Ok(())
}
