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

# Day 16

This was an interesting one, and probably my messiest solution so far. Messy in terms of everything works fine, but it's not cleanly 
refactored, not particularly generic.

The first part is an implement the rules type problem, just stay calm, implement as described. So far so good.

The second bit, ended up with a bit of a debug session. It was a little too easy to use your first part (error sum per ticket) to filter
out invalid tickets - except it's not *quite* the same. Zeroes can be invalid values, so there can be tickets which are invalid, but have
a zero error sum. D'oh. It's quite instructive to go through the debugging process however, so not super frustrating.
It appears that some people on line were complaining that it wasn't obvious enough how to deduce the rule to field mappings. On the one hand, 
yes, it's true, it was necessary to decide on a strategy. This wasn't a 'implement algorithm X' problem. On the other hand - it makes it
interesting! And we are at day 16 now, so a bit of extended mental effort seems reasonable.

# Day 17

I wasn't expecting this one to come back, but it's nice to see - 3D game of life! Fully expecting a riot of visualizations for this.

* I attacked it in a reverse fashion to the seating problem. I heavily invested in a structure to represent to cube array and handle
flattening a 3/4D coordinate to/from a single index, and for accurately acquiring lists of neighbour cells. As a result it ran almost
perfectly first time, and very little time spent looking for errors today.

* Used an array first rather than a hashset/map. Not sure how significant a difference that is.

* Hard coded three dimensions. For part 2 added the fourth dimension, again hard coded. Was still able to run the part1 solution
as a slice through the 4d space. (My space isn't infinite, but I do presize intelligently so that edges aren't significant.)

* I *didn't* bother trying to code generic dimensions. (Although I don't think it would be significantly more difficult - mostly busy
work.) Spatial dimensions are one of those things which are almost 100% limited except for very special casees. Similarly, my neighbour
sets were via pre-computed arrays. Works fine but glad we're not pushing to five dimensions.

* Something I note - variable names. Particularly when in the weeds, more and more helpful to use desciptive variable names rather than
i,j,k or k,v etc.

* There *are* visualizations - but at the same time it's really hard to visual pleasantly. They're pleasant but quite hard to
interpret. <Sigh> Four dimensions.

* My release run for the month is now up to 2.2s thanks to today being ~1.2s. (Debug run is at 34s in total.)
** Update. Ok, I said I wouldn't optimize much, but had the idea of really aggressively only working on cells which can change. i.e. every
active cube, plus it's neighbours. It feels quite clunky (a lot of code to check we don't double count) but leads to a 10x speed up. Gives the
impression that a set based approach might be both significantly simpler and also performant.

* A Redditor pointed out that John Conway died this year, so it's nice to see something of a tribute here.

# Day 18

A pleasant day, although vastly helped by using high powered libraries (i.e. Pest for parsing) and a fairly restricted problem.

* Because I used Pest for parsing, I never really had problems with the bracketing. My parser always returned bracketed expressions wrapped
for me, so was able to handle with trivial recursion.

* Because of part 1, I did a simple left to right parser (except for brackets), rather than building a tree.

* For part 2, I ended up implementing a parse to reduce all the additions, and then handle the remaining multiplations. This was certainly
coding for the current problem, not coding for extension. I'd definitely be running into problems if I had many more operators, or if 
the operatore weren't associative, as they are here. I'm going to look into some of the background literature on the subject.

* I decided Rust was not a suitable language for some of the more _abusive_ techniques, live code evaluation, etc. I've seen some on Reddit now,
for example Ruby. I admire their unholy existence. (e.g. Redefine +, * to multiple and swap respectively. In you input, swap '+' and '*' symbols.
Use the standard Ruby parser to evaluate the expression. Laugh maniacally.)

* It's one of those paradoxes that this non-geometric problem can *arguably* produce more useful representations than something like yesterdays.
There's a particularly nice one that iteratively visually collapses terms to reduce to a single answer.

So, yes, felt like a gentle Friday, but know that's largely depending on the initial approach. Certainly some pits that I managed to step around.

# Day 19

This was an interesting one - it feels like parsing is a bit of a theme this year.

The first part was not so hard, given an entirely predictable parse. At least I don't have to consider kleene star type patterns I thought...

Then part two. I *really* need to bash through some more parsing texts (I've got one that comes at the start of a modern compiler design
book - but who wants to spend time wading through parsing methods when wanting to write a parser? I consider myself chastised and will get
back to it!)
Unfortunately, I quickly found that a greedy match would not work. (Matching greedily, I suddenly failed to validate any rules!) On the other 
hand I did discover that I only hit the looping rules once at most, as a result I could do a search trying the different varients of the
looping rules with too much cost (i.e. try rule 42 with one loop, two loops, ...). The result is not bllindingly fast, but still in milliseconds.
It's not general - but I don't particularly want to write a general regex parser in a single day.

Definitely one that points to more reading.

# Day 20

This was a cool one. I always enjoy something with a visual element. Two observations made this singificantly easier:

* I realised that edges could be represented as integer values. (i.e. the hash/dot patterns could be interpreted as binary.)

* The problem data was friendly. The matches were all unique, so it was completely deterministic on how to reconstruct the chart from
any starting point. So no backtracking trials, etc.

There was a lot of busy work, just pulling out the right keys for the different edges, then transformation rules when flipping/rotating. But
I think I kept it under control, and flexible enough to spit out relevant debug when needed. (I had an annoying bug around printing
the rows of a tile when in a particular orientation, which took me some time to track down.)

The monster hunting was again entirely deterministic, just a question of coding. (And counting carefully the monster coords!)

I was prepared for handling the various chart flips/rotations to hunt for the monsters, but then lucked out by find monsters in my
default orientation! (I'd settled in my mind that I'd do this most cleanly by shuffling/flipping my starting corner to get all the combinations
but it wasn't necessary in the end.)

So - a fairly long day, but a lot going on, and several stages, and interesting algorithms. Maybe my favourite so far.

# Day 21

Another fun one! It felt like it harked back to day 16 (at the end) where attempting to determine the allowable interpretation of 
some fields.
It's telling that this is one of those where *almost* everyone with the first star also has the second star. (It seems a lot of people
accidentally solved the second part while solving part one!)

In any case, it's a nice change of pace from the sea monsters. There's some tweaking to get it done efficiently, but nothing too bad. Looking
at the problems that people have had, there seems a reasonable hurdle in just understanding the problem properly. There is a small cognitive
gap I think (or at least, the description is complete, but there's a slight jump to make from the description to what you need to solve the
problem. I think it's quite subtle though.) Once you've got straight what the consequences of the problem are though, the actual implementation 
is straight forward.

# Day 22

The crab trials. This was fun, and I liked the narrative around it, becoming increasingly demented, adrift in the sea, locking wits
with a small crab.

Again, this day was mostly about keeping the rules straight in your head. Then the second part involves keeping it straight, through
multiple levels of recursion, and implementing it cleanly. I like this sort of problem in that it's challenging, but *dosn't* rely on
a mathematical insight. It's a good exercise in envisioning how the programme runs in a complex recursive manner.

One major performance I found was to use the default Rust hasher rather than building a string key for the game state each round. This was
a bit cheeky of me - I had an *suspicion* that creating the hash was pretty expensive, and runnning on the inner loop. Redditor's also 
commented on this being a major cost. Swtiching to the built in hasher gave me a ten-fold speed improvement, so good win.

# Day 23

The crab strikes back! I'm really enjoying the atmosphere on this raft. And the balance between the two parts was very nice too, a classic
case of part one being implementable in a straight forward fashion, but part two (although essentially the same) needs some real insights
into fixing raising performance by several orders of magnitude.

I had an inkling that this would be the case, but went ahead with a rough and ready solution. My reasoning was:

  a. It would get me into part 2 as quickly as possible, so I could start thinking on it.
  b. It would give me something to validate against.
  c. I don't know exactly what performance direction I'd have to optimize for.

My solution was maybe egregiously rough... reallocating the whole data set for every round. But, it's enough for part one.
The second part, I had a couple of false starts:

  * I had some idea that maybe the cup movements would stay fairly localized, so I'd be able to stick with a small part of the 
  game rather than trying to model all of the million cups. A quick mock up showed that to be false (and a proper reading of the
  example result should have warned me). Also, the memory allocation is still too pricey, I grind to a halt once the set I need to 
  consider hits a few tens of thousands anyway.
  
  * These ring type data structures start me thinking on linked lists, doubly linked lists, zippers, etc. I get towards implementing
  a doubly linked lists...
  
  * ... and then think of mashing it down to a handful of static data structures. (i.e. Essentially the same data, but handling the 
  storage myself.) A few arrays to track the next/last/value of each cell.
  
  * ... and then while sketching out how that would look on paper, realise I don't need the previous cell and can do it all in a simple
  array of integers. Implementation is clear from this point.
  
It all progresses neatly from there, with my existing implementation as guidance. I had a brief misstep at the very last when I read the 
values I needed as two the theh *left* of 1 - as a result it was extra complication and worried me by getting the wrong answer on the 
test data. It was brief though, re-read and fixed and matched against the test data. Then run against the full data and finished off that 
second star.

At that point my solution was running in 3 seconds, which is OK, but seems longer than ideal. I pull out the usual tricks to beef things up:

  * Use pre-allocated arrays, rather than vectors. This is nudging the limit that the program stack can handle - and annoyingly it's one of
  the places where Rust errors aren't so good. You just get a brief 'stock overflow' message.
  
  * ... on the other hand, Clippy has some very helpful suggestions on how to box the array and allocate it on the heap.
  
  * I pull out the hash set I was using to skip over the removed cups when finding where to insert. I don't actually know if this helps
  much, but it feels wasteful to use a hash set to check against exactly three elements.
  
  * I rip out the initial implementation of part one, and replace with my super charged version.
  
I'm now down to about 300ms for the full day, and can rest easy.

Incidentally, this has a very similar flavour to a puzzle back in 2018, which I was doing in Racket (I think?). It was a similar game involving
elves sitting in a ring, and had a similar second part where the complexity sky rockets. Being a fairly naive Racket programmer I did 
everything with immutable data structures at first and watched my CPU melt. It was a good learning experience though, and ended up
having to learn more about Racket, and learned about zipper data structures for the first time.

# Day 24

Bizarrely, I think this was actually my fastest to implement this year. I did spend some time thinking about the problem this morning, but
implementation was about half an hour.

  * The key observation relies on reducing the hexagon problem to a simple grid. Best way to think is pencil on paper - you *could* work out
  real x, y coordinates, with an annoying root three appearing because of the 60 degree angle.
  * You *could* deal with that by using float, and appropriate 'close enough' equality.
  * You *could* deal with that by storing your coordinates as an integer part, and a root three part. (Observing that you'll never be able
  to form an integer by adding root three parts, and vice versa.)
  * Or...
  * Observe that the hex grid is rectangular grid with a slightly skewed connectivity.
  
Once you realise the last it's pretty trivial. Essentially the east/west directions behave as normal (plus/minus on the x axis), while the
angled connections are slightly skewed. It doesn't matter which way - I put sw -> ne on the diagonal, and se -> nw on the vertical. Visually
this looks like just shoving rows of hexagons so they line up on a rectangular grid.

The previous problems have primed me well for game of life type problems, although I do try something slightly new for this one (I build a 
count of neghbours, rather than check the neighbours of each tile.) I rely heavily on hash sets/maps. I don't bother to optimise with 
arrays for this problem.

All clear quickly, and ready for tomorrow. I almost definitely won't have time to complete tomorrow on Christmas day, but look forward to 
having a think and getting it implemented over the holidays.

