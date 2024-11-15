use tokio::task::spawn_blocking;
use regex::Regex;
use simplepush_rs::SimplePush;
use simplepush_rs::Message;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env = env_file_reader::read_file("./.env")?;

    let params = [("art", env["EGOV_ART"].as_str()), ("nummer", env["EGOV_NUMMER"].as_str()), ("B1", "Suche starten")];
    let client = reqwest::Client::new();
    let res = client.post(env["EGOV_URL"].as_str())
        .form(&params)
        .send()
        .await?
        .text()
        .await?;

    let re = Regex::new(r#"Dieser (Reisepass|Personalausweis) <\/font>
<font style="color:#000080;font-family:Arial;font-size:9pt;">(.*)<\/font>"#).unwrap();

    let text: String = re.captures_iter(&res).map(|caps| {
        let (_, [inner_text]) = caps.extract();
        inner_text
    }).collect();

    let mut msg: String = env["SP_PREFIX"].clone();
    msg.push_str(&text);

    let _ = spawn_blocking(move || {
        SimplePush::send(Message::new_with_encryption(
            env["SP_KEY"].as_str(),
            Some(env["SP_TITLE"].as_str()),
            &msg,
            None,
            None,
            env["SP_PASSWORD"].as_str(),
            Some(env["SP_SALT"].as_str()),
        )).expect("Unsuccessful SimplePush request")
    }).await;
    
    Ok(())
}
