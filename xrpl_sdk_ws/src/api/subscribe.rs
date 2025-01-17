//! https://xrpl.org/subscribe.html

use crate::{client::Client, util::format_joined_keys, util::Result};

// #TODO consider a 'Builder' api.

impl Client {
    pub async fn subscribe_account(&mut self, account: &str) -> Result<()> {
        let id = self.next_id();
        let msg = format!(
            "{{\"id\": \"{id}\", \"command\": \"subscribe\", \"accounts\": [\"{account}\"]}}"
        );
        self.send(&msg).await?;
        Ok(())
    }

    pub async fn subscribe_accounts(&mut self, accounts: &[&str]) -> Result<()> {
        let id = self.next_id();
        let accounts = format_joined_keys(accounts);
        let msg =
            format!("{{\"id\": \"{id}\", \"command\": \"subscribe\", \"accounts\": [{accounts}]}}");
        self.send(&msg).await?;
        Ok(())
    }

    // #TODO consider renaming to `subscribe_topic`
    pub async fn subscribe_stream(&mut self, stream: &str) -> Result<()> {
        let id = self.next_id();
        let msg = format!(
            "{{\"id\": \"{id}\", \"command\": \"subscribe\", \"streams\": [\"{stream}\"]}}"
        );
        self.send(&msg).await?;
        Ok(())
    }

    // #TODO consider renaming to `subscribe_topics`
    pub async fn subscribe_streams(&mut self, streams: &[&str]) -> Result<()> {
        let id = self.next_id();
        let streams = format_joined_keys(streams);
        let msg =
            format!("{{\"id\": \"{id}\", \"command\": \"subscribe\", \"streams\": [{streams}]}}");
        self.send(&msg).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::DEFAULT_WS_URL;
    use futures_util::{SinkExt, StreamExt};

    #[tokio::test]
    async fn client_should_subscribe() {
        let mut client = Client::connect(DEFAULT_WS_URL)
            .await
            .expect("cannot connect");

        client
            .subscribe_streams(&["ledger"])
            .await
            .expect("cannot subscribe");

        let (_, rx) = client.stream.split();

        tokio::pin!(rx);

        while let Some(msg) = rx.next().await {
            dbg!(&msg);
        }
    }
}
