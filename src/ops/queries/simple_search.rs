use elastic::{
    error::Error,
    prelude::*,
};
use ops::Client;
use serde_json::Value;

// use model::account::Account;

pub trait SimpleSearchQuery {
    fn simple_search_query(&self) -> Result<SearchResponse<Value>, Error>;
}

impl SimpleSearchQuery for Client {
    fn simple_search_query(&self) -> Result<SearchResponse<Value>, Error> {
        let query = json!({
            "query": {
                "bool": {
                    "must": [
                        {"regexp": {
                            "Devices_name.keyword": {
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
        });

        self.io
            .search()
            .index("nms-devices_status-test-2020.07")
            .body(query)
            .send()
    }
}
