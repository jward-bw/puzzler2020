# puzzler2020
My solutions to the 2020 Christmas puzzler.

To build the binaries install rust and then run `cargo build --release` in this directory or any sub-directory. The binaries will be built to `target/release/[quick|short]`.

Both of my solutions are written in Rust using only the standard library.

## quick (src/bin/quick.rs)

This was my first solution to the puzzler.

My idea here was to encode all the edges of the graph by pattern matching on the characters.

I initially made some attempts at making a concurrent/multi-threaded solution, but found that a single-threaded solution was faster due to the overhead of spinning up threads. I haven't spent a great deal of time optimising this, but it runs pretty fast regardless and is quite readable.

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

### UPDATE: Faster!

Inspired by my changes to the short binary, I changed this code to only convert the first character of each word to lowercase, rather than the entire word. This sped-up the code by roughly 3x! (I hope the assumption that only the first character of a word is uppercase is valid.)

```bash
$ time target/release/quick | awk '{print length, $0}' | sort -nr | head -n 4
9 snowshoer
9 reconsole
9 horologer
9 gonyocele

real	0m0.035s
user	0m0.035s
sys	0m0.010s
```

## short (src/bin/short.rs)

I also wanted to try making a solution with as few characters as possible to rival Chris and JohnM's short python solutions. I knew it was very unlikely I'd be able to achieve the same levels of brevity as those solutions using Rust, so I started by working out the smallest possible encoding of the graph.

The two options were loading the data from file/stdin or having a short text encoding in the code. Due to the strong typing and safety in Rust, loading data from stdin/file requires unwrapping lots of Result types, so I resorted to finding a succinct textual representation of the graph.

The best way I found to represent the graph as a string of characters was as an Eulerian path (a trail which visits every edge exactly once). Unfortunately an Eulerian path is only possible on a graph which has exactly 0 or 2 vertices with an odd degree, and our graph has 10 vertices with an odd degree. To fix this we can add redundant edges until we have a graph with only 2 vertices with an odd degree. [There is a nice algorithm](https://en.wikipedia.org/wiki/Eulerian_path#Hierholzer's_algorithm) to find the final Eulerian path by joining Eulerian circuits of subgraphs, which gave this solution: `rownsholrelegontoecoshgcrbyntybltboyswgwhc`. The rest of the code is pretty straightforward; the code searches the path string for each 2 character pair in each dictionary word (and each pair may need to be reversed, since the graph encoding isn't guaranteed to have both directions for each edge).

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

### UPDATE: Faster and shorter.

I found that by converting the graph into directional graph, doubling the edges and reversing the duplicated edges, the resulting Eulerian circuit is almost twice as long as the original graph encoding, but I can reduce the total character count of the code. I can remove the code to reverse a character pair if it isn't found initially in the path, which also gives a speedup of around ~25%.

```bash
$ cat src/bin/short.rs | wc -c
344
$ time target/release/short | awk '{print length, $0}' | sort -nr | head -n 4
9 snowshoer
9 reconsole
9 horologer
9 gonyocele

real	0m0.148s
user	0m0.148s
sys	0m0.011s
```

### UPDATE: Even fasterer and shorterer.

By converting the dictionary word into a slice of u8s rather than converting into an iterator of chars anc collecting into a Vec, we can shave off about 100ms (was taking less than 50ms). Unfortunately I also realised that this solution didn't consider dictionary words which had uppercase characters, so in fixing that I have slowed down the code and used more characters. This solution is at least 1 character longer than it needs to be. It takes one more character to transform the pairs of characters to lowercase than the entire word, but since the majority of words in the dictionary are not valid words in the graph only performing the operation on pairs represents a significant speedup (~30ms):

```bash
$ cat src/bin/short.rs | wc -c
342
$ time target/release/short | awk '{print length, $0}' | sort -nr | head -n 4
9 snowshoer
9 reconsole
9 horologer
9 gonyocele

real	0m0.090s
user	0m0.090s
sys	0m0.011s
```

### Final update: Longer and (slightly) slower 😢

Unfortunately my code was incorrectly allowing all single character words. I couldn't think of a good solution to integrate this into my current method, so I've appended it as a separate check. This has added quite a few characters, but not increased the runtime by much. There's probably a nice way to do this, but I have run out of time to work on it!

```bash 
$ cat src/bin/short.rs | wc -c
395
$ time target/release/short | awk '{print length, $0}' | sort -nr | head -n 4
9 snowshoer
9 reconsole
9 horologer
9 gonyocele

real	0m0.092s
user	0m0.092s
sys	0m0.009s
```


