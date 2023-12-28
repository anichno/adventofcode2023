use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy)]
enum PulseLevel {
    Low,
    High,
}

#[derive(Debug)]
struct AddressedPulse {
    level: PulseLevel,
    sender: String,
    destination: String,
}

#[derive(Debug)]
struct FlipFlop {
    on: bool,
    outputs: Vec<String>,
}

#[derive(Debug)]
struct Conjunction {
    inputs: HashMap<String, PulseLevel>,
    outputs: Vec<String>,
}

#[derive(Debug)]
struct Broadcast {
    outputs: Vec<String>,
}

#[derive(Debug)]
enum Module {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcast(Broadcast),
}

impl Module {
    fn process_pulse(&mut self, pulse: AddressedPulse) -> Vec<AddressedPulse> {
        let mut out_pulses = Vec::new();

        match self {
            Module::FlipFlop(flipflop) => match pulse.level {
                PulseLevel::Low => {
                    let out_pulse_level = if flipflop.on {
                        flipflop.on = false;
                        PulseLevel::Low
                    } else {
                        flipflop.on = true;
                        PulseLevel::High
                    };

                    for out in flipflop.outputs.iter() {
                        out_pulses.push(AddressedPulse {
                            level: out_pulse_level,
                            sender: pulse.destination.clone(),
                            destination: out.clone(),
                        });
                    }
                }
                PulseLevel::High => (), // do nothing
            },
            Module::Conjunction(conjunction) => {
                *conjunction.inputs.get_mut(&pulse.sender).unwrap() = pulse.level;
                let mut send_low = true;
                for val in conjunction.inputs.values() {
                    if matches!(*val, PulseLevel::Low) {
                        send_low = false;
                        break;
                    }
                }
                let out_pulse_level = if send_low {
                    PulseLevel::Low
                } else {
                    PulseLevel::High
                };
                for out in conjunction.outputs.iter() {
                    out_pulses.push(AddressedPulse {
                        level: out_pulse_level,
                        sender: pulse.destination.clone(),
                        destination: out.clone(),
                    });
                }
            }
            Module::Broadcast(broadcaster) => {
                for out in broadcaster.outputs.iter() {
                    out_pulses.push(AddressedPulse {
                        level: pulse.level,
                        sender: "broadcaster".to_owned(),
                        destination: out.clone(),
                    });
                }
            }
        }

        out_pulses
    }
}

fn parse(input: &[&str]) -> HashMap<String, Module> {
    let mut modules = HashMap::new();
    let mut conjunctions = HashSet::new();
    for line in input {
        let (left, right) = line.split_once(" -> ").unwrap();
        let outputs = right.split(", ").map(|s| s.to_owned()).collect();
        if let Some(id) = left.strip_prefix('%') {
            // flip flop
            modules.insert(
                id.to_owned(),
                Module::FlipFlop(FlipFlop { on: false, outputs }),
            );
        } else if let Some(id) = left.strip_prefix('&') {
            // conjunction
            conjunctions.insert(id);
            modules.insert(
                id.to_owned(),
                Module::Conjunction(Conjunction {
                    inputs: HashMap::new(),
                    outputs,
                }),
            );
        } else if left == "broadcaster" {
            modules.insert(left.to_owned(), Module::Broadcast(Broadcast { outputs }));
        } else {
            panic!("invalid module id: {left}");
        }
    }

    let mut conjuction_inputs: HashMap<String, HashSet<String>> = HashMap::new();
    for (module_id, module) in modules.iter() {
        match module {
            Module::FlipFlop(ff) => {
                for out in &ff.outputs {
                    if conjunctions.contains(out.as_str()) {
                        conjuction_inputs
                            .entry(out.clone())
                            .or_default()
                            .insert(module_id.clone());
                    }
                }
            }
            Module::Conjunction(c) => {
                for out in &c.outputs {
                    if conjunctions.contains(out.as_str()) {
                        conjuction_inputs
                            .entry(out.clone())
                            .or_default()
                            .insert(module_id.clone());
                    }
                }
            }
            Module::Broadcast(b) => {
                for out in &b.outputs {
                    if conjunctions.contains(out.as_str()) {
                        conjuction_inputs
                            .entry(out.clone())
                            .or_default()
                            .insert(module_id.clone());
                    }
                }
            }
        }
    }

    for (conjunction_id, inputs) in conjuction_inputs {
        let Some(Module::Conjunction(conj)) = modules.get_mut(&conjunction_id) else {
            panic!()
        };
        for input in inputs {
            conj.inputs.insert(input.clone(), PulseLevel::Low);
        }
    }

    modules
}

fn solve1(input: &[&str]) -> i64 {
    let mut modules = parse(input);
    let mut pulses = VecDeque::new();
    let mut num_high_pulses = 0;
    let mut num_low_pulses = 0;

    for _ in 0..1000 {
        pulses.push_back(AddressedPulse {
            level: PulseLevel::Low,
            sender: "button".to_owned(),
            destination: "broadcaster".to_owned(),
        });

        while let Some(pulse) = pulses.pop_front() {
            match pulse.level {
                PulseLevel::Low => num_low_pulses += 1,
                PulseLevel::High => num_high_pulses += 1,
            }
            if let Some(module) = modules.get_mut(&pulse.destination) {
                let new_pulses = module.process_pulse(pulse);
                for new_pulse in new_pulses {
                    pulses.push_back(new_pulse);
                }
            }
        }
    }

    num_high_pulses * num_low_pulses
}

fn solve2(input: &[&str]) -> i64 {
    let mut modules = parse(input);
    let mut pulses = VecDeque::new();
    let mut num_button_presses = 0;

    let mut dt_seen_inputs = HashMap::new();
    let Some(Module::Conjunction(dt)) = modules.get("dt") else {
        panic!()
    };
    for input in dt.inputs.iter() {
        dt_seen_inputs.insert(input.0.clone(), 0);
    }
    loop {
        pulses.push_back(AddressedPulse {
            level: PulseLevel::Low,
            sender: "button".to_owned(),
            destination: "broadcaster".to_owned(),
        });
        num_button_presses += 1;

        while let Some(pulse) = pulses.pop_front() {
            if &pulse.destination == "dt" && matches!(pulse.level, PulseLevel::High) {
                let dt_input = dt_seen_inputs.get_mut(&pulse.sender).unwrap();
                if *dt_input == 0 {
                    *dt_input = num_button_presses;
                    if dt_seen_inputs.values().all(|v| *v != 0) {
                        return dt_seen_inputs.values().fold(1, |a, x| utils::lcm(a, *x)) as i64;
                    }
                }
            }
            if let Some(module) = modules.get_mut(&pulse.destination) {
                let new_pulses = module.process_pulse(pulse);
                for new_pulse in new_pulses {
                    pulses.push_back(new_pulse);
                }
            }
        }
    }
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    println!("part 1: {}", solve1(&input));
    println!("part 2: {}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &[&str] = &[
        "broadcaster -> a, b, c",
        "%a -> b",
        "%b -> c",
        "%c -> inv",
        "&inv -> a",
    ];
    const INPUT2: &[&str] = &[
        "broadcaster -> a",
        "%a -> inv, con",
        "&inv -> b",
        "%b -> con",
        "&con -> output",
    ];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT1), 32000000);
        assert_eq!(solve1(INPUT2), 11687500)
    }
}
