use reqwest::Client;
use serde_json::{Value};


use recombox_plugin_provider::select_source::{InputPayload, OuputPayloadInfo, OuputPayload};


pub async fn new(input: InputPayload) -> anyhow::Result<OuputPayload>{
    
    let client = Client::new();
    let res = client.get(format!("https://apibay.org/q.php?q={}&cat=201", input.id))
        .send()
        .await?;

    let data: Value = res.json().await?;

    let data_vec = data.as_array()
        .ok_or("Unable to convert to vec")
        .map_err(|e| anyhow::Error::msg(e))?;

    let mut new_output_payload = OuputPayload(vec![]);

    for item in data_vec{
        let id = item.get("id")
            .ok_or("Unable to get id")
            .map_err(|e| anyhow::Error::msg(e))?
            .as_str()
            .ok_or("Unable to convert to str")
            .map_err(|e| anyhow::Error::msg(e))?
            .to_string();

        let title = item.get("name")
            .ok_or("Unable to get title")
            .map_err(|e| anyhow::Error::msg(e))?
            .as_str()
            .ok_or("Unable to convert to str")
            .map_err(|e| anyhow::Error::msg(e))?
            .to_string();

        new_output_payload.0.push(OuputPayloadInfo{
            id,
            title
        })
    }

    return Ok(new_output_payload);
    

    // return Err(anyhow::Error::msg("Not implemented"));

}