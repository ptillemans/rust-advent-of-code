Day 2 - Advent of Code 2018window.addEventListener('click', function(e,s,r){if(e.target.nodeName==='CODE'&&e.detail===3){s=window.getSelection();s.removeAllRanges();r=document.createRange();r.selectNodeContents(e.target);s.addRange(r);}});

[Advent of Code](/)
==========

* [[About]](/2018/about)
* [[Events]](/2018/events)
* [[Shop]](https://teespring.com/stores/advent-of-code)
* [[Settings]](/2018/settings)
* [[Log Out]](/2018/auth/logout)

@snamellit 2\*

{:year [2018](/2018)}
==========

* [[Calendar]](/2018)
* [[AoC++]](/2018/support)
* [[Sponsors]](/2018/sponsors)
* [[Leaderboard]](/2018/leaderboard)
* [[Stats]](/2018/stats)

Our [sponsors](/2018/sponsors) help make Advent of Code possible:

[Alfie by Prodo](https://aoc.prodo.ai/) - a more immediate, feedback-driven coding experience. Try our online JavaScript playground with Advent of Code!

\--- Day 2: Inventory Management System ---
----------

You stop falling through time, catch your breath, and check the screen on the device. "Destination reached. Current Year: 1518. Current Location: North Pole Utility Closet 83N10." You made it! Now, to find those anomalies.

Outside the utility closet, you hear footsteps and a voice. "...I'm not sure either. But now that so many people have chimneys, maybe he could sneak in that way?" Another voice responds, "Actually, we've been working on a new kind of *suit* that would let him fit through tight spaces like that. But, I heard that a few days ago, they lost the prototype fabric, the design plans, everything! Nobody on the team can even seem to remember important details of the project!"

"Wouldn't they have had enough fabric to fill several boxes in the warehouse? They'd be stored together, so the box IDs should be similar. Too bad it would take forever to search the warehouse for *two similar box IDs*..." They walk too far away to hear any more.

Late at night, you sneak to the warehouse - who knows what kinds of paradoxes you could cause if you were discovered - and use your fancy wrist device to quickly scan every box and produce a list of the likely candidates (your puzzle input).

To make sure you didn't miss any, you scan the likely candidate boxes again, counting the number that have an ID containing *exactly two of any letter* and then separately counting those with *exactly three of any letter*. You can multiply those two counts together to get a rudimentary [checksum](https://en.wikipedia.org/wiki/Checksum) and compare it to what your device predicts.

For example, if you see the following box IDs:

* `abcdef` contains no letters that appear exactly two or three times.
* `bababc` contains two `a` and three `b`, so it counts for both.
* `abbcde` contains two `b`, but no letter appears exactly three times.
* `abcccd` contains three `c`, but no letter appears exactly two times.
* `aabcdd` contains two `a` and two `d`, but it only counts once.
* `abcdee` contains two `e`.
* `ababab` contains three `a` and three `b`, but it only counts once.

Of these box IDs, four of them contain a letter which appears exactly twice, and three of them contain a letter which appears exactly three times. Multiplying these together produces a checksum of `4 * 3 = 12`.

*What is the checksum* for your list of box IDs?

To begin, [get your puzzle input](2/input).

Answer:

You can also [Shareon [Twitter](https://twitter.com/intent/tweet?text=%22Inventory+Management+System%22+%2D+Day+2+%2D+Advent+of+Code+2018&url=https%3A%2F%2Fadventofcode%2Ecom%2F2018%2Fday%2F2&related=ericwastl&hashtags=AdventOfCode) [Mastodon](javascript:void(0);)] this puzzle.

(function(i,s,o,g,r,a,m){i['GoogleAnalyticsObject']=r;i[r]=i[r]||function(){
(i[r].q=i[r].q||[]).push(arguments)},i[r].l=1\*new Date();a=s.createElement(o),
m=s.getElementsByTagName(o)[0];a.async=1;a.src=g;m.parentNode.insertBefore(a,m)
})(window,document,'script','//www.google-analytics.com/analytics.js','ga');
ga('create', 'UA-69522494-1', 'auto');
ga('set', 'anonymizeIp', true);
ga('send', 'pageview');