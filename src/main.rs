use regex::Regex;

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
        let (_, [_, inner_text]) = caps.extract();
        inner_text
    }).collect();

    let mut msg: String = env["GOTIFY_PREFIX"].clone();
    msg.push_str(&text);

    let client = gotify::Client::new_unauthenticated(env["GOTIFY_URL"].as_str())?;
    let client = client.authenticate(env["GOTIFY_TOKEN"].as_str())?;

    client.create_message(msg).with_title(env["GOTIFY_TITLE"].clone()).await?;
    
    Ok(())
}
