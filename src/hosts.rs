use regex::Regex;
use std::fs::File;
use std::io::{Read, Result, Write};
use std::path::Path;

const HOSTS: &str = r"C:\Windows\System32\drivers\etc\hosts";

pub fn set_hosts(hosts: &Vec<String>) -> Result<()> {
    let p = &Path::new(HOSTS);
    let mut f = File::open(p)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    for host in hosts {
        let h = String::from(r"\s*\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\s+");
        let p = h + host;
        let r = Regex::new(p.as_str()).unwrap();
        let c = r.captures(&content.as_str());
        if c.is_none() {
            let h = String::from("\r\n127.0.0.1 ");
            content += &(h + host);
            println!("insert {0}", host);
        } else {
            println!("exists {0}", host);
        }
    }
    let mut writer = File::create(p).unwrap();
    writer.write_all(content.as_bytes())?;
    Ok(())
}
