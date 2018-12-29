use colored::*;
use std::io::*;
use crate::dns::*;

pub struct Console;

impl Console {
    pub fn prompt (question: &str) -> String {
        print!("{} {} ", "?".green (), question.bold ());
        stdout ().flush ().expect ("Failed to flush Stdout");

        let stdin = stdin();
        let mut handle = stdin.lock ();
        let mut answer = String::new ();

        handle.read_line (&mut answer).expect ("Failed to read from Stdin");
        answer.pop ();

        println!("\x1b[1A{} {} {}", "?".green (), question.bold (), answer.cyan ());
        stdout ().flush ().expect ("Failed to flush Stdout");
        return answer;
    }

    pub fn status (status: &str) {
        println!("{}", status.bright_black ());
    }

    pub fn space (count: u8) {
        for _ in 0..count {
            println!("");
        }
    }

    pub fn title (title: &str) {
        println!("{}", title.bold ().blue ());
    }

    pub fn info (key: &str, value: &String) {
        println!("{}\t{}", key, value.blue ());
    }

    pub fn resource_record (response: &DnsMessage, record: &DnsResourceRecord) {
        let data: String = match &record.parsed_data {
            DnsRecordData::A {ip_addr} => ip_addr.to_string (),
            DnsRecordData::MX {priority, name} => format!("{} ({})", name, priority),
            DnsRecordData::AAAA {ip_addr} => ip_addr.to_string (),
            DnsRecordData::NS {name} => format!("{}", name),
            DnsRecordData::CNAME {name} => format!("{}", name),
            DnsRecordData::PTR {name} => format!("{}", name),
            DnsRecordData::TXT {text} => format!("\"{}\"", text),
            DnsRecordData::None | _ => "<Unsupported>".to_owned ()
        };

        let domain: &String = &response.question.name;
        let record_type: String = record.r#type.to_string ();

        println!("{}\t\t{}\t{}\t\t{}", domain.blue (), record_type, record.ttl.to_string (), data.green ());
    }
}