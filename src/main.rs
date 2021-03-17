extern crate elastic;

#[macro_use]
extern crate serde_json;

use elastic::prelude::*;
use serde_json::Value;
use std::error::Error;

fn run() -> Result<(), Box<dyn Error>> {
    let client = SyncClient::builder()
        .static_node("http://192.168.3.122:9200")
        .build()?;

    let res = client
        .search::<Value>()
        .index("logstash-syslog-2021.03.17")
        .body(json!({
            "query": {
                "bool": {
                    "must": [
                        {"regexp": {
                            "syslog_hostname.raw": {
                                "value": ".*ASR.*"
                            }
                        }}
                    ],
                    "filter": [
                        {"range": {
                            "@timestamp": {
                                "gte": "now-15m/m",
                                "lt": "now/m"
                            }
                        }}
                    ]
                }
            }
        }))
        .send()?;

    for hit in res.hits() {
        println!("{:?}", hit)
    }

    println!("{:?}", res);

    Ok(())
}


fn main() {
    run().unwrap();
}
