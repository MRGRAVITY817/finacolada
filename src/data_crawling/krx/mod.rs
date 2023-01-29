pub mod download;
pub mod merge_table;
pub mod save;

pub enum InfoType {
    Sector,
    Individual,
}

pub enum MarketType {
    Kospi,
    Kosdaq,
}

pub type KrxSectorRow = (
    String, // issue code
    String, // issue name
    String, // market type: Kospi or Kosdaq
    String, // industry type
    u32,    // closing price
    i32,    // compared price
    f32,    // fluctuation rate
    u64,    // market cap
);

pub type KrxIndividualRow = (
    String, // issue code
    String, // issue name
    u32,    // closing price
    i32,    // compared price
    f32,    // fluctuation rate
    String, // EPS (number but contains `-`)
    String, // PER (number but contains `-`)
    String, // Leading EPS (number but contains `-`)
    String, // Leading PER (number but contains `-`)
    String, // BPS (number but contains `-`)
    String, // PBR (number but contains `-`)
    u32,    // Dividend Per Share
    f32,    // Dividend Yield Ratio
);
