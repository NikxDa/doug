// Standard Library
use std::env;
use crate::dns::{DnsClient, DnsRecordType, DnsClass};
use colored::*;
use regex::Regex;
use std::str::FromStr;

extern crate strum;
#[macro_use] extern crate strum_macros;

// Local modules
mod dns;
mod byte_serializable;

fn main () {
    // Read command line args
    let args: Vec<String> = env::args().collect();

    // No args? Interactive!
    if args.len () == 1 {
        println!("INTERACTIVE MODE");
    } else {
        // Prepare arguments (url, dns, type)
        let mut query: (String, String, String) = (
            "".to_string (),
            "".to_string (),
            "A".to_string ()
        );

        let dns_regex = Regex::new (r"@[0-9\.]+").unwrap ();
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

        let mut dns_client = DnsClient::new ();

        if query.1.chars ().count () > 0 {
            dns_client.set_dns_address (query.1);
        }

        let dns_result = dns_client.lookup (query.0, DnsRecordType::from_str (&*query.2).unwrap ());

        println!("DouG v0.1\n");
        println!("Using DNS server: {}\n", dns_client.dns_addr.ip ().to_string ().blue ());

        for record in dns_result.resource_records {
            let ip: String = record.data.iter ().map (|itm| itm.to_string ()).collect::<Vec<String>>().join (".");
            let domain: &String = &dns_result.question.name;
            let record_type: String = record.r#type.to_string ();

            println!("{}\t\t{}\t\t{}", domain.blue (), record_type, ip.green ());
        }
    }
}

