use dotenv::dotenv;
use failure::Error;
use fehler::throws;
use futures::StreamExt;
use rs_deribit::models::subscription::{PrivateSubscribeRequest, PublicSubscribeRequest};
use rs_deribit::models::{
    AuthRequest, BuyRequest, CancelRequest, SellRequest, SubscriptionData, SubscriptionMessage,
    SubscriptionParams,
};
use rs_deribit::{Deribit, DeribitBuilder, DeribitError};
use std::env::var;
use tokio::runtime::Runtime;
use tokio::time::{delay_for, Duration};

struct SubscriptionTest {
    rt: Runtime,
    drb: Deribit,
    key: String,
    secret: String,
}

impl Default for SubscriptionTest {
    fn default() -> Self {
        let _ = dotenv();
        let _ = env_logger::try_init();
        Self {
            key: var("DERIBIT_TEST_MAIN_KEY").unwrap(),
            secret: var("DERIBIT_TEST_MAIN_SECRET").unwrap(),
            drb: DeribitBuilder::default().testnet(true).build().unwrap(),
            rt: Runtime::new().unwrap(),
        }
    }
}

// #[test]
// #[throws(Error)]
// fn announcements() {
//     let SubscriptionTest { drb, mut rt, .. } = SubscriptionTest::default();
//     let fut = async {
//         let (mut client, subscription) = drb.connect().await.unwrap();

//         let req = PrivateSubscribeRequest::new(&["announcements".into()]);
//         let _ = client.call(req).await.unwrap();

//         let v = subscription.take(1).collect::<Vec<_>>().await;
//         Ok::<_, Error>(v)
//     };

//
//
//     let v = rt.block_on(fut)?;
//     assert_eq!(v.len(), 1);
// }

#[test]
#[throws(Error)]
fn book() {
    let SubscriptionTest { drb, mut rt, .. } = SubscriptionTest::default();

    let fut = async {
        let (mut client, subscription) = drb.connect().await.unwrap();

        let req = PublicSubscribeRequest::new(&[
            "book.BTC-PERPETUAL.raw".into(),
            "book.ETH-PERPETUAL.raw".into(),
        ]);

        let _ = client.call(req).await.unwrap();

        let v = subscription.take(5).collect::<Vec<_>>().await;

        Ok::<_, Error>(v)
    };

    let v = rt.block_on(fut)?;
    assert_eq!(v.len(), 5);
    for v in v {
        match v {
            Ok(SubscriptionMessage {
                params:
                    SubscriptionParams::Subscription {
                        data: SubscriptionData::Book(..),
                        ..
                    },
                ..
            }) => {}
            _ => panic!(),
        }
    }
}

#[test]
#[throws(Error)]
fn grouped_book() {
    let SubscriptionTest { drb, mut rt, .. } = SubscriptionTest::default();

    let fut = async {
        let (mut client, subscription) = drb.connect().await.unwrap();

        let req = PublicSubscribeRequest::new(&[
            "book.BTC-PERPETUAL.10.20.100ms".into(),
            "book.ETH-PERPETUAL.10.20.100ms".into(),
        ]);

        let _ = client.call(req).await.unwrap();

        let v = subscription.take(5).collect::<Vec<_>>().await;
        Ok::<_, Error>(v)
    };

    let v = rt.block_on(fut)?;
    assert_eq!(v.len(), 5);
    for v in v {
        match v {
            Ok(SubscriptionMessage {
                params:
                    SubscriptionParams::Subscription {
                        data: SubscriptionData::GroupedBook(..),
                        ..
                    },
                ..
            }) => {}
            _ => panic!(),
        }
    }
}

#[test]
#[throws(Error)]
fn deribit_price_index() {
    let SubscriptionTest { drb, mut rt, .. } = SubscriptionTest::default();
    let fut = async {
        let (mut client, subscription) = drb.connect().await.unwrap();

        let req = PublicSubscribeRequest::new(&[
            "deribit_price_index.btc_usd".into(),
            "deribit_price_index.eth_usd".into(),
        ]);
        let _ = client.call(req).await.unwrap();

        let v = subscription.take(2).collect::<Vec<_>>().await;
        Ok::<_, Error>(v)
    };

    let v = rt.block_on(fut)?;
    assert_eq!(v.len(), 2);
    for v in v {
        match v {
            Ok(SubscriptionMessage {
                params:
                    SubscriptionParams::Subscription {
                        data: SubscriptionData::DeribitPriceIndex(..),
                        ..
                    },
                ..
            }) => {}
            _ => panic!(),
        }
    }
}

#[test]
#[throws(Error)]
fn deribit_price_ranking() {
    let SubscriptionTest { drb, mut rt, .. } = SubscriptionTest::default();
    let fut = async {
        let (mut client, subscription) = drb.connect().await.unwrap();

        let req = PublicSubscribeRequest::new(&[
            "deribit_price_ranking.btc_usd".into(),
            "deribit_price_ranking.eth_usd".into(),
        ]);
        let _ = client.call(req).await.unwrap();

        let v = subscription.take(2).collect::<Vec<_>>().await;
        Ok::<_, Error>(v)
    };

    let v = rt.block_on(fut)?;
    assert_eq!(v.len(), 2);
    for v in v {
        match v {
            Ok(SubscriptionMessage {
                params:
                    SubscriptionParams::Subscription {
                        data: SubscriptionData::DeribitPriceRanking(..),
                        ..
                    },
                ..
            }) => {}
            _ => panic!(),
        }
    }
}

#[test]
#[throws(Error)]
fn estimated_expiration_price() {
    let SubscriptionTest { drb, mut rt, .. } = SubscriptionTest::default();
    let fut = async {
        let (mut client, subscription) = drb.connect().await.unwrap();

        let req = PublicSubscribeRequest::new(&[
            "estimated_expiration_price.btc_usd".into(),
            "estimated_expiration_price.eth_usd".into(),
        ]);
        let _ = client.call(req).await.unwrap();

        let v = subscription.take(2).collect::<Vec<_>>().await;
        Ok::<_, Error>(v)
    };

    let v = rt.block_on(fut)?;
    assert_eq!(v.len(), 2);

    for v in v {
        match v {
            Ok(SubscriptionMessage {
                params:
                    SubscriptionParams::Subscription {
                        data: SubscriptionData::EstimatedExpirationPrice(..),
                        ..
                    },
                ..
            }) => {}
            _ => panic!(),
        }
    }
}

#[test]
#[throws(Error)]
fn markprice_options() {
    let SubscriptionTest { drb, mut rt, .. } = SubscriptionTest::default();
    let fut = async {
        let (mut client, subscription) = drb.connect().await.unwrap();

        let req = PublicSubscribeRequest::new(&[
            "markprice.options.btc_usd".into(),
            "markprice.options.eth_usd".into(),
        ]);
        let _ = client.call(req).await.unwrap();

        let v = subscription.take(2).collect::<Vec<_>>().await;
        Ok::<_, Error>(v)
    };

    let v = rt.block_on(fut)?;
    assert_eq!(v.len(), 2);

    for v in v {
        match v {
            Ok(SubscriptionMessage {
                params:
                    SubscriptionParams::Subscription {
                        data: SubscriptionData::MarkPriceOption(..),
                        ..
                    },
                ..
            }) => {}
            _ => panic!(),
        }
    }
}

#[test]
#[throws(Error)]
fn perpetual() {
    let SubscriptionTest { drb, mut rt, .. } = SubscriptionTest::default();
    let fut = async {
        let (mut client, subscription) = drb.connect().await.unwrap();

        let req = PublicSubscribeRequest::new(&[
            "perpetual.BTC-PERPETUAL.raw".into(),
            "perpetual.ETH-PERPETUAL.raw".into(),
        ]);
        let _ = client.call(req).await.unwrap();

        let v = subscription.take(2).collect::<Vec<_>>().await;
        Ok::<_, Error>(v)
    };

    let v = rt.block_on(fut)?;
    assert_eq!(v.len(), 2);

    for v in v {
        match v {
            Ok(SubscriptionMessage {
                params:
                    SubscriptionParams::Subscription {
                        data: SubscriptionData::Perpetual(..),
                        ..
                    },
                ..
            }) => {}
            _ => panic!(),
        }
    }
}

#[test]
#[throws(Error)]
fn quote() {
    let SubscriptionTest { drb, mut rt, .. } = SubscriptionTest::default();
    let fut = async {
        let (mut client, subscription) = drb.connect().await.unwrap();

        let req = PublicSubscribeRequest::new(&[
            "quote.BTC-PERPETUAL".into(),
            "quote.ETH-PERPETUAL".into(),
        ]);
        let _ = client.call(req).await.unwrap();

        let v = subscription.take(10).collect::<Vec<_>>().await;
        Ok::<_, Error>(v)
    };

    let v = rt.block_on(fut)?;
    assert_eq!(v.len(), 10);

    for v in v {
        match v {
            Ok(SubscriptionMessage {
                params:
                    SubscriptionParams::Subscription {
                        data: SubscriptionData::Quote(..),
                        ..
                    },
                ..
            }) => {}
            _ => panic!(),
        }
    }
}

#[test]
#[throws(Error)]
fn ticker() {
    let SubscriptionTest { drb, mut rt, .. } = SubscriptionTest::default();
    let fut = async {
        let (mut client, subscription) = drb.connect().await.unwrap();

        let req = PublicSubscribeRequest {
            channels: vec![
                "ticker.BTC-PERPETUAL.raw".into(),
                "ticker.ETH-PERPETUAL.raw".into(),
                "ticker.BTC-28JUN19.100ms".into(),
                "ticker.BTC-28JUN19.raw".into(),
                "ticker.BTC-28JUN19-7500-P.raw".into(),
                "ticker.BTC-28JUN19-7500-P.100ms".into(),
            ],
        };

        let _ = client.call(req).await.unwrap();

        let v = subscription.take(5).collect::<Vec<_>>().await;

        Ok::<Vec<_>, DeribitError>(v)
    };

    let v = rt.block_on(fut)?;
    assert_eq!(v.len(), 5);

    for v in v {
        match v {
            Ok(SubscriptionMessage {
                params:
                    SubscriptionParams::Subscription {
                        data: SubscriptionData::Ticker(..),
                        ..
                    },
                ..
            }) => {}
            _ => panic!(),
        }
    }
}

#[test]
#[throws(Error)]
fn trades() {
    let SubscriptionTest {
        mut rt,
        drb,
        key,
        secret,
    } = SubscriptionTest::default();

    let fut = async move {
        let (mut client, subscription) = drb.connect().await.unwrap();
        let _ = client
            .call(AuthRequest::credential_auth(&key, &secret))
            .await?;

        let req = PublicSubscribeRequest {
            channels: vec![
                "trades.BTC-PERPETUAL.raw".into(),
                "trades.ETH-PERPETUAL.raw".into(),
            ],
        };
        let _ = client.call(req).await.unwrap();

        client
            .call(BuyRequest::market("BTC-PERPETUAL", 10.))
            .await?
            .await?;

        client
            .call(SellRequest::market("BTC-PERPETUAL", 10.))
            .await?
            .await?;
        let v = subscription.take(2).collect::<Vec<_>>().await;
        Ok::<_, Error>(v)
    };

    let v = rt.block_on(fut)?;
    assert_eq!(v.len(), 2);

    for v in v {
        match v {
            Ok(SubscriptionMessage {
                params:
                    SubscriptionParams::Subscription {
                        data: SubscriptionData::Trades(..),
                        ..
                    },
                ..
            }) => {}
            _ => panic!(),
        }
    }
}

#[test]
#[throws(Error)]
fn user_orders() {
    let SubscriptionTest {
        mut rt,
        drb,
        key,
        secret,
    } = SubscriptionTest::default();

    let fut = async move {
        let (mut client, subscription) = drb.connect().await?;

        let _ = client
            .call(AuthRequest::credential_auth(&key, &secret))
            .await?
            .await?;

        let req = PrivateSubscribeRequest {
            channels: vec!["user.orders.BTC-PERPETUAL.raw".into()],
        };
        let _ = client.call(req).await?.await?;

        delay_for(Duration::from_secs(1)).await;

        let req = BuyRequest::limit("BTC-PERPETUAL", 100f64, 10f64);

        let resp = client.call(req).await?.await?;
        let id = resp.0.order.order_id;

        let v = subscription.take(1).collect::<Vec<_>>().await;
        let req = CancelRequest::new(&id);
        let resp = client.call(req).await?.await?;
        assert_eq!(id, resp.order.order_id);
        Ok::<_, Error>(v)
    };
    let v = rt.block_on(fut)?;

    for v in v {
        match v {
            Ok(SubscriptionMessage {
                params:
                    SubscriptionParams::Subscription {
                        data: SubscriptionData::UserOrders(..),
                        ..
                    },
                ..
            }) => {}
            _ => panic!(),
        }
    }
}

#[test]
#[throws(Error)]
fn user_portfolio() {
    let SubscriptionTest {
        mut rt,
        drb,
        key,
        secret,
    } = SubscriptionTest::default();

    let fut = async move {
        let (mut client, subscription) = drb.connect().await.unwrap();

        let _ = client
            .call(AuthRequest::credential_auth(&key, &secret))
            .await?;

        let req = PrivateSubscribeRequest::new(&[
            "user.portfolio.BTC".into(),
            "user.portfolio.ETH".into(),
        ]);
        let _ = client.call(req).await.unwrap();

        let v = subscription.take(2).collect::<Vec<_>>().await;
        Ok::<_, Error>(v)
    };

    let v = rt.block_on(fut)?;
    assert_eq!(v.len(), 2);

    for v in v {
        match v {
            Ok(SubscriptionMessage {
                params:
                    SubscriptionParams::Subscription {
                        data: SubscriptionData::UserPortfolio(..),
                        ..
                    },
                ..
            }) => {}
            _ => panic!(),
        }
    }
}

#[test]
#[throws(Error)]
fn user_trades() {
    let SubscriptionTest {
        mut rt,
        drb,
        key,
        secret,
    } = SubscriptionTest::default();

    let fut = async move {
        let (mut client, subscription) = drb.connect().await.unwrap();
        let _ = client
            .call(AuthRequest::credential_auth(&key, &secret))
            .await?
            .await?;

        let req = PrivateSubscribeRequest {
            channels: vec![
                "user.trades.BTC-PERPETUAL.raw".into(),
                "user.trades.ETH-PERPETUAL.raw".into(),
            ],
        };
        let _ = client.call(req).await?.await?;

        delay_for(Duration::from_secs(1)).await;

        client
            .call(BuyRequest::market("BTC-PERPETUAL", 10.))
            .await?
            .await?;

        client
            .call(SellRequest::market("BTC-PERPETUAL", 10.))
            .await?
            .await?;
        let v = subscription.take(2).collect::<Vec<_>>().await;
        Ok::<_, Error>(v)
    };

    let v = rt.block_on(fut)?;
    assert_eq!(v.len(), 2);

    for v in v {
        match v {
            Ok(SubscriptionMessage {
                params:
                    SubscriptionParams::Subscription {
                        data: SubscriptionData::UserTrades(..),
                        ..
                    },
                ..
            }) => {}
            _ => panic!(),
        }
    }
}

#[test]
#[throws(Error)]
fn sub_unsub() {
    let SubscriptionTest { drb, mut rt, .. } = SubscriptionTest::default();
    let fut = async {
        let (mut client, _) = drb.connect().await.unwrap();

        let req = PublicSubscribeRequest {
            channels: vec![
                "ticker.BTC-PERPETUAL.raw".into(),
                "ticker.ETH-PERPETUAL.raw".into(),
                "ticker.BTC-28JUN19.100ms".into(),
                "ticker.BTC-28JUN19.raw".into(),
                "ticker.BTC-28JUN19-7500-P.raw".into(),
                "ticker.BTC-28JUN19-7500-P.100ms".into(),
            ],
        };

        let _ = client.call(req).await.unwrap();
        let req = PublicSubscribeRequest::new(&[
            "ticker.BTC-PERPETUAL.raw".into(),
            "ticker.ETH-PERPETUAL.raw".into(),
            "ticker.BTC-28JUN19.100ms".into(),
            "ticker.BTC-28JUN19.raw".into(),
            "ticker.BTC-28JUN19-7500-P.raw".into(),
            "ticker.BTC-28JUN19-7500-P.100ms".into(),
        ]);

        client.call(req).await.unwrap();
        Ok::<(), DeribitError>(())
    };
    rt.block_on(fut)?;
}
