use reqwest::{ Client as ReqwestClient };
use serde::{ Serialize, de::DeserializeOwned };
use anyhow::{ Context, Error };
use url::Url;

pub struct Client {
    token: Option<String>,
    client: ReqwestClient
}

impl Client {
    pub fn new(token: Option<String>) -> Client {
        Client {
            client: ReqwestClient::new(),
            token
        }
    }

    pub async fn request_with_cursor<U: AsRef<str>, F: Serialize + ?Sized, Res: DeserializeOwned, C: AsRef<str>>(&self, url: U, params: &F, cursor: Option<C>) -> Result<Res,Error> {

        let url_string = format!("https://slack.com/api/{}", url.as_ref());
        let mut url = Url::parse(&url_string)
            .with_context(|| format!("'{}' is not a valid url", url_string))?;

        if let Some(cursor) = cursor {
            url.query_pairs_mut().append_pair("cursor", cursor.as_ref());
        }

        let mut req = self.client
            .post(url)
            .form(params);

        if let Some(tok) = &self.token {
            req = req.bearer_auth(tok);
        }

        let res = req.send()
            .await
            .with_context(|| format!("Failed to make request to Slack"))?;

        res.json()
            .await
            .with_context(|| format!("Failed to decode response from Slack into valid JSON"))

    }

    pub async fn request<U: AsRef<str>, F: Serialize + ?Sized, Res: DeserializeOwned>(&self, url: U, params: &F) -> Result<Res,Error> {
        self.request_with_cursor(url, params, None as Option<String>).await
    }
}