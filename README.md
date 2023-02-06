# Finacolada 🍍🥥

Minimal finance manager for Hoon and his family.  
Hope we all have a nice glass of pinacolada in sunny beach 🍹

## TODO

### Gathering data

- [x] Get the latest biz day from _Naver Finance_.
- [x] Get WICS sector info from _WiseIndex_.
- [x] Get daily stock price for individual issue, from _Naver Finance_.
- [X] Get financial statements from _FN Guide_.
  - [x] Save annual data of each ticker into parquet file.
  - [x] Calculate value metrics (PER, PBR, PCR, PSR).
  - [ ] Do this for every issues.
- [ ] Get various data from _DART_.
  - [ ] Use provided API to get data.
  - [ ] Design the the data pipeline for these data.
  - [ ] Get disclosure info.
  - [ ] Get business report.
- [x] Combine individual & sector info from _KRX_.
  - [x] Get OTP for KOSPI & KOSDAQ.
  - [x] KRX_SEC: Industrial classification status by sector, KOSPI & KOSDAQ (KRX_SEC)
  - [x] KRX_IND PER/PBR/Dividend for individual issue.
  - [x] Merge KRX_SEC and KRX_IND, remove intersections with none-sense data.
  - [x] Ignore preferred stock & spac stock.

### Organizing & Visualizing data

- [ ] _add todo item..._

### Add quantitative strategy

- [ ] _add todo item..._

## Testing

### Normal testing

```bash
$ cargo test

# If snapshot isn't fully updated,
$ cargo insta test --review
```

### Code Coverage

```bash
# Generate source testing profile
$ CARGO_INCREMENTAL=0 \
        RUSTFLAGS='-Cinstrument-coverage' \
        LLVM_PROFILE_FILE='profile/cargo-test-%p-%m.profraw' \
        cargo test

# Generate coverage result in target/coverage/html/index.html
$ grcov profile \
        --binary-path ./target/debug/deps/ \
        -s . -t html --branch --ignore-not-existing \
        --ignore '../*' --ignore "/*" -o target/coverage/html
```
