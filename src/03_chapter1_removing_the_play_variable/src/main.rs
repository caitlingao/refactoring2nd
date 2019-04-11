use serde::{Deserialize, Serialize};
use serde_json::{Value};
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize)]
struct Invoice {
    customer: String,
    performances: Vec<Perf>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Perf {
    play_id: String,
    audience: i32,
}

fn statement(invoice: &Invoice) -> String {
    let mut total_amount = 0;
    let mut volume_credits = 0;
    let mut result = String::new();
    result.push_str(&format!("Statement for {}\n", &invoice.customer));

    for perf in &invoice.performances {
        // add volume credits
        volume_credits += 0.max(&perf.audience - 30);

        // add extra credit for every ten comedy attendees
        if play_for(&perf)["type"] == "comedy" { volume_credits += &perf.audience / 5; }

        // print line for this order
        result.push_str(&format!("  {}: ${} ({} seats)\n", play_for(&perf)["name"].as_str().unwrap(), &(amount_for(&perf) / 100).to_string(), &perf.audience.to_string()));
        total_amount += amount_for(&perf);
    }

    result.push_str(&format!("Amount owed is ${}\n", &(total_amount / 100).to_string()));
    result.push_str(&format!("You earned {} credits\n", &volume_credits.to_string()));
    result
}

fn amount_for(a_performance: &Perf) -> i32 {
    let mut result = 0;

    match play_for(a_performance)["type"].as_str() {
        Some("tragedy") => {
            result = 40000;
            if a_performance.audience > 30 { result += 1000 * (a_performance.audience - 30); }
        },
        Some("comedy") => {
            result = 3000;
            if a_performance.audience > 20 { result += 10000 + 500 * (a_performance.audience - 20); }
            result += 3000 * a_performance.audience;
        },
        _ => { panic!("unknown type: {:?}", play_for(a_performance)["type"].as_str()); }
    }

    result
}

fn play_for(a_performance: &Perf) -> Value  {
    plays()[&a_performance.play_id].clone()
}

fn plays() -> Value {
    let mut plays_file = File::open("./util/plays.json").unwrap();
    let mut plays_data = String::new();
    plays_file.read_to_string(&mut plays_data).unwrap();

    return serde_json::from_str(&mut plays_data).unwrap();
}

fn main() {
    let mut invoice_file = File::open("./util/invoice.json").unwrap();
    let mut invoice_data = String::new();
    invoice_file.read_to_string(&mut invoice_data).unwrap();
    let invoice: Invoice = serde_json::from_str(&mut invoice_data).unwrap();

    println!("{}", statement(&invoice));
}
