# fcnt

![Crates.io](https://img.shields.io/crates/v/fcnt?color=9cf)
![Crates.io](https://img.shields.io/crates/d/fcnt?color=green&label=install)

**fcnt** is a file counter used in command line.

It can quickly count the number and size of huge amount of files in multiple directories through multi-threading.

## Usage

```shell
$ fcnt [OPTIONS] [DIRECTORIES]...
```

- Arguments:

  [DIRECTORIES]...  the directories (default: ./)

- Options:

  ```
  -a                 Count all regular and hidden files
  -r <PATTERN>       Match entries using regex (only matche filenames)
  -o <ORDER_BY>      The value to sort the results by [possible values: name, n, file, f, dir, d, size, s]
  -s                 Count the total size of files
  -t <THREAD_NUM>    The number of threads for traversal (invalid in NON_RECURSIVE mode)
  -v                 Verbose mode, open this option will display the found entries
  -R                 Non-recursive mode (files in sub-directories will be ignored)
  -h, --help         Print help
  -V, --version      Print version
  ```

## Example

By default, the results will be sorted by file count in descending order.

```shell
fcnt ./Pictures ./Music ./src/package
Path           Files  Dirs
./src/package   8070  3120
./Pictures      7799   274
./Music         3455  1196
──────────────────────────
Total          19324  4590
```

If the `-s` option is specified, the sort column will be change to `Size`.

```shell
$ fcnt -s ./Pictures ./Music ./src/package
Path           Files  Dirs   Size
./Music         3455  1196    21G
./Pictures      7799   274  17.5G
./src/package   8070  3120     4G
─────────────────────────────────
Total          19324  4590  42.6G
```
