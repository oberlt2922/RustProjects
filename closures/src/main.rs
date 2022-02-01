use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| { //instantiates Cacher with calculation set to the passed in closure
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        return num;
    });

    if intensity < 25 {
        println!("Today, do {} pushups", expensive_result.value(intensity));
        println!("Next, do {} situps", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today!  Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes", expensive_result.value(intensity));
        }
    }
}


struct Cacher<T, K, V>
where T: Fn(K) -> V, 
      K: std::cmp::Eq + std::hash::Hash + Clone,
      V: Clone
{
    calculation: T,//the closure that is executed
    pub value: HashMap<K, V>//a list of values so that the calculation only has to run once for each passed in argument
}

impl <T, K, V> Cacher<T, K, V>
where T: Fn(K) -> V,
      K: std::cmp::Eq + std::hash::Hash + Clone,
      V: Clone
{
    fn new(calculation: T) -> Cacher<T, K, V> {//constructor sets calculation to the passed in closure and sets value to a new hashmap
        Cacher{
            calculation,
            value: HashMap::new()
        }
    }

    fn value(& mut self, arg: K) -> V {
        match self.value.contains_key(&arg) {
            true => self.value.get(&arg).unwrap().clone(),
            false => {
                let v = (self.calculation)(arg.clone());
                self.value.insert(arg, v.clone());
                return v;
            }
        }
    }
}

#[test]
fn multiple_calls_with_different_values() {
    let mut c = Cacher::new(|a| a);
    let _v1 = c.value(1);
    let v2 = c.value(2);
    assert_eq!(v2, 2);
}

#[test]
fn call_with_string_key_and_num_val() {
    let mut c = Cacher::new(|_a| 2);
    let v1 = c.value("String");
    assert_eq!(c.value.get_key_value("String") != None, v1 == 2);
}

#[test]
fn call_with_num_key_and_string_val() {
    let mut c = Cacher::new(|a: u32| {
        return format!("String {}", a.to_string().as_str());
    });
    let v1 = c.value(1);
    assert_eq!(c.value.get_key_value(&1) != None, v1 == String::from("String 1"));
}