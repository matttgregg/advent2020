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

Disclaimer - not a professional Rust programmer, so often I'll try things a few ways, hack code about just to see how things look. Code may well look weird
(which I'd be happy to receive pointers on!), and no guarantees on the qualirt of commits.

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

# Day 9

A nice problem here, in some ways felt like an extension of the ideas introduced in day 1 (find pairs that sum to a target), and ratcheted
it up enough that efficient solutions are worthwhile.

Ended up using a hash set look up for totals again, and otherwise rolling window/pointer approaches. With my crude timings this looks to 
be one of the most efficient days (~300us on my laptop).

* After completing and looking at other peoples solutions, I notice there's often a trade off between efficiency and expressivity. Some (in several
different languages) express the problem well trading off some performance. My solution errs a little more on efficiency. As always, best solutions are
going to depend on context.

* Feels like we didn't really get to the bottom of hacking the flight computer and XMAS code today. Wouldn't be surprised to see more tomorrow.

* Who's instinct, on first getting onto the plane, is to hack into the flight computer? I'm a little creeped out that we didn't even have a slight
motivation for doing this. :)

# Day 10

:D Pleasantly uneven parts 1 and 2. Part 1 - sort and count the gap sizes. Part 2...
(Probably not just me - looking at the official AoC stats page this day has a significant number of people with just part 1 complete, a much larger
proportion than on previous days.)

Neither need extremely esoteric solutions. Part 2 falls quickly to some dynamic programming, and well known tricks to avoid wasteful calculations. The 
interesting part is that without some sort of smart ass-ery to improve efficiency, you're likely looking at your computer melting before getting anywhere.
(Previous days have been much more lenient on inefficient solutions.) Done cleanly however, my runtime comes in happily under 1 ms.

Also - fewer problems with the borrow checker, getting a much stronger intuition for working with the language, what's idiomatic, how to read it.

Looking around at some other solutions:

* Analytical solutions. I need to get my head around these.

* Tricks to sub-divide the problem. This is really interesting - visualizations show that the problem does easily sub-divide (and easy enough to see why), which
allows a more straight solution. The interesting thing is that the dynamic programming approach with blind memo-ization takes advantage of this *even without
explicitly coding it*, which is cool. Generally however, I do agree that getting a 'feel' for a problem is a really important, besides just throwing a generalised
algorithm at it.

When I have time, I might remove the recursion as an exercise. It's always a good thing to do for practice.

# Day 11

Interesting this one, largely on how to do all this efficiently, and concisely. 
The extension for part 2 requiring non-adjacent 'neighbours' (where I mean a neighbour to mean the cells which affect the current cell) is 
interesting and important to keep code clear about.
With any sort of problem like this, I try to follow my own advice:

* Make the code clear. (Descriptive function names, descriptive variable names.)
* Make the code modular and testable in pieces. It was extremely useful to be able to validate the 'neighbour' generation outside the whole 
routine.

I'll admit I haven't been as clean as I might be in production, particularly following refactoring. I've mainly used 'testable' here to mean
that it's easy for me to put in debug statements, not that I've fully broken functions down to single responsibilities, but it's enough for
these purposes.

Following this, I got to a working solution (and claimed my starts) with some abysmal performance, and using hashes to look values up. It took a 
few seconds to run. It's the first time I wrote a test which seemed to expensive to run automatically. However, rust gives you enough fine control
to optimize - by flattening down to arrays, brutally reducing the amount of redundant work being done, squeezed it down to ~20ms. Still my slowest
day yet, but in the right order of magnitude. There's probably more to be done (tighter representation - there's very little really needed) but
I think I've reached my limit in the context of this problem.

Unsurprisingly, lots of interesting visualizations today. The way the problem settles is quite interesting. I plan to attempt a few visualizations 
at some later point, and this one is a good candidate.

# Day 12

Is this the slightly easier weekend problem? Computationally not too hard, although we do have a bit of state to worry about (e.g. the ferry has 
a location *and* a heading.)

* A few things make this easier - particularly realising that the only valid turns are quarter, half, three-quarter turns, so immediately 
simplifies how turns work.

* Thankfully, have fairly good geometric intuition (I'm a visual person!) and so didn't find it do hard to code up the various movements, even
when the waypoint movements were introduced.

* Computationally, not many tricks. Read instruction, update state, repeat. No real complexity problems to worry about.

* Gave a chance to play with traits for a change. In this case I wrote two implementations for a `Navigable` trait. This allowed me to write once
and forget the instruction parsing code. As always the code ends up verbose, but eloquent. Thinking if I was looking for a more battle-hardened 
implementation it's quite easy to layer on the Rust error handling with `Result` return values. 

* Again, a day which naturally leads itself to visualization. Probably less interesting than yesterdays (the route is exactly as described
in the puzzle input - no surprises), but still impressed by the inventiveness of a lot of the Reddit coders.

# Day 13

Another day with a huge jump between parts one and two! Looking at the stats at the moment, I see about 14,000 have both stars, but 11,000 (i.e. 
almost as many again!) only have the first star.

Feeling fairly happy to have implemented both parts fairly quickly and without googling anything. I thought it might be instructive to 
go through my thought processes:

* Try brute force, just to get something down and check I understand the solution. Watch it get stuck for a few seconds and start to think.

* Do any generic programming methods help here? Memoization? Dynamic programming? Not really - I'm not being wasteful in recomputing things, or
computing things that I *obviously* don't need. Observation: an insight is needed to dramatically reduce what's being considered.

* ... and I really do mean dramatic. My first bus is 13, so I'm only bothering to check multiples of 13 as my start point. (My answer time *must*
be when that bus leaves.) I've got a bigger bus at 439 - I could be checking evering 439 instead, if I work relative to that bus... but 
that's only ~40 times faster.

* ... but... what if I was jumping and considering 13 *and* 439 at the same time? What would that look like?

* Pick a tiny example to get a feeling for the problem. Bus3, the Bus5. I see an answer at t=9. (9 = 3*3, 10 = 2*5). Then ... think ...
the next one is at 24 (24 = 8 * 3, 25 = 5 * 5). And then.. notice that the jump between these solutions is 15. And 15 = 3 * 5.

* Trick. I do *not* bother to prove this. It *feels* right, and I know I can prove it later.

* So I now feel towards a speculative algorithm. I start with a solution at 0, and jumping through by 13. If I find I align with a second bus, say
439, then I know that I'll find another solution at (439 * 13), and following increments.

* This is an algorithm that I can program! I have a niggle - is this going to be the *smallest* such? If my two bus numbers have a common
factor, what will the consequence of that be. My maths intuition says things are going to be a bit different. 

* I ignore these niggles and program it anyway! And I get the right answer.

Working back afterwards we see that, yes, indeed co-primality is needed for this to work. But, it turns out in all inputs for AoC the bus
numbers are co-prime. And, yes, the formal name for this is the chinese remainder theorem. 

It's worth noting that often in these coding problems, there's a piece of meta information - the problem is soluble, the solution is achievable
but a relatively bright person, the solution runs quickly. This gives some guidance on the sort of things that are worth trying. Also, it's 
worth building in stages - I always knew I *might* have to add some special behaviour if the buses weren't co-prime,  but the simpler version
is easier to program, and a natural waypoint.

Finally, I attended, many years ago, a lecture on investigative number theory. The lecturer was enlightening on how he played with systems, tried
things out, and saw what happened. I think often people don't see this investigative nature of advanced mathematics. I think that the lecturer
mainly used Mathematica, obviously taking advantage of the expressive language, and the interactive environment. This is one aspect that I have 
been missing in Rust. The language of Rust does mean you do need some level of confidence to see *through* the syntax into the mathematics at
times. The interactive environment is hard to replace, but in practice we end up trying to work around with tests, flexible input, mini-DSLs, and 
a fast(ish) compile/run cycle.

# Day 14

Surprisingly I don't have much to say about todays! I did enjoy it, but it felt like a stay calm, implement whats been described, and everything
will be fine. No major tricks to the implementation.

* I would say it's probably one of the harder puzzle descriptions to understand. It's not the most complicated procedure needing to be 
modelled, but it is complicated to explain in plain english. You could describe it more mathematically I guess, at the risk of completely
bemusing those without a maths-y background, making the solution more obvious, and less interesting. Generally I enjoy the flavour text, and
think Eric does a great job of describing complicated problems in an interesting way. (I'm not a speed coder, so certainly don't begrudge
the minutes spent reading the problem description - it's a valuable skill anyway!)

* Probably one of those days when I feel my lack of understanding the std toolkit of my language more completely. I could likely be a 
lot cleaner in my code if I used idiomatic bit operations. I *have* since seen some examples on Reddit using mask fields in interesting
ways (a set/reset and floating field), but I'm not going to change my code at this point. (There aren't significant performance gains,
and I don't think the existing code is too ugly.)

# Day 15

An interesting day this one, as depending on the implementation this is either fairly easy or frustratingly hard (especially part 2).
I wouldn't be surprised if this is one of those that sees a divide between regular coders and less experienced coders. For my part, I
automatically reached for a hash to store my *last seen* information. Instincts say I am *not* going to want to walk back through the 
list every time to find my last usage. Again - don't lose your head on the fine details. But, implemented this way you find that both parts
complete in reasonable time with no changes. (Approx. 3 seconds for part 2 on my test.)
I'd already coded the limit of the sequence of the parameter, so the hardest change for part 2 was making sure that I counted the zeroes correctly!

Out of interest tried variations using an array rather than a hash set. (Got me down to ~1s for the expected memory cost.) And then using unsigned
32 bit ints wherever possible. (Improved my memory, down to reliably sub 1s.) But not much beyond that - no significant order of magnitude 
improvements, which is as expected. Numberphile do have a video on the sequence (labelled the *Don't Know* sequence) which I'll look into later.
I'm sure they've already had plenty of hits today!




