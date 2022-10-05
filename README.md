# das-pay

People's payment engine CLI.

## Get started

Clone the repository:

```bash,ignore
git clone git@github.com:yoguut/das-pay.git
```

Since this repo tracks large files as pointers to a lfs bucket, lfs needs to be pulled separately as followed:

```bash,ignore
git lfs pull
```

Next, run the following terminal commands in the repo directory:

```bash,ignore
cargo r -- sample_input.csv > sample_output.csv
```

The above command will output a summary of accounts based on the recent transactions within `sample_input`.
If you have a different input file you would like to use, simply point the input csv to your desired file
path.

## License

GPL 3.0
