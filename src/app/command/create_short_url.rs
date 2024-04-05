pub struct CreateShortUrl;

impl CreateShortUrl {
    pub async fn execute(&self, full_url: String) -> Result<String, String> {
        Ok("1".to_owned())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_short_url() {
        let command = CreateShortUrl;

        let result = command.execute("https://www.google.com".to_owned()).await;

        assert_ne!(result, Ok("".to_owned()));
    }
}