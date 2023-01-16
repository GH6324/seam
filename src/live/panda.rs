use std::collections::HashMap;

use anyhow::{Ok, Result};

const URL: &str = "https://api.pandalive.co.kr/v1/live/play/";

use crate::{
    common::CLIENT,
    model::{Node, ShowType},
};

/// pandalive
///
/// https://www.pandalive.co.kr/
pub async fn get(rid: &str) -> Result<ShowType> {
    let mut form = HashMap::new();
    form.insert("action", "watch");
    form.insert("userId", rid);
    let json: serde_json::Value = CLIENT.post(URL).form(&form).send().await?.json().await?;
    match &json["PlayList"] {
        serde_json::Value::Null => Ok(ShowType::Off),
        list => {
            let mut nodes = vec![];
            for item in ["hls", "hls2", "hls3", "rtmp"] {
                if list.get(item).is_some() {
                    nodes.push(Node {
                        rate: "原画".to_string(),
                        url: list[item][0]["url"].as_str().unwrap().to_string(),
                    });
                }
            }
            Ok(ShowType::On(nodes))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::match_show_type;

    #[tokio::test]
    async fn test_panda() {
        match_show_type(get("wpsl1002").await.unwrap());
    }
}
