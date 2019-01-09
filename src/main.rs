// Standard Library
use std::env;
use crate::dns::{DnsClient, DnsRecordType, DnsMessage};
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

    // Welcome users
    Console::title ("Doug v0.1");
    Console::space (1);

    // Grab result from either interactive of argument mode
    let dns_result: DnsMessage = match args.len () {
        1 => interactive_mode (&dns_client),
        _ => default_mode (&mut dns_client, &args)
    };

    // Print info
    Console::status (&*format!(
        "Querying {} records of {} via {}...",
        dns_result.question.r#type.to_string ().cyan (),
        dns_result.question.name.cyan (),
        dns_client.dns_addr.ip ().to_string ().cyan ()
    ));
    Console::space (1);

    // Print all resource records with section titles
    for record_index in 0..dns_result.resource_records.len () {
        if record_index == 0 {
            println!("{}", "Answer Section:".bright_black ());
        } else if record_index == dns_result.header.answer_count as usize && dns_result.header.authority_count > 0 {
            println!("\n{}", "Authority Section:".bright_black ());
        } else if record_index == (dns_result.header.answer_count + dns_result.header.authority_count) as usize {
            println!("\n{}", "Additional Section:".bright_black ());
        }

        let record = &dns_result.resource_records [record_index];
        Console::resource_record(&dns_result, record);
    }
}

fn interactive_mode (dns_client: &DnsClient) -> DnsMessage {
    // Print interactive notice
    Console::status ("Entering interactive mode...");

    // Query necessary data
    let url = Console::prompt ("What domain would you like to look up?");
    let record_type = Console::prompt ("What record type do you wish to use?");
    Console::space (1);

    // Match the record type, fallback to A
    let record_type = match DnsRecordType::from_str (&record_type) {
        Ok(itm) => itm,
        Err(_) => DnsRecordType::A
    };

    // Return the response
    dns_client.query (url, record_type)
}

fn default_mode (dns_client: &mut DnsClient, args: &Vec<String>) -> DnsMessage {
    // Prepare arguments (url, dns, type)
    let mut query: (String, String, String) = (
        "".to_owned (), 
        "".to_owned (), 
        "A".to_owned ()
    );

    // Prepare Regex to read data
    let dns_regex = Regex::new (r"@.+").unwrap ();
    let type_regex = Regex::new (r"[A-Z]{1,5}").unwrap ();

    // Parse arguments
    for itm in args {
        if dns_regex.is_match (&itm) {
            query.1 = itm.chars ().skip (1).collect::<String>();
        } else if type_regex.is_match (&itm) {
            query.2 = itm.clone ();
        } else {
            query.0 = itm.clone ();
        }
    }

    // Set the DNS address if necessary
    if query.1.chars ().count () > 0 {
        dns_client.set_dns_address (query.1.to_owned ());
    }

    // Return the response
    dns_client.query (query.0.to_owned (), DnsRecordType::from_str (&*query.2).unwrap ())
}

