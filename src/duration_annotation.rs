use serde::{Deserialize, Serialize};
use std::{fs::OpenOptions, path::PathBuf, time::Duration};
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DurationAnnotation {
    pub timestamp: u64,
    pub git_description: String,
    pub test_name: String,
    pub duration: Duration,
    pub scenario: String,
}
impl DurationAnnotation {
    pub fn new(scenario: String, test_name: String, duration: Duration) -> Self {
        DurationAnnotation {
            timestamp: timestamp(),
            git_description: zingolib::git_description()
                .to_string()
                .trim_end()
                .to_string(),
            scenario,
            test_name,
            duration,
        }
    }
}
impl std::fmt::Display for DurationAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#""test_name": {}, "timestamp": {}, "git_description": {}, "duration": {}"#,
            self.test_name,
            self.timestamp,
            self.git_description,
            self.duration.as_millis() as u64
        )
    }
}
fn timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
fn path_to_times(file_name: PathBuf) -> PathBuf {
    let timing_dir = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR").expect("To be inside a manifested space."),
    )
    .join("tests/times");
    timing_dir.join(file_name)
}
pub fn read_duration_annotation_file(target: PathBuf) -> Vec<DurationAnnotation> {
    let data_set: Vec<DurationAnnotation> = if let Ok(data) = std::fs::read_to_string(target) {
        serde_json::from_str(&data[..]).expect("To deserialize a string")
    } else {
        vec![]
    };
    data_set
}
pub fn get_duration_annotations(storage_file: PathBuf) -> Vec<DurationAnnotation> {
    read_duration_annotation_file(storage_file)
}
pub fn record_time(annotation: &DurationAnnotation) {
    let version_info = zingolib::git_description()
        .to_string()
        .trim_end()
        .to_string();
    let storage_path = format!("{}_sync_duration_annotation.json", version_info).to_string();
    let storage_location = path_to_times(PathBuf::from(storage_path));
    let mut data_set = get_duration_annotations(storage_location.clone());
    data_set.push(annotation.clone());

    //let json_dataset = array!(data_set);
    let mut time_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(storage_location)
        .expect("to access a data_store file");
    std::io::Write::write_all(
        &mut time_file,
        serde_json::to_string(&data_set)
            .expect("to serialiaze")
            .as_bytes(),
    )
    .expect("To write out a new data vector");
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deserialize_json_into_duration_annotation() {
        let test_name = String::from("test_test_name");
        let ta = DurationAnnotation::new(
            "scenario".to_string(),
            test_name,
            Duration::from_millis(1_000),
        );
        let ta2 = ta.clone();
        let ta_serde_json = serde_json::to_value(ta).unwrap();
        let ta: DurationAnnotation = serde_json::from_value(ta_serde_json).unwrap();
        assert_eq!(ta, ta2);
    }
}
#[derive(Debug)]
pub struct Annotations(pub Vec<DurationAnnotation>);
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
    pub fn filter_on_testname(&self, name: &str) -> Annotations {
        let matches = self
            .0
            .clone()
            .into_iter()
            .filter(|da| da.test_name == name)
            .collect();
        Annotations(matches)
    }
    #[allow(dead_code)]
    fn filter_on_scenario(&self, name: &str) -> Annotations {
        let matches = self
            .0
            .clone()
            .into_iter()
            .filter(|da| da.scenario.contains(name))
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
    pub fn get_da_roof(&self) -> u64 {
        let durations = self
            .0
            .iter()
            .map(|da| da.duration.as_millis() as u64)
            .collect::<Vec<u64>>();
        let duration_max = durations.iter().fold(0, |acc, d| acc.max(*d));
        (duration_max >> 3) + duration_max
    }
    pub fn get_testname(&self) -> String {
        self.0[0]
            .test_name
            .clone()
            .trim_start_matches("sync_1153_baseline_synctimes_")
            .trim_end_matches("_client_pu_false")
            .to_string()
    }
}
