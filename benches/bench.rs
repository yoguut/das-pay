#![feature(test)]

extern crate das_pay;
extern crate test;

use csv;
use std::{fs::File, io::Write};
use tempdir;

#[bench]
fn bench_serde(b: &mut test::Bencher) {
    let tmpdir = tempdir::TempDir::new("large").unwrap();
    let path = tmpdir.path().join("-fixture.csv");
    let mut tmpfile = File::create(&path).unwrap();
    let mut buffer = String::from("type,client,tx,amount\n");
    for n in 1..10000 {
        if n % 2 == 0 {
            let line = format!("deposit,1,{},2.0\n", n);
            buffer += line.as_str();
        } else {
            let line = format!("withdrawal,1,{},1.0\n", n);
            buffer += line.as_str();
        }
    }
    writeln!(tmpfile, "{}", buffer).unwrap();

    b.iter(|| {
        let rdr = csv::Reader::from_path(&path).unwrap();
        let _ = das_pay::sequential_serde(rdr);
    });

    drop(tmpfile);
    tmpdir.close().unwrap();
}
