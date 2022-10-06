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

> WARNING: If you choose to ignore this guide and forget to pull LFS assets, you can run into serialization error.

Next, run the following terminal commands in the repo directory:

```bash,ignore
cargo r -- sample_input.csv > sample_output.csv
```

The above command will output a summary of accounts based on the recent transactions within `sample_input`.
If you have a different input file you would like to use, simply point the input csv to your desired file
path.

## Journal

The Journal is a `mdbook` on its own under the `journal` subdirectory. It's there to keep a record of my current progress, so I can get more contexts on whatever topics I am researching before passing out.

## License

GPL 3.0
