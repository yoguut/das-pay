# das-pay

People's payment engine CLI.

## Get started

First, make sure to use a rustc version higher than `1.31` since it's
required by `serde`'s `derive` feature.

Next, clone the repository:

```bash,ignore
git clone git@github.com:yoguut/das-pay.git
```

Next, since this repo tracks large files as pointers to a lfs bucket,
lfs needs to be pulled separately as followed:

```bash,ignore
git lfs pull
```

> WARNING: If you forget to pull LFS assets, you can run into
> serialization error.

Next, run the following terminal commands in the repo directory:

```bash,ignore
cargo r -- sample_input.csv > sample_output.csv
```

The above command will output a summary of accounts based on the recent
transactions within `sample_input`.

If you have a different input file you would like to use, simply point
the input csv to your desired file path.

## Contributing

> I have not tested the development setup on `vscode`, so take the settings
> and config files in `.vscode` with a grain of salt.

Install cargo-watch if not present: `cargo install cargo-watch`
Install mdbook if not present: `cargo install mdbook`

To get things started, here is the recommended setup (not required):

```bash,ignore
cargo watch -x check -x test
```

If any errors show up from the last cmd, please open an issue to report to the
maintainer.

## Journal

The Journal is a `mdbook` under the `journal` subdirectory.
I need it to keep contexts on whatever topics I am researching before.

> Not relevant for interview.

## License

GPL 3.0
