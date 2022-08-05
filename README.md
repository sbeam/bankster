# Bankster

The command-line app that acts as the ultimate back-end for all banking and
consumer finance purposes. There is definitely nothing it can't do to help you
track and display the status of all your bank's customer accounts.  And no edge
cases, nope.

## Description

Consumes a specific CSV format of account transation data, and outputs a report
describing final account status for each client mentioned in the input data.

Handles deposit and withdrawal of funds, along with dispute, resolution and
chargeback of indicated transactions. See `testdata.csv` for sample data
format.

## Usage

Takes a filename as the sole argument, or can read input data from STDIN.

### Run

```
cargo run transactions.csv
```

or

```
cat transactions.csv | cargo run
```

### Binaries

Binaries are not provided, but you can generate those like a champ with a simple:

```
cargo build -r
```

### Testing

```
cargo test
```

## TODO

  * [ ] Improved error logging and tracking. Currently, logs to STDOUT when error conditions are encounterd, eg unparsable data, overdrafts, or invalid chargebacks. These should
  probably be carefully logged, and the accounts flagged or marked for resolution before further transactions are processed.
  * [ ] Improved handling of bad data and edge cases, eg invalid or duplicate transaction ID's.
  * [ ] Efficiency improvements. With larger amounts of data, memory usage could become an issue. All deposits are currently memoized by transaction ID in case of future disputes. In a production system this should be offloaded to a separate key/value data store, possibly along with the complete list of transactions per account.
  * [ ] Determine what to do if a chargeback leads to a negative account balance. Should there be a time-hold on deposits, after which chargebacks can no longer be placed?
