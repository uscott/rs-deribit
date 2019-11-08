use deribit::models::{AuthRequest, Currency, GetTransfersRequest};
use deribit::DeribitBuilder;
use dotenv::dotenv;
use failure::{Error, Fallible};
use std::env::var;
use tokio::runtime::Runtime;

#[test]
fn get_transfers() -> Fallible<()> {
    let _ = dotenv();
    let key = var("DERIBIT_KEY").unwrap();
    let secret = var("DERIBIT_SECRET").unwrap();

    let drb = DeribitBuilder::default().testnet(true).build().unwrap();
    let rt = Runtime::new()?;

    let fut = async move {
        let (mut client, _) = drb.connect().await?;
        let req = AuthRequest::credential_auth(&key, &secret);
        let _ = client.call(req).await?.await?;

        let req = GetTransfersRequest::with_currency(Currency::BTC);
        Ok::<_, Error>(client.call(req).await?.await?)
    };
    let _ = rt.block_on(fut)?;
    Ok(())
}
