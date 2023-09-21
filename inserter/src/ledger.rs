use surrealdb::sql::{Id, Thing};
use surrealdb::{Connection, Surreal};

pub async fn insert<C: Connection>(
    db: Surreal<C>,
    lms: protos::vega::events::v1::LedgerMovements,
) -> surrealdb::Result<()> {
    for lm in lms.ledger_movements {
        for le in lm.entries {
            insert_le(db.clone(), le).await?;
        }
    }

    Ok(())
}

async fn insert_le<C: Connection>(
    db: Surreal<C>,
    le: protos::vega::LedgerEntry,
) -> surrealdb::Result<()> {
    println!("{:?}", &le);
    let sql = "create ledger_entry SET 
    from_account = $from_account,
    to_account = $to_account,
    amount = type::decimal($amount),
    type = $type,
    timestamp = time::from::micros($timestamp/1000),
    from_account_balance = type::decimal($from_account_balance),
    to_account_balance = type::decimal($to_account_balance);
    ";

    let result = db
        .query(sql)
        .bind(("from_account", &le.from_account))
        .bind(("to_account", &le.to_account))
        .bind(("amount", &le.amount))
        .bind(("type", le.r#type()))
        .bind(("timestamp", le.timestamp))
        .bind(("from_account_balance", &le.from_account_balance))
        .bind(("to_account_balance", &le.to_account_balance))
        .await?;
    result.check()?;
    Ok(())
}
