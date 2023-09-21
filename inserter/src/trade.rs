use surrealdb::sql::{Id, Thing};
use surrealdb::{Connection, Surreal};

pub async fn insert<C: Connection>(
    db: Surreal<C>,
    t: protos::vega::Trade,
) -> surrealdb::Result<()> {
    // println!("{:?}", &t);
    let sql = "update $id SET
         market = $market,
         price = type::decimal($price),
         size = type::decimal($size),
         buyer = $buyer,
         seller = $seller,
         aggressor = $aggressor,
         buy_order = $buy_order,
         sell_order = $sell_order,
         timestamp = time::from::micros($timestamp/1000),
         type = $type,
         buyer_fee = $buyer_fee,
         seller_fee = $seller_fee,
         buyer_auction_batch = $buyer_auction_batch,
         seller_auction_batch = $buyer_auction_batch;
    ";

    let result = db
        .query(sql)
        .bind(("id", Thing::from(("trade", Id::from(&t.id)))))
        .bind(("market", Thing::from(("market", Id::from(&t.market_id)))))
        .bind(("price", &t.price))
        .bind(("size", &t.size))
        .bind(("buyer", Thing::from(("party", Id::from(&t.buyer)))))
        .bind(("seller", Thing::from(("party", Id::from(&t.seller)))))
        .bind(("aggressor", &t.aggressor))
        .bind(("buy_order", Thing::from(("order", Id::from(&t.buy_order)))))
        .bind((
            "sell_order",
            Thing::from(("party", Id::from(&t.sell_order))),
        ))
        .bind(("timestamp", t.timestamp))
        .bind(("type", t.r#type))
        .bind(("buyer_fee", t.buyer_fee))
        .bind(("seller_fee", t.seller_fee))
        .bind(("buyer_auction_batch", t.buyer_auction_batch))
        .bind(("seller_auction_batch", t.seller_auction_batch))
        .await?;
    result.check()?;

    Ok(())
}
