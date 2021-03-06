extern crate pixie_rust;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use pixie_rust::recommender::Recommender;

extern crate csv;

fn main() {
    let mut recommender: Recommender<String> = Recommender::new();

    println!("Loading Data...");
    // Anime dataset from https://www.kaggle.com/CooperUnion/anime-recommendations-database
    let file = File::open("examples/anime.csv").unwrap();
    let buf_reader = BufReader::new(file);
    let mut csv_reader = csv::Reader::from_reader(buf_reader);

    let mut ratings: HashMap<String, f32> = HashMap::new();

    for entry_res in csv_reader.records() {
        let entry = entry_res.unwrap();
        let name = entry.get(1).unwrap();
        let categories_str = entry.get(2).unwrap();
        let rating = entry.get(5).unwrap().parse::<f32>().unwrap_or(0.0);
        ratings.insert(String::from(name), rating);
        recommender.add_object(&String::from(name));
        let categories = categories_str.split(",");
        for cat in categories {
            let trimmed = cat.trim();
            recommender.add_tag(trimmed);
            recommender.tag_object(&String::from(name), trimmed);
        }
    }
    println!("Data Loaded!");

    let top_recommendations = recommender
        .object_recommendations(
            &vec![
                String::from("Cowboy Bebop"),
                String::from("Serial Experiments Lain"),
                String::from("Ghost in the Shell"),
            ],
            15,
            5000,
            |_, _| 1.0,
            |_, to| match to {
                name => ratings.get(name).unwrap_or(&0.0).clone(),
            },
        )
        .iter()
        .take(10)
        .cloned()
        .collect::<Vec<String>>();

    println!("Recommendations: {:?}", top_recommendations);
}
