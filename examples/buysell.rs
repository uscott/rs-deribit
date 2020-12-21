use dotenv::dotenv;
use failure::Error;
use fehler::throws;
use ex_drbt::models::{AuthRequest, BuyRequest, SellRequest};
use ex_drbt::DeribitBuilder;
use std::env::var;

#[throws(Error)]
#[tokio::main]
async fn main() {
    let _ = dotenv();

    let key = var("DERIBIT_TEST_MAIN_KEY").unwrap();
    let secret = var("DERIBIT_TEST_MAIN_SECRET").unwrap();

    let drb = DeribitBuilder::default().testnet(true).build().unwrap();

    let (mut client, _) = drb.connect().await?;

    let req = AuthRequest::credential_auth(&key, &secret);
    let _ = client.call(req).await?;

    let req = BuyRequest::market("BTC-PERPETUAL", 10f64);
    let resp = client.call(req).await?;
    println!("{:?}", resp.await?);

    let req = SellRequest::market("BTC-PERPETUAL", 10f64);
    let resp = client.call(req).await?;
    println!("{:?}", resp.await?);
}
