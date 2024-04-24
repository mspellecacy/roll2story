use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::{Display, Formatter};
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

#[derive(Debug, Clone, Deserialize, Serialize)]
struct StoryFacetResult {
    facet_name: String,
    facet_item: String,
}

impl Display for StoryFacetResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.facet_name, self.facet_item)
    }
}

fn get_story_facet(facet: &StoryFacet, rng: &mut ThreadRng) -> StoryFacetResult {
    let random_item_index: usize = rng.gen_range(0..facet.items.len());
    let facet_name = facet.name.clone();
    let facet_item = facet.items[random_item_index].clone();

    StoryFacetResult {
        facet_name,
        facet_item,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config_path = Path::new("facets.ron");
    let facets_file = File::open(config_path).expect("Missing 'facets.ron' config file");
    let config: Config = match from_reader(facets_file) {
        Ok(c) => c,
        Err(e) => panic!("Failed to load config: {e}"),
    };

    if args.contains(&"debug".to_string()) {
        println!(
            "Config:\n{}",
            ron::ser::to_string_pretty(&config, ron::ser::PrettyConfig::default()).unwrap(),
        );
    }

    fn do_facets(facets: &Vec<StoryFacet>) {
        facets
            .iter()
            .filter(|f| f.enabled)
            .map(|f| get_story_facet(&f, &mut thread_rng()))
            .map(|f| println!("{f}"))
            .collect()
    }

    let mut _continue_ = "Y".to_string();
    while _continue_.trim() != "n" {
        _continue_.clear();

        println! {"roll4story\n-----"};
        do_facets(&config.facets);
        println! {"-----"};

        println!("More? Y/n");
        std::io::stdin().read_line(&mut _continue_).unwrap();
    }
}
