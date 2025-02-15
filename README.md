# A Fast File Finder in Rust 

![Course banner](assets/banner.png)

## Introduction

One of the things developers need to do every day is to find files on their system. This can be a cumbersome task, especially when you have to search through a large number of files. In this workshop, we will build a fast file finder in Rust that will help you search for files on your system quickly and efficiently.

Our file finder (ff) will recursively search through directories and display the files that match the search criteria. It supports searching by regular expressions. You can also add additional features like searching by file type, limiting the search depth, or excluding certain directories.

## Who's the Target Audience?

Target audience: Rust beginners/intermediate developers interested in building a small real-world CLI tool with Rust.

### Necessary Tools

- [rust](https://www.rust-lang.org/tools/install)
- [git](https://git-scm.com/)

## Structure

Use the [slides](./slides.pdf) to go through the exercises.

Use `src/main.rs` to start writing your code.
If you get stuck, check out the [examples](/examples) folder, which contains working source code for each block. You can run the `block1` example using the following command:

```bash
cargo run --example block1
```

We recommend to try it yourself first and only refer to the example code in case you run into issues.

## Blocks

Here are the individual blocks of work that we will cover:

- Block 0 - Check Rust Installation and Version
- Block 1 - Iterating over Directories and Files 
- Block 2 - Error Handling 
- Block 3 - Command-line Arguments 
- Block 4 - Regular Expressions
- Block 5 - Iterators and Code Structure
- Block 6 - Performance Optimization (try it in a large directory like `/home`)
- Block 7 - Bring your own features!

## Extra Features

Pick whatever you like from the list below or come up with your own ideas:

- Fix all clippy warnings
- Search by file type (e.g., only search for `.rs` files)
- Add color to the output
- Benchmark your file finder against `find` or `fd`
- Query the file system asynchronously
- Visualize the search results in a tree-like structure
- Sort the search results by size, date, or name
- Add tests and documentation
- "Smart Replace" - Find and rename files with regex capture groups:
  ff "test_(.+)\.py" --rename "{}_{datetime}_{1}.py"
- Xargs mode: 
  ff "\.json$" --chain "jq . {} | grep error"

## Show And Tell!

We are curious to see what you have built. If you want to share your shell with
us, please send us a link to your repository. We will add it to the list below.

We'd be happy to hear your answers to the following questions:

- What did you learn?
- What was easy?
- What was hard?
- Would you do anything differently?
- What would you like to learn next?

## Closing Words

If you enjoyed this workshop, please share it with your friends and colleagues.
It would greatly help us if you could tweet/toot about it or share it on
[Reddit](https://www.reddit.com/r/rust/) or [LinkedIn](https://www.linkedin.com/).
Thanks!

You might also want to [subscribe to our newsletter](https://corrode.dev/blog/) for
future workshops and other Rust content.

If you are looking for professional Rust training, please get in touch with us
at [corrode.dev](https://corrode.dev/).
