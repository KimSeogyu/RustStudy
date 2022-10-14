use std::collections::HashMap;
use std::iter::Zip;
use std::thread;
use std::time::Duration;

struct CachedData<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    valueMap: HashMap<u32, u32>,
}

impl<T> CachedData<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> CachedData<T> {
        CachedData {
            calculation,
            valueMap: HashMap::new(),
        }
    }

    fn value(&mut self, key: u32) -> u32 {
        match self.valueMap.get(&key) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(key);
                self.valueMap.insert(key, v);
                v
            }
        }
    }
}

fn main() {
    // let v: Vec<_> = Counter::new()
    //     .zip(Counter::new().skip(1))
    //     .map(|(a, b)| a * b)
    //     .filter(|x| x % 3 == 0)
    //     .collect();
    // println!("using_other_iterator_trait_methods {:#?}", v);
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = CachedData::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!(
            "Today, run for {} minutes!",
            expensive_result.value(intensity)
        );
    }
}

#[test]
fn call_with_different_values() {
    let mut c = CachedData::new(|a| a);
    let v1 = c.value(1);
    let v2 = c.value(2);
    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
}

#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn using_other_iterator_trait_methods() {
    // let sum: u32 = Counter::new().zip(Counter::new());

    eprintln!(
        "using_other_iterator_trait_methods {:#?}",
        Counter::new().zip(Counter::new())
    );
}

#[test]
fn what_is_this() {
    let buffer: &mut [i32];
    let coefficient: [i64; 12];
    let qlp_shift: i16;

    println!("{:#?}", 12 as u32);
}
