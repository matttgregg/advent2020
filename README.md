# Advent of Code 2020 in Rust as a learning exercise.

First - go try [Advent of Code](https://adventofcode.com) if you haven't already. It's an enjoyable set of computing problems, easy to interact with,
usually interesting and with an enjoyably kooky narrative tying it all to Christmas.

**BEWARE SPOILERS FOR AOC2020** - It's partly the nature of the beast, but I'll try to avoid anything too explicit.

## Background

Professional developer, currently mostly Go as a day job. A bit of a language magpie, and been back into reading the Rust book this autumn. The original
plan was to try rust for a November game jam, and toyed with Rust given the strong wasm support. (Everything's better in a browser, right?) Given almost
Rust from scratch, almost no experience of game dev, and very little idea for a game, November passed with almost no code written, however...

December was coming. I've done some Advent of code for the past couple of years (Racket and Haskell most recently),
so I quickly finished off the last few chapters of the Rust book (slight skimming, but hopefully got enough) and ready to 
get going.

## Day 1

A gentle start, and one that I've seen come up previously in coding challenges. Nothing too complicated, and good warmup to setting up my rust environment, 
getting to grips with basic syntax and the cargo build system.

## Day 2

The first day with non-trivial parsing. Also good to see part two requiring a re-interpretation of data (a few AoC puzzles are structured like this.)
I opt to use Regex first of all, becuase who doesn't love regex? Have some fun getting into the way vectors and hash maps work in rust. I have to say
that I'm pleasantly surprised by how complete the rust standard libraries are - often I'll think of a function I need and it will already be implemented.

* I play with some basic bench marking on my days of data.
* I think about parsing, because I know it will be useful on future days. I look at Nom (I've enjoyed parser combinators in F# and Haskell, the notation
    seems very clean.), but for some reason I decide against Nom. Possibly it's the Rust syntax, possibly it's my lack of familiarity with the rust syntax
    but I decide to try...
* Pest. I think the 'elegant' tagline drew me in. Downsides - it *is* a separate grammar file in a separate format. It does need that slightly tedious
    walking through generic tree strustures, but I guess there's no avoiding that. Positives - the format is very clean, the rust integration works well.
* I find a few hundred times speed up from changing from Regex to Pest for my parsing. I realise that I made the rooky error of not pre-compiling my
    regex expressions. Regardless, I'm not running around a few milliseconds. That feels like a decent speed to aim for over the cmoing days, will be 
    interesting to see when it's no longer possible. 

## Day 3

Good clean fun in the snow. If you're comfortable with modular arithmetic this will be fine. Slight interest in tweaking for the part 2, high speed runs, but
mostly just a matter of organizing the code.
Good to see the Reddit groups going to town on visualizing this one!

## Day 4

This one... those passport rules. This felt the most pernickety so far. Given I already had Pest examples running, I used it again for parsing this. The first
part went quickly, because I quickly decided... *I didn't need to keep all this data*. I'm just counting up the fields, checking if I've got all of the ones
required and then I can forget about it and move to the next. This does mean I can't create the amazing visualizations that some people are handling, but <shrug>.
(I might try more visualization later, I really admire those that create them.)

Anyway, part 2 gets interesting:

* A lot of those rules, can be handled in the parser naturally. Digit counts, trailing units, etc.. **Except** I don't want to fail to parse the file if I
    encounter a bad value. So my parser ends up recognizing both good and bad data alternatives. I'm not sure how I feel about this being in the parser, and
    if that logic should live more in the rust code.
* On the bright side, my parser immediately crashes on a `pid` with too many digits, tipping me off on the sort of things to watch for. The extra digits seems
    to be a common trip point on the message boards, so really glad to have caught that early.
* Of course I still need to do the numeric bounds checking in code.

## Day 5

Truly the only day where I looked at the puzzle while still in bed, though 'hmmm, I can see what this is', got out of bed, did the puzzle, went back to bed.
*cough* binary representation *cough*
I mean, it still takes me about half an hour, because I'm being fairly leisurely, but this one felt clean and direct.

## Day 6

Oh, this was quite fun when it came down to it. Simple parsing, build sets, look at intersections and unions. Like a lot of the early days it's a question
of unpacking the description and seeing the problem underneath.

For fun, decided to re-implement using binary representations of u32 for my question answers (instead of hash sets), and reduce intersections and unions
to OR and AND operations. Worked cleanly without much effort. Learnt about handling binary in Rust. Again, impressed to find the standard library already
has a `count_ones` function on integers. Modest performance improvement. (I think roughly halved the runtime.)

## Day 7

Ah, things getting a bit more interesting. Getting into deeply nested data structures. In terms of the narrative, literally deeply nested objects.
Observations for the day:

* I used Pest again for my parsing, and wasted a fair bit of time tweaking my parser rules due to misunderstanding how whitespace is handled. Despite the
    I realised fairly quickly that *the data format is much nicer than it seems*. Although it looks like natural language it's actually very regular. Even the 
    bag descriptions are *always* two words - it feels like AoC is being kind to us here - I guess it is still early. No evil gotchas lurking, no loops in the
    luggage, no oddly formatted rules.
    
* I *didn't* use recursion, although in some ways it's a natural fit. I've been spoilt with the deep and expressive ways in which Haskell handles these
    types of structures (check out [recursion schemes](https://blog.sumtypeofway.com/posts/introduction-to-recursion-schemes.html), I found it enlightening. Partly
    I was also unsure about how Rust would handle recursions (probably well), and I was always being reasonably performance minded this year.
    
* Not using resursion allowed me to concentrate on the algorithm directly, and have some clarity on the performance characteristics. The method I used on
    part two I particularly liked (repeated transformations of the bags by unpacking), and was fairly efficient with the data given.
    
* Some fighting with the borrow checker, something I've avoided largely so far. But now I'm both iterating over and manipulating non-trivial data, so did have
    a brief period of mexican stand off with the borrow checker. Fixed by pushing things around like chasing bubbles under wall paper, until it was all happy
    but it wasn't very satisfactory. Had a bit of [a read and a google]() later on on these patterns, some discussions on the Discord group, and now feel I understand
    a bit more.
   
* Performance bouncing around the 5ms mark today. (Past few days I've been getting under 1ms). The whole set of days still runs almost instantly on my laptop. This
    is naturally a bit more complex. I've already spent some time improving performance from ~35ms, mainly by being smarter about memoizing data and making
    full use of the loops I do create. I can't get too excited about wringing more performance from this.
    
# Day 8

Happy to see a VM type challenge come up. Nicely designed, the problem (at least the second part) is not purely implementing the VM. The VM itself is quite 
clear (so far...). Regular, human readable instruction names. Everything has similar numbers of arguments.

* Again, I went in with a Pest parser at first, definitely overkill here. I rewrote later to just match the strings directly (no real need even for 
regex) and was pleasantly surprised to see it made almost no difference to performance. The generated pest parsers seem to be fairly efficient.

* More learning and understanding of rust, playing with loops and style. The interesting thing that caught me was converting between i64 and usize, and 
wrap around. I got around it early due to a. running in release mode (no overflow checks), and b. usize = u64 on my architecture. I was grateful to be runnning
clippy in pedantic mode, so went the extra mile to remove my conversions properly. Interesting to handle them in a watertight fashion.

* Performance again bouncing around the 1ms mark. Re-implemented part 2 to use a back tracking approach rather than start from scratch, and shaved maybe a
third off that. Still not hitting problems where performance is a critical bottle neck. 

* Everyone seems in agreement that the difficulty curve is slowly rising, so looking forward to an interesting week ahead.
