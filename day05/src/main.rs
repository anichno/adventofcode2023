use std::ops::Range;

#[derive(Debug)]
struct Mappings {
    seed_to_soil: Vec<Mapping>,
    soil_to_fertilizer: Vec<Mapping>,
    fertilizer_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temperature: Vec<Mapping>,
    temperature_to_humidity: Vec<Mapping>,
    humidity_to_location: Vec<Mapping>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Mapping {
    input_range: Range<u64>,
    output_range: Range<u64>,
}

fn parse(input: &[&str]) -> (Vec<u64>, Mappings) {
    fn parse_mapping<'a, 'b>(lines: &'a mut impl Iterator<Item = &'b &'b str>) -> Vec<Mapping> {
        let mut mappings = Vec::new();

        // read header line
        lines.next();

        let mut cur_line = lines.next().unwrap();
        while !cur_line.is_empty() {
            let (destination_range_start, remaining) = cur_line.split_once(' ').unwrap();
            let destination_range_start = destination_range_start.trim().parse().unwrap();
            let (source_range_start, remaining) = remaining.trim().split_once(' ').unwrap();
            let source_range_start = source_range_start.trim().parse().unwrap();
            let length: u64 = remaining.trim().parse().unwrap();

            let destination_range = destination_range_start..destination_range_start + length;
            let source_range = source_range_start..source_range_start + length;
            mappings.push(Mapping {
                input_range: source_range,
                output_range: destination_range,
            });

            if let Some(next_line) = lines.next() {
                cur_line = next_line;
            } else {
                // end of input
                break;
            }
        }

        mappings
    }
    let mut lines = input.iter();
    let seed_line = lines.next().unwrap();
    let seeds: Vec<u64> = seed_line
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|n| n.trim().parse().unwrap())
        .collect();

    // blank line
    lines.next();

    let mut seed_to_soil = parse_mapping(&mut lines);
    seed_to_soil.sort_unstable_by_key(|v| v.input_range.start);

    let mut soil_to_fertilizer = parse_mapping(&mut lines);
    soil_to_fertilizer.sort_unstable_by_key(|v| v.input_range.start);

    let mut fertilizer_to_water = parse_mapping(&mut lines);
    fertilizer_to_water.sort_unstable_by_key(|v| v.input_range.start);

    let mut water_to_light = parse_mapping(&mut lines);
    water_to_light.sort_unstable_by_key(|v| v.input_range.start);

    let mut light_to_temperature = parse_mapping(&mut lines);
    light_to_temperature.sort_unstable_by_key(|v| v.input_range.start);

    let mut temperature_to_humidity = parse_mapping(&mut lines);
    temperature_to_humidity.sort_unstable_by_key(|v| v.input_range.start);

    let mut humidity_to_location = parse_mapping(&mut lines);
    humidity_to_location.sort_unstable_by_key(|v| v.input_range.start);

    (
        seeds,
        Mappings {
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        },
    )
}

fn convert(id: u64, mappings: &[Mapping]) -> u64 {
    for mapping in mappings {
        if mapping.input_range.contains(&id) {
            let offset = id - mapping.input_range.start;
            return mapping.output_range.start + offset;
        }
    }

    id
}

fn solve1(input: &[&str]) -> u64 {
    let (seeds, mappings) = parse(input);
    let mut min_location = u64::MAX;
    for seed in seeds {
        let soil = convert(seed, &mappings.seed_to_soil);
        let fertilizer = convert(soil, &mappings.soil_to_fertilizer);
        let water = convert(fertilizer, &mappings.fertilizer_to_water);
        let light = convert(water, &mappings.water_to_light);
        let temperature = convert(light, &mappings.light_to_temperature);
        let humidity = convert(temperature, &mappings.temperature_to_humidity);
        let location = convert(humidity, &mappings.humidity_to_location);

        min_location = min_location.min(location);
    }

    min_location
}

// mappings are assumed to be sorted
fn convert_range(id_ranges: &[Range<u64>], mappings: &[Mapping]) -> Vec<Range<u64>> {
    let mut out_ranges = Vec::new();

    for mut in_range in id_ranges.iter().cloned() {
        for mapping in mappings {
            if in_range.start < mapping.input_range.start {
                out_ranges.push(in_range.start..mapping.input_range.start.min(in_range.end));
                in_range.start = mapping.input_range.start.min(in_range.end);

                if in_range.start == in_range.end {
                    break;
                }
            }

            if mapping.input_range.contains(&in_range.start)
                || mapping.input_range.contains(&in_range.end)
            {
                let offset = in_range.start - mapping.input_range.start;
                let len = in_range.end.min(mapping.input_range.end) - 1 - in_range.start;
                out_ranges.push(
                    mapping.output_range.start + offset..mapping.output_range.start + offset + len,
                );

                in_range.start = in_range.end.min(mapping.input_range.end);

                if in_range.start == in_range.end {
                    break;
                }
            }
        }

        // any leftover in in_range is just passthrough
        if in_range.start != in_range.end {
            out_ranges.push(in_range);
        }
    }

    out_ranges
}

fn solve2(input: &[&str]) -> u64 {
    let (seeds, mappings) = parse(input);
    let seed_ranges: Vec<Range<u64>> = seeds.chunks(2).map(|c| c[0]..c[0] + c[1]).collect();
    let mut min_location = u64::MAX;
    for seed_range in seed_ranges {
        let soil = convert_range(&[seed_range], &mappings.seed_to_soil);
        let fertilizer = convert_range(&soil, &mappings.soil_to_fertilizer);
        let water = convert_range(&fertilizer, &mappings.fertilizer_to_water);
        let light = convert_range(&water, &mappings.water_to_light);
        let temperature = convert_range(&light, &mappings.light_to_temperature);
        let humidity = convert_range(&temperature, &mappings.temperature_to_humidity);
        let location = convert_range(&humidity, &mappings.humidity_to_location);

        for loc in location {
            min_location = min_location.min(loc.start);
        }
    }

    min_location
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    println!("part 1: {}", solve1(&input));
    println!("part 2: {}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &[
        "seeds: 79 14 55 13",
        "",
        "seed-to-soil map:",
        "50 98 2",
        "52 50 48",
        "",
        "soil-to-fertilizer map:",
        "0 15 37",
        "37 52 2",
        "39 0 15",
        "",
        "fertilizer-to-water map:",
        "49 53 8",
        "0 11 42",
        "42 0 7",
        "57 7 4",
        "",
        "water-to-light map:",
        "88 18 7",
        "18 25 70",
        "",
        "light-to-temperature map:",
        "45 77 23",
        "81 45 19",
        "68 64 13",
        "",
        "temperature-to-humidity map:",
        "0 69 1",
        "1 0 69",
        "",
        "humidity-to-location map:",
        "60 56 37",
        "56 93 4",
    ];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT), 35)
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT), 46)
    }
}
