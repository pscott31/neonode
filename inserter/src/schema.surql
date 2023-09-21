REMOVE db vega;
DEFINE db vega;
DEFINE ns vega;

DEFINE TABLE party SCHEMAFULL;
DEFINE TABLE market SCHEMAFULL;
DEFINE TABLE orders SCHEMALESS;

DEFINE TABLE order SCHEMAFULL;
DEFINE FIELD market ON TABLE order TYPE record(market) ASSERT type::is::record($value);
DEFINE FIELD party ON TABLE order TYPE record(party) ASSERT type::is::record($value);
DEFINE FIELD side ON TABLE order TYPE string ASSERT type::is::string($value);
DEFINE FIELD price ON TABLE order TYPE decimal ASSERT type::is::decimal($value);
DEFINE FIELD size ON TABLE order TYPE decimal ASSERT type::is::decimal($value);
DEFINE FIELD remaining ON TABLE order TYPE decimal ASSERT type::is::decimal($value);
DEFINE FIELD time_in_force ON TABLE order TYPE string ASSERT type::is::string($value);
DEFINE FIELD created_at ON TABLE order TYPE datetime ASSERT type::is::datetime($value);
DEFINE FIELD status ON TABLE order TYPE string ASSERT type::is::string($value);
DEFINE FIELD expires_at ON TABLE order TYPE datetime ASSERT type::is::datetime($value);
DEFINE FIELD reference ON TABLE order TYPE string ASSERT type::is::string($value);
DEFINE FIELD reason ON TABLE order TYPE string ASSERT type::is::string($value);
DEFINE FIELD updated_at ON TABLE order TYPE datetime ASSERT type::is::datetime($value);
DEFINE FIELD version ON TABLE order TYPE int ASSERT type::is::int($value);
DEFINE FIELD batch_id ON TABLE order TYPE int ASSERT type::is::int($value);
DEFINE FIELD pegged_order ON TABLE order TYPE option<object>;
DEFINE FIELD liquidity_provision_id ON TABLE order TYPE string ASSERT type::is::string($value);
DEFINE FIELD post_only ON TABLE order TYPE bool ASSERT type::is::bool($value);
DEFINE FIELD reduce_only ON TABLE order TYPE bool ASSERT type::is::bool($value);
-- DEFINE FIELD iceberg_order ON TABLE order TYPE 

DEFINE EVENT updated ON TABLE order THEN {
    LET $party = $after.party;
    LET $market = $after.market;
    UPDATE $party;
    UPDATE $market;
    RELATE $party->orders->$this;
    RELATE $market->orders->$this;
};

DEFINE EVENT updated ON TABLE trade THEN {
    LET $buyer = $after.buyer;
    LET $seller = $after.seller;
    LET $market = $after.market;
    RELATE $buyer->trades->$this set side='buy';
    RELATE $seller->trades->$this set side='sell';
    RELATE $market->trades->$this;
};

DEFINE EVENT updated ON TABLE ledger_entry THEN {
    LET $from_party = type::thing('party', $after.from_account.owner);
    LET $to_party = type::thing('party', $after.to_account.owner);
    RELATE $from_party->tranfered->$this set side='sent';
    RELATE $from_party->tranfered->$this set side='received';
};
