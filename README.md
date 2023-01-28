# Finacolada 🍍🥥

Minimal finance manager for Hoon and his family.  
Hope we all have a nice glass of pinacolada in sunny beach 🍹

## TODO

### Gathering data

- [x] Get the latest biz day from _Naver Finance_.
- [x] Get WICS sector info from _WiseIndex_.
- [ ] Get adjusted closing price.
- [ ] Get daily stock price for individual issue, from _Naver Finance_.
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

```bash
$ cargo test
# If snapshot isn't fully updated,
$ cargo insta test --review
```
