# plume-dev-formatter-meter

`plume-dev-formatter-meter` is a Rust project in developer tools. Its focus is to build a Rust toolkit that studies formatter behavior through framed sample traffic, with bounds and ordering tests and no network dependency.

## Use Case

The point is to make a small domain rule concrete enough that a reader can change it and immediately see what broke.

## Plume Dev Formatter Meter Review Notes

For a quick review, compare `change width` with `review cost` before reading the middle cases.

## Highlights

- `fixtures/domain_review.csv` adds cases for change width and diagnostic quality.
- `metadata/domain-review.json` records the same cases in structured form.
- `config/review-profile.json` captures the read order and the two review questions.
- `examples/plume-dev-formatter-walkthrough.md` walks through the case spread.
- The Rust code includes a review path for `change width` and `review cost`.
- `docs/field-notes.md` explains the strongest and weakest cases.

## Code Layout

The fixture data drives the tests. The code stays thin, while `metadata/domain-review.json` and `config/review-profile.json` explain what each case is meant to protect.

The Rust addition stays small enough to inspect in one sitting.

## Run The Check

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

## Regression Path

The same command runs the local verification path. The highest-scoring domain case is `stale` at 216, which lands in `ship`. The most cautious case is `edge` at 145, which lands in `ship`.

## Future Work

This remains a local project with deterministic fixtures. It does not depend on credentials, hosted services, or live data. Future work should add richer malformed inputs before widening the public API.
