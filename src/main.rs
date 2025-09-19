
mod skip_list;

use std::fs::File;
use std::time;
use std::time::{Duration, Instant};
use csv;
use csv::DeserializeRecordsIter;
use serde::Deserialize;
use uuid::Uuid;

//Index,Customer Id,First Name,Last Name,Company,City,Country,Phone 1,Phone 2,Email,Subscription Date,Website
#[derive(Debug, Deserialize, Clone)]
struct Customer {
    index: i32,
    customer_id: String,
    first_name: String,
    last_name: String,
    company: String,
    city: String,
    country:String,
    phone1: String,
    phone2:String,
    email: String,
    subscription_date: String,
    website: String,
}

use crate::skip_list::*;
use crate::skip_list::PromotionType::{Deterministic, Probabilistic};
fn average(v: &Vec<f32>) -> f32 {
     let sum: f32 = v.iter().sum();
    sum / v.len() as f32
}

fn main() {

    let mut probabilistic_skip_list: SkipList<KeyValuePair<String, Customer>> = SkipList::new(Deterministic);

    let csv_reader = csv::Reader::from_path("./src/utils/customers-2mil.csv");
    let mut records = csv_reader.expect("CSV read error");
    let deserialized_records: DeserializeRecordsIter<File,Customer> = records.deserialize();

    let checkpoints = vec![1, 5, 10, 50, 100, 500, 1_000, 5_000, 10_000, 50_000, 100_000, 500_000, 1_000_000, 1_500_000, 2_000_000];
    let mut samples_insert: Vec<(usize, f32)> = vec![];
    let mut samples_search: Vec<(usize, f32)> = vec![];
    let mut samples_delete: Vec<(usize, f32)> = vec![];

    for (i, result) in deserialized_records.enumerate() {
        let customer: Customer = result.expect("Csv read error");
        probabilistic_skip_list.insert(KeyValuePair(customer.customer_id.clone(), customer));

        if checkpoints.contains(&(i + 1)) {

            let mut curr_time_vec_insert: Vec<f32> = vec![];
            let mut curr_time_vec_search: Vec<f32> = vec![];
            let mut curr_time_vec_delete: Vec<f32> = vec![];

            for _ in 0..5000 {
                let dummy = Customer {
                    index: 0,
                    customer_id: "DUMMY".to_string(),
                    first_name: "".to_string(),
                    last_name: "".to_string(),
                    company: "".to_string(),
                    city: "".to_string(),
                    country: "".to_string(),
                    phone1: "".to_string(),
                    phone2: "".to_string(),
                    email: "".to_string(),
                    subscription_date: "".to_string(),
                    website: "".to_string(),
                };
                let curr_uuid_string = Uuid::new_v4().to_string();
                let copy_del = curr_uuid_string.clone();
                let copy_search = curr_uuid_string.clone();

                let t_start_insert = Instant::now();
                probabilistic_skip_list.insert(KeyValuePair(curr_uuid_string, dummy));
                curr_time_vec_insert.push(t_start_insert.elapsed().as_secs_f32());

                let t_start_search = Instant::now();
                probabilistic_skip_list.search(copy_search.to_string());
                curr_time_vec_search.push(t_start_search.elapsed().as_secs_f32());

                let t_start_delete = Instant::now();
                probabilistic_skip_list.delete(copy_del.to_string());
                curr_time_vec_delete.push(t_start_delete.elapsed().as_secs_f32());

            }

            // immediately delete dummy
            let elapsed_insert = average(&curr_time_vec_insert);
            samples_insert.push((i + 1, elapsed_insert));

            let elapsed_search = average(&curr_time_vec_search);
            samples_search.push((i + 1, elapsed_search));

            let elapsed_delete = average(&curr_time_vec_delete);
            samples_delete.push((i + 1, elapsed_delete));
        }
    }

    for (n, t) in &samples_insert {
        println!("Insert {} took {:.9} seconds", n, t);
    }

    for (n, t) in &samples_search {
        println!("Search {} took {:.9} seconds", n, t);
    }

    for (n, t) in &samples_delete {
        println!("Delete {} took {:.9} seconds", n, t);
    }

}
