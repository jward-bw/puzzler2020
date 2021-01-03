# puzzler2020
My solutions to the 2020 Christmas puzzler.

To build the binaries install rust and then run `cargo build --release` in this directory or any sub-directory.
The binaries will be built to `target/release/[quick|short]`.

Both of my solutions are written in Rust using only the standard library.

## quick (src/bin/quick.rs)

This was my first solution to the puzzler.

My idea here was to encode all the edges of the graph by pattern matching on the characters.

I initially made some attempts at making a concurrent/multi-threaded solution, but found that a single-threaded
solution was faster due to the overhead of spinning up threads. I haven't spent a great deal of time optimising
this, but it runs pretty fast regardless and is quite readable.

```bash
$ time target/release/quick | awk '{print length, $0}' | sort -nr | head -n 4
9 snowshoer
9 reconsole
9 horologer
9 gonyocele

real	0m0.107s
user	0m0.104s
sys	0m0.013s
```

## short (src/bin/short.rs)

I also wanted to try making a solution with as few characters as possible to rival Chris and JohnM's short
python solutions. I knew it was very unlikely I'd be able to achieve the same levels of brevity as those
solutions using Rust, so I started by working out the smallest possible encoding of the graph.

The two options were loading the data from file/stdin or having a short text encoding in the code. Due to the
strong typing and safety in Rust, loading data from stdin/file requires unwrapping lots of Result types, so I
resorted to finding a succinct textual representation of the graph.

The best way I found to represent the graph as a string of characters was as an Eulerian path (a trail which
visits every edge exactly once). Unfortunately an Eulerian path is only possible on a graph which has exactly
0 or 2 vertices with an odd degree, and our graph has 10 vertices with an odd degree. To fix this we can add
redundant edges until we have a graph with only 2 vertices with an odd degree:

Original graph:
![image](https://user-images.githubusercontent.com/53442247/103480118-3ef66600-4dca-11eb-95c3-267b72844e49.png)

Augmented graph:
![image](https://user-images.githubusercontent.com/53442247/103480101-29813c00-4dca-11eb-82eb-1a87434d5654.png)

[There is a nice algorithm](https://en.wikipedia.org/wiki/Eulerian_path#Hierholzer's_algorithm) to find the final Eulerian path by joining Eulerian circuits of subgraphs, which gave
this solution: `rownsholrelegontoecoshgcrbyntybltboyswgwhc`. The rest of the code is pretty straightforward;
the code searches the path string for each 2 character pair in each dictionary word (and each pair may need to
be reversed, since the graph encoding isn't guaranteed to have both directions for each edge).

The total number of characters in the source file is 432:
```bash
$ cat src/bin/short.rs | wc -c
432
```

And the speed isn't too bad either:
```bash
$ time target/release/short | awk '{print length, $0}' | sort -nr | head -n 4
9 snowshoer
9 reconsole
9 horologer
9 gonyocele

real	0m0.227s
user	0m0.209s
sys	0m0.022s
```
