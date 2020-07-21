use chrono::Timelike;
use aoc::utils::*;
use regex::Regex;
use chrono::NaiveDateTime;
use std::collections::HashMap;
use ndarray::Array1;
use ndarray_stats::QuantileExt;
#[macro_use(s)]
extern crate ndarray;

fn main() {
    let input_data: Vec<_> = read_inputs("inputs/day-4.txt");
    let all_start_times = parse_string_vec::<String>(&input_data, r"\[(\d+-\d+-\d+ \d+:\d+)] (.*)");
    println!("All start times #7 = {:?}", &all_start_times[6]);
    #[derive(Debug)]
    struct TimedEvent {
        event_time: NaiveDateTime,
        description: String
    };
    let mut all_timed_events: Vec<TimedEvent> = Vec::new();
    for timed_event in all_start_times {
        all_timed_events.push(TimedEvent{event_time: NaiveDateTime::parse_from_str(&timed_event[1], "%Y-%m-%d %H:%M").unwrap(), description: timed_event[2].clone()});
    }
    all_timed_events.sort_by(|a, b| a.event_time.partial_cmp(&b.event_time).unwrap());
    println!("All timed events #1 = {:?}", &all_timed_events[0]);
    println!("All timed events #2 = {:?}", &all_timed_events[1]);
    println!("All timed events #3 = {:?}", &all_timed_events[2]);
    println!("All timed events #4 = {:?}", &all_timed_events[3]);
    let re_guard = Regex::new(r"Guard #(\d+) begins shift").unwrap();
    let re_asleep = Regex::new(r"falls asleep").unwrap();
    let re_awake = Regex::new(r"wakes up").unwrap();
    let mut guards: HashMap<i32, Array1<i32>> = HashMap::new();
    let mut curr_asleep: Option<usize> = None;
    let mut curr_guard: Option<i32> = None;
    for timed_event in all_timed_events {
        let guard_id = parse_string::<i32>(&timed_event.description, &re_guard);
        if guard_id.len() > 0 {
            if !guards.contains_key(&guard_id[0]) {
                guards.insert(guard_id[0], Array1::zeros(60));
            }
            curr_asleep = None;
            curr_guard = Some(guard_id[0].clone());
            continue;
        }
        let asleep = parse_string::<String>(&timed_event.description, &re_asleep);
        if asleep.len() > 0 {
            curr_asleep = Some(timed_event.event_time.minute() as usize);
            continue;
        }
        let awake = parse_string::<String>(&timed_event.description, &re_awake);
        if awake.len() > 0 {
            match curr_asleep {
                Some(x) => {
                    match curr_guard {
                        Some(g) => {
                            let awake = (timed_event.event_time.minute()) as usize;
                            let mut slice = guards.get_mut(&g).unwrap().slice_mut(s![x..awake]);
                            slice += 1;
                        },
                        None => println!("Warning got awake at {} without guard id", &timed_event.event_time)
                    }
                },
                None => println!("Warning got awake at {} without asleep", &timed_event.event_time)
            }
        }
    }
    let mut max_sum = 0;
    let mut result_hash = 0;
    let mut result_hash_2 = 0;
    let mut max_g_id = 0;
    let mut max_g_id_2 = 0;
    let mut max_asleep = 0;
    for (g_id, asleep) in guards {
        if asleep.sum() > max_sum {
            max_sum = asleep.sum();
            max_g_id = g_id;
            result_hash = g_id * asleep.argmax().unwrap() as i32;
        }
        if asleep.max().unwrap() > &max_asleep {
            max_asleep = *asleep.max().unwrap();
            max_g_id_2 = g_id;
            result_hash_2 = g_id * asleep.argmax().unwrap() as i32;
        }
    }
    println!("You're looking for hash value = {} corresponding to guard #{}", result_hash, max_g_id);
    println!("You're looking for hash value (part 2) = {} corresponding to guard #{}", result_hash_2, max_g_id_2);
}
