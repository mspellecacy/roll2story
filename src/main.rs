use rand::Rng;
use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::path::Path;

/// Config Example
///  (
//     facets: [
//         (
//             name: "My Facet",
//             enabled: true,
//             items: [
//                 "item1",
//                 "item2",
//                 "item3",
//             ],
//         ),
//      ]
//  )

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Config {
    facets: Vec<StoryFacet>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct StoryFacet {
    name: String,
    enabled: bool,
    items: Vec<String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config_path = Path::new("facets.ron");
    let facets_file = File::open(config_path).expect("Missing facets.ron");
    let config: Config = match from_reader(facets_file) {
        Ok(c) => c,
        Err(e) => panic!("Failed to load config: {}", e),
    };

    if args.contains(&"debug".to_string()) {
        println!(
            "Config: \n {}",
            ron::ser::to_string_pretty(&config, ron::ser::PrettyConfig::default()).unwrap(),
        );
    }

    let mut rng = rand::thread_rng();
    let chosen_facets: Vec<String> = config
        .facets
        .iter()
        .filter(|f| f.enabled)
        .map(|f| {
            let random_item_index: usize = rng.gen_range(0..f.items.len());
            let item = f.items[random_item_index].clone();
            format!("{} \n {}", f.name, item)
        })
        .collect();

    chosen_facets.iter().map(|f| println!("{f}")).collect()
}
