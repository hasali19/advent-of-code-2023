use std::collections::{HashMap, VecDeque};

use aoc2023::aoc_solution;
use itertools::Itertools;

fn main() -> eyre::Result<()> {
    aoc_solution(20, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ModuleType {
    FlipFlop(bool),
    Conjunction,
    Broadcaster,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

fn part_1(input: &str) -> eyre::Result<()> {
    let mut modules = input
        .lines()
        .map(|line| {
            let (input, outputs) = line.split_once(" -> ").unwrap();
            let outputs = outputs.split(", ").collect_vec();

            let input_type = match &input[..1] {
                "%" => ModuleType::FlipFlop(false),
                "&" => ModuleType::Conjunction,
                _ if input == "broadcaster" => ModuleType::Broadcaster,
                _ => unreachable!(),
            };

            (input.trim_start_matches(['%', '&']), (input_type, outputs))
        })
        .collect::<HashMap<_, _>>();

    let mut memory = HashMap::new();

    for (name, (_, outputs)) in &modules {
        for output in outputs {
            if let Some((ModuleType::Conjunction, _)) = modules.get(*output) {
                memory
                    .entry(*output)
                    .or_insert_with(HashMap::new)
                    .insert(*name, Pulse::Low);
            }
        }
    }

    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for _ in 0..1000 {
        let mut signal_queue = VecDeque::from([("button", "broadcaster", Pulse::Low)]);

        while let Some((source, target, pulse)) = signal_queue.pop_front() {
            // println!("{source} -> {target} ({pulse:?})");
            match pulse {
                Pulse::Low => low_pulses += 1,
                Pulse::High => high_pulses += 1,
            }

            let Some((module_type, outputs)) = modules.get_mut(target) else {
                continue;
            };

            match module_type {
                ModuleType::FlipFlop(on) => {
                    if pulse == Pulse::Low {
                        *on = !*on;
                        let pulse = if *on { Pulse::High } else { Pulse::Low };
                        for output in outputs {
                            signal_queue.push_back((target, output, pulse));
                        }
                    }
                }
                ModuleType::Conjunction => {
                    let memory = memory.get_mut(target).unwrap();
                    memory.insert(source, pulse);
                    let pulse = if memory.values().all(|pulse| *pulse == Pulse::High) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    };
                    for output in outputs {
                        signal_queue.push_back((target, output, pulse));
                    }
                }
                ModuleType::Broadcaster => {
                    for output in outputs {
                        signal_queue.push_back((target, output, pulse));
                    }
                }
            }
        }
    }

    println!("{}", low_pulses * high_pulses);

    Ok(())
}

fn part_2(_input: &str) -> eyre::Result<()> {
    Ok(())
}
