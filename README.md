# bytomancer-aoc-2024
My solution repository for Advent of Code 2024.

## Spoiler Warning
Within the `src/solutions` and `lua_src/solutions` folders,
you will see my code to solve Advent of Code 2024.

If you wish to remain spoiler free,
avoid these folders!

## Objective
I've decided on a few objectives this year for Advent of Code.
- As I have done all years, I will not use any AI tools.
- I still wish to solve each problem as fast as I can using Rust.
- I'd like to return to each problem and solve them in Fennel.

I also see no value for myself in using AI to complete the problems;
I'm using Advent of Code to learn and improve.

I was debating about not doing Rust,
but I wanted to feel okay going to sleep at 1AM after making some submission,
and I didn't believe it was likely if I attempted Fennel first.

Fennel will be my first real introduction to lisps.
Obviously there are countless lisp dialects out there,
but it was extremely easy to incorporate Fennel into my existing tooling.
Additionally, Fennel made the most sense to me,
as Lua appears quite often in game development,
which is where I tend to focus much of my personal project time.

## AOC Framework

This project began in November 2022,
as I worked on solving the AOC 2021 problems.
From my time tinkering with these problems,
I decided to add on a few features to ease development
(and for the simple fun of it).

### Features

1. I've implemented an input downloader which retrieves input files via the web.
    - A `.env` file is required with `SESSION=<Session ID from your cookie>`.
    - Files are downloaded to a `_cache/` folder created in the project root.
    - If an input file is already found locally, that file is loaded instead.
2. Final submissions are sent automatically to the form.
    - Using the same `.env` as above,
      executing the program with the `-s` or `--submit`
      option will send the result to the website's submission URL.
    - The resulting page is scanned and outputs a result to the command line.
3. Arguments dictate the solution to be run.
    - After discovering significant re-use between the days,
      I decided to package my code together in a single project.
    - Execution is performed with `cargo run -- dXsY`,
      representing day X solution Y.
4. Colorization is used heavily.
    - Tracking outputs and debugging is much simpler,
      thanks to the `colored` crate.
5. Automatic generation of solution files.
    - Usage: `cargo run -- d01s1 -g`
    - Executing the program with the `-g` or `--generate`
      option will perform some metaprogramming,
      generating new rust files.
    - A special outcome is performed if the target name ends in `lua`.
      A .fnl file will be generated, and the lua template will be used instead.
      This enables a rust framework driven system which feeds the input to lua,
      then runs your solution, written in fennel.
      This special fennel case was snuck in so I could do 2024 in fennel easily.

## Personal Times
```
      -------Part 1--------   --------Part 2--------
Day       Time  Rank  Score       Time   Rank  Score
 16   00:52:00  3391      0   02:39:43   4123      0
 15   01:18:22  5807      0   13:08:53  14797      0
 14   00:54:46  5727      0   01:36:41   4711      0
 13   00:52:54  5491      0   22:49:08  28845      0
 12   01:31:26  8203      0   01:47:00   3845      0
 11   00:16:07  4396      0   01:00:31   4802      0
 10   00:17:09  2445      0   00:18:39   1876      0
  9   00:40:37  5198      0   01:04:51   2855      0
  8   00:47:12  6152      0   00:57:34   5474      0
  7   00:42:47  6817      0   00:47:07   5623      0
  6   00:20:30  3733      0   00:37:46   2283      0
  5   00:34:50  7351      0   01:04:52   7453      0
  4   00:16:35  3228      0   00:26:58   2885      0
  3   00:14:04  6042      0   00:19:01   3344      0
  2   00:12:13  3814      0   00:20:09   2834      0
  1   00:08:06  3540      0   00:15:01   4195      0
```
