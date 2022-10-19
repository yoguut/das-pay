use csv;
use das_pay;
use std::fs::File;
use std::io::Write;
use tempdir;

#[test]
fn should_fail_deserialize_on_invalid_transaction_type() {
    let tmpdir = tempdir::TempDir::new("malformed").unwrap();
    let path = tmpdir.path().join("-fixture.csv");
    let mut tmpfile = File::create(&path).unwrap();
    let contents = "\
type,client,tx,amount
invalid,1,2,1.03
    ";
    writeln!(tmpfile, "{}", contents).unwrap();
    let rdr = csv::Reader::from_path(&path).unwrap();
    let err = das_pay::sequential_serde(rdr).unwrap_err();
    assert!(err.to_string().contains("Invalid type"));
    drop(tmpfile);
    tmpdir.close().unwrap();
}
