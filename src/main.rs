// Standard Library
use std::env;
use crate::dns::{DnsClient, DnsRecordData, DnsRecordType, DnsResponse};
use colored::*;
use regex::Regex;
use std::str::FromStr;
use crate::console::Console;

extern crate strum;
#[macro_use] extern crate strum_macros;

// Local modules
mod dns;
mod byte_serializable;
mod console;

fn main () {
    // Read command line args
    let args: Vec<String> = env::args().collect();
    
    // Create DnsClient
    let mut dns_client = DnsClient::new ();
    let dns_result: DnsResponse;

    // Welcome users
    println!("{}\n", "DouG v0.1".bold ().blue ());

    // Save url for later
    let query_url: String;

    // No args? Interactive!
    if args.len () == 1 {
        println!("{}", "Entering interactive mode...".bright_black ());
        let url = Console::prompt ("What domain would you like to look up?");
        let record_type = Console::prompt ("What record type do you wish to use?");

        query_url = url.clone ();
        dns_result = dns_client.lookup (url, DnsRecordType::from_str (&record_type).unwrap ());
        println!("");
    } else {
        // Prepare arguments (url, dns, type)
        let mut query: (String, String, String) = (
            "".to_string (),
            "".to_string (),
            "A".to_string ()
        );

        let dns_regex = Regex::new (r"@.+").unwrap ();
        let type_regex = Regex::new (r"[A-Z]{1,5}").unwrap ();

        for itm in args {
            if dns_regex.is_match (&itm) {
                query.1 = itm.chars ().skip (1).collect ();
            } else if type_regex.is_match (&itm) {
                query.2 = itm;
            } else {
                query.0 = itm;
            }
        }

        if query.1.chars ().count () > 0 {
            dns_client.set_dns_address (query.1);
        }

        query_url = query.0.clone ();
        dns_result = dns_client.lookup (query.0, DnsRecordType::from_str (&*query.2).unwrap ());
    }

    println!("Host:\t\t{}", query_url.blue ());
    println!("DNS Server:\t{}", dns_client.dns_addr.ip ().to_string ().blue ());
    println!("Record Type:\t{}", dns_result.question.r#type.to_string ().blue ());
    println!("");

    for record_index in 0..dns_result.resource_records.len () {
        if record_index == 0 {
            println!("{}", "Answer Section:".bright_black ());
        } else if record_index == dns_result.header.answer_count as usize {
            println!("\n{}", "Authority Section:".bright_black ());
        } else if record_index == (dns_result.header.answer_count + dns_result.header.authority_count) as usize {
            println!("\n{}", "Additional Section:".bright_black ());
        }

        let record = &dns_result.resource_records [record_index];
        let data: String = match &record.parsed_data {
            DnsRecordData::A {ip_addr} => ip_addr.to_string (),
            DnsRecordData::MX {priority, name} => format!("{} ({})", name, priority),
            DnsRecordData::None | _ => "<Unsupported>".to_owned ()
        };

        let domain: &String = &dns_result.question.name;
        let record_type: String = record.r#type.to_string ();

        println!("{}\t\t{}\t\t{}", domain.blue (), record_type, data.green ());
    }
}

