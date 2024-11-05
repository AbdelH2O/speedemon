# Speedemon
Parallelized file downloader with a focus on speed and simplicity.

### Usage
```bash
Usage: speedemon [OPTIONS] --link <LINK>

Options:
  -l, --link <LINK>        The link to the file to download
  -o, --output <OUTPUT>    The output file. If not specified, the file will be saved in the current directory [default: .]
  -p, --threads <THREADS>  The number of threads to use for downloading [default: 4]
  -r, --retries <RETRIES>  The number of retries to use for downloading [default: 3]
  -t, --timeout <TIMEOUT>  The timeout for each request [default: 10]
  -h, --help               Print help
  -V, --version            Print version
```
