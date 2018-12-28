// Standard Library
use std::env;
use crate::dns::{DnsClient, DnsRecordType, DnsResponse};
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

    // No args? Interactive!
    if args.len () == 1 {
        println!("{}", "Entering interactive mode...".bright_black ());
        let url = Console::prompt ("What domain would you like to look up?");
        let record_type = Console::prompt ("What record type do you wish to use?");

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

        dns_result = dns_client.lookup (query.0, DnsRecordType::from_str (&*query.2).unwrap ());
    }

    println!("Using DNS Server:\t{}", dns_client.dns_addr.ip ().to_string ().blue ());
    println!("Querying Record Type:\t{}", dns_result.question.r#type.to_string ().blue ());
    println!("");

    for record in dns_result.resource_records {
        let ip: String = record.data.iter ().map (|itm| itm.to_string ()).collect::<Vec<String>>().join (".");
        let domain: &String = &dns_result.question.name;
        let record_type: String = record.r#type.to_string ();

        println!("{}\t\t{}\t\t{}", domain.blue (), record_type, ip.green ());
    }
}

