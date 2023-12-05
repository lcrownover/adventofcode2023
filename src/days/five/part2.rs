#[derive(Debug)]
struct PlantMap {
    translations: Vec<Translation>,
}

#[derive(Debug)]
struct Translation {
    range: u64,
    source: u64,
    dest: u64,
}

impl PlantMap {
    fn new() -> Self {
        PlantMap {
            translations: vec![],
        }
    }

    fn from_lines(&mut self, lines: &Vec<String>) {
        for line in lines {
            self.add_translation_from_line(line)
        }
    }

    fn add_translation_from_line(&mut self, line: &String) {
        let elems: Vec<u64> = line
            .split_whitespace()
            .map(|v| v.trim().parse::<u64>().unwrap())
            .collect();
        let maprange = *elems.iter().nth(2).unwrap();
        let source_start = *elems.iter().nth(1).unwrap();
        let dest_start = *elems.iter().nth(0).unwrap();
        let t = Translation {
            range: maprange,
            source: source_start,
            dest: dest_start,
        };
        self.translations.push(t)
    }

    fn translate(&self, source: &u64) -> u64 {
        // println!("translating {}", source);
        for t in &self.translations {
            if (t.source..t.source + t.range).contains(source) {
                let i = source - t.source;
                // println!("found       {}", t.dest + i);
                return t.dest + i;
            }
        }
        *source
    }
}

#[derive(Debug)]
enum MapType {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
    Unknown,
}

fn parse_seeds(line: &String) -> Vec<u64> {
    let mut seeds: Vec<u64> = vec![];
    let seed_values: Vec<u64> = line
        .split(":")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|v| v.trim().parse::<u64>().unwrap())
        .collect();
    let mut seed_start: u64 = 0;
    for (i, sv) in seed_values.iter().enumerate() {
        if i % 2 == 0 {
            // even numbers are the start values
            seed_start = *sv;
            continue;
        }
        // odd numbers are the range
        for j in seed_start..seed_start + sv {
            seeds.push(j)
        }
    }
    seeds
}

pub fn run(lines: &Vec<String>) {
    let mut lines = lines.clone();
    lines.push("".to_string());

    let mut seeds: Vec<u64> = vec![];
    let mut seed_to_soil = PlantMap::new();
    let mut soil_to_fertilizer = PlantMap::new();
    let mut fertilizer_to_water = PlantMap::new();
    let mut water_to_light = PlantMap::new();
    let mut light_to_temperature = PlantMap::new();
    let mut temperature_to_humidity = PlantMap::new();
    let mut humidity_to_location = PlantMap::new();

    let mut maptype: MapType = MapType::Unknown;
    let mut reading: bool = false;
    let mut cache: Vec<String> = vec![];

    for line in lines {
        // println!("processing line: {line:?}");
        if line.starts_with("seeds") {
            // println!("found seeds line");
            seeds = parse_seeds(&line);
        }
        if line.starts_with("seed-to-soil") {
            // println!("seed-to-soil");
            maptype = MapType::SeedToSoil;
            reading = true;
            continue;
        }
        if line.starts_with("soil-to-fertilizer") {
            // println!("soil-to-fertilizer");
            maptype = MapType::SoilToFertilizer;
            reading = true;
            continue;
        }
        if line.starts_with("fertilizer-to-water") {
            // println!("fertilizer-to-water");
            maptype = MapType::FertilizerToWater;
            reading = true;
            continue;
        }
        if line.starts_with("water-to-light") {
            // println!("water-to-light");
            maptype = MapType::WaterToLight;
            reading = true;
            continue;
        }
        if line.starts_with("light-to-temperature") {
            // println!("light-to-temperature");
            maptype = MapType::LightToTemperature;
            reading = true;
            continue;
        }
        if line.starts_with("temperature-to-humidity") {
            // println!("temperature-to-humidity");
            maptype = MapType::TemperatureToHumidity;
            reading = true;
            continue;
        }
        if line.starts_with("humidity-to-location") {
            // println!("humidity-to-location");
            maptype = MapType::HumidityToLocation;
            reading = true;
            continue;
        }
        // This is where we dump the data and assign the map
        if line.trim().is_empty() {
            reading = false;
            if cache.is_empty() {
                continue;
            }
            // println!("found empty line, processing cache:");
            // println!("{cache:?}");
            match maptype {
                MapType::SeedToSoil => seed_to_soil.from_lines(&cache),
                MapType::SoilToFertilizer => soil_to_fertilizer.from_lines(&cache),
                MapType::FertilizerToWater => fertilizer_to_water.from_lines(&cache),
                MapType::WaterToLight => water_to_light.from_lines(&cache),
                MapType::LightToTemperature => light_to_temperature.from_lines(&cache),
                MapType::TemperatureToHumidity => temperature_to_humidity.from_lines(&cache),
                MapType::HumidityToLocation => humidity_to_location.from_lines(&cache),
                MapType::Unknown => panic!("maptype logic broken"),
            }
            cache = vec![];
        }
        if reading {
            // println!("found number line, adding to cache");
            cache.push(line.clone())
        }
    }

    // println!("{seeds:?}");
    // println!("maps:");
    // println!("seed -> soil {:?}", seed_to_soil);
    // println!("soil -> fertilizer {:?}", soil_to_fertilizer);
    // println!("fertilizer -> water {:?}", fertilizer_to_water);
    // println!("water -> light {:?}", water_to_light);
    // println!("light -> temperature {:?}", light_to_temperature);
    // println!("temperature -> humidity {:?}", temperature_to_humidity);
    // println!("humidity -> location {:?}", humidity_to_location);

    let mut lowest_location: Option<u64> = None;

    for seed in seeds {
        let soil = seed_to_soil.translate(&seed);
        let fertilizer = soil_to_fertilizer.translate(&soil);
        let water = fertilizer_to_water.translate(&fertilizer);
        let light = water_to_light.translate(&water);
        let temperature = light_to_temperature.translate(&light);
        let humidity = temperature_to_humidity.translate(&temperature);
        let location = humidity_to_location.translate(&humidity);

        if lowest_location.is_none() || location < lowest_location.unwrap() {
            lowest_location = Some(location)
        }
    }

    println!("{}", lowest_location.unwrap())
}
