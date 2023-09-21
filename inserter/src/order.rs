use surrealdb::sql::{Id, Thing};
use surrealdb::{Connection, Surreal};

pub async fn insert<C: Connection>(
    db: Surreal<C>,
    oe: protos::vega::Order,
) -> surrealdb::Result<()> {
    // println!("{:?}", &oe);

    let sql = "update $id SET 
            market = $mid,
            party = $pid,
            side = $side,
            price = type::decimal($price), 
            size = type::decimal($size),
            remaining = type::decimal($remaining),
            time_in_force = $tif,
            type = $type,
            created_at = time::from::micros($created_at/1000),
            status = $status,
            expires_at = time::from::micros($expires_at/1000),
            reference = $reference,
            reason = $reason,
            updated_at =  time::from::micros($updated_at/1000),
            version = $version,
            batch_id = $batch_id,
            pegged_order = $pegged_order,
            liquidity_provision_id = $liquidity_provision_id,
            post_only = $post_only,
            reduce_only = $reduce_only,
            iceberg_order = $iceberg_order
            ;
        ";

    let result = db
        .query(sql)
        .bind(("id", Thing::from(("order", Id::from(&oe.id)))))
        .bind(("mid", Thing::from(("market", Id::from(&oe.market_id)))))
        .bind(("pid", Thing::from(("party", Id::from(&oe.party_id)))))
        .bind(("side", oe.side()))
        .bind(("price", &oe.price))
        .bind(("size", &oe.size))
        .bind(("remaining", &oe.remaining))
        .bind(("tif", &oe.time_in_force()))
        .bind(("type", oe.r#type()))
        .bind(("created_at", oe.created_at))
        .bind(("status", oe.status()))
        .bind(("expires_at", oe.expires_at))
        .bind(("reference", &oe.reference))
        .bind(("reason", oe.reason()))
        .bind(("updated_at", oe.updated_at))
        .bind(("version", oe.version))
        .bind(("batch_id", oe.batch_id))
        .bind(("pegged_order", &oe.pegged_order))
        .bind(("liquidity_provision_id", &oe.liquidity_provision_id)) // TODO linky?
        .bind(("post_only", oe.post_only))
        .bind(("reduce_only", oe.reduce_only))
        .bind(("iceberg_order", &oe.iceberg_order))
        .await?;
    result.check()?;
    Ok(())
}
