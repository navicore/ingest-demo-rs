use crate::model::Record;
use rand::thread_rng;
use std::error::Error;
use std::io::stdout;
use std::path::Path;

pub fn generate_records(count: usize, example_path: &Path) -> Result<(), Box<dyn Error>> {
    let example = Record::load_example(example_path.to_str().unwrap())?;
    let mut rng = thread_rng();

    for _ in 0..count {
        let record = Record::generate_random(&example, &mut rng);
        let json = serde_json::to_string(&record)?;
        println!("{}", json);
    }

    Ok(())
}
