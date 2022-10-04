use serde_json::json;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub fn cloudwatch_metric(dimension: &str, name: &str, error: bool, error_message: Option<String>) {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string()
        .parse::<u64>()
        .unwrap();
    let metric = serde_json::to_string(&json!(
      {
        "_aws": {
          "Timestamp": timestamp,
          "CloudWatchMetrics": [
            {
              "Namespace": "Crypteia",
              "Dimensions": [["All", dimension]],
              "Metrics": [
                { "Name": name, "Unit": "Count"}
              ]
            }
          ]
        },
        "All": "all",
        dimension: dimension,
        name: 1,
        "ErrorMessage": error_message.unwrap_or("".to_string()),
      }
    ))
    .unwrap();
    if error {
        eprintln!("{}", metric);
    } else {
        println!("{}", metric);
    }
}
