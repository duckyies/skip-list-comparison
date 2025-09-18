
mod skip_list;

use std::time;
use std::time::{Duration, Instant};
use csv;
use serde::Deserialize;

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

fn main() {

    let curr = Instant::now();

    let mut skip_list: SkipList<KeyValuePair<String, Customer>> = SkipList::new(Probabilistic);
    let csv_reader = csv::Reader::from_path("./src/customers-2mil.csv");
    if let Ok(mut records) = csv_reader {
        for result in records.deserialize() {
            let customer: Customer = result.expect("Csv read error");
            skip_list.insert(KeyValuePair(customer.customer_id.clone(), customer)).expect("PANICCCC!!!!!!!!!! get a better csv!!!!");
        }
    }
    else {
        println!("Csv read error!")
    }

    println!("Time spent inserting - {:?}",curr.elapsed());

    let time_search = Instant::now();
    skip_list.search("79DBd7f1161fb04".to_string());
    println!("Time spent searching - {:?}",time_search.elapsed());

    println!("{:?}",skip_list.length());

    skip_list.delete("79DBd7f1161fb04".to_string());
    println!("{:#?}",skip_list.search("79DBd7f1161fb04".to_string()));
    let cus = Customer {
        index: 0,
        customer_id: "".to_string(),
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
    let time_insert_new = Instant::now();
    skip_list.insert(KeyValuePair("88z".to_string(), cus));
    println!("Time spent inserting - {:?}",time_insert_new.elapsed());
    let time_searchs = Instant::now();
    skip_list.search("88z".to_string());
    println!("Time spent searching - {:?}",time_searchs.elapsed());
}
