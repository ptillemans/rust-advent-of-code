Day 6 - Advent of Code 2018window.addEventListener('click', function(e,s,r){if(e.target.nodeName==='CODE'&&e.detail===3){s=window.getSelection();s.removeAllRanges();r=document.createRange();r.selectNodeContents(e.target);s.addRange(r);}});

[Advent of Code](/)
==========

* [[About]](/2018/about)
* [[Events]](/2018/events)
* [[Shop]](https://teespring.com/stores/advent-of-code)
* [[Settings]](/2018/settings)
* [[Log Out]](/2018/auth/logout)

@snamellit 10\*

var y=[2018](/2018);
==========

* [[Calendar]](/2018)
* [[AoC++]](/2018/support)
* [[Sponsors]](/2018/sponsors)
* [[Leaderboard]](/2018/leaderboard)
* [[Stats]](/2018/stats)

Our [sponsors](/2018/sponsors) help make Advent of Code possible:

[SmartyStreets](https://smartystreets.com/aoc) - Global address validation made by developers, for developers

\--- Day 6: Chronal Coordinates ---
----------

The device on your wrist beeps several times, and once again you feel like you're falling.

"Situation critical," the device announces. "Destination indeterminate. Chronal interference detected. Please specify new target coordinates."

The device then produces a list of coordinates (your puzzle input). Are they places it thinks are safe or dangerous? It recommends you check manual page 729. The Elves did not give you a manual.

*If they're dangerous,* maybe you can minimize the danger by finding the coordinate that gives the largest distance from the other points.

Using only the [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry), determine the *area* around each coordinate by counting the number of [integer](https://en.wikipedia.org/wiki/Integer) X,Y locations that are *closest* to that coordinate (and aren't *tied in distance* to any other coordinate).

Your goal is to find the size of the *largest area* that isn't infinite. For example, consider the following list of coordinates:

```
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9

```

If we name these coordinates `A` through `F`, we can draw them on a grid, putting `0,0` at the top left:

```
..........
.A........
..........
........C.
...D......
.....E....
.B........
..........
..........
........F.

```

This view is partial - the actual grid extends infinitely in all directions. Using the Manhattan distance, each location's closest coordinate can be determined, shown here in lowercase:

```
aaaaa.cccc
aAaaa.cccc
aaaddecccc
aadddeccCc
..dDdeeccc
bb.deEeecc
bBb.eeee..
bbb.eeefff
bbb.eeffff
bbb.ffffFf

```

Locations shown as `.` are equally far from two or more coordinates, and so they don't count as being closest to any.

In this example, the areas of coordinates A, B, C, and F are infinite - while not shown here, their areas extend forever outside the visible grid. However, the areas of coordinates D and E are finite: D is closest to 9 locations, and E is closest to 17 (both including the coordinate's location itself). Therefore, in this example, the size of the largest area is *17*.

*What is the size of the largest area* that isn't infinite?

To begin, [get your puzzle input](6/input).

Answer:

You can also [Shareon [Twitter](https://twitter.com/intent/tweet?text=%22Chronal+Coordinates%22+%2D+Day+6+%2D+Advent+of+Code+2018&url=https%3A%2F%2Fadventofcode%2Ecom%2F2018%2Fday%2F6&related=ericwastl&hashtags=AdventOfCode) [Mastodon](javascript:void(0);)] this puzzle.

(function(i,s,o,g,r,a,m){i['GoogleAnalyticsObject']=r;i[r]=i[r]||function(){
(i[r].q=i[r].q||[]).push(arguments)},i[r].l=1\*new Date();a=s.createElement(o),
m=s.getElementsByTagName(o)[0];a.async=1;a.src=g;m.parentNode.insertBefore(a,m)
})(window,document,'script','//www.google-analytics.com/analytics.js','ga');
ga('create', 'UA-69522494-1', 'auto');
ga('set', 'anonymizeIp', true);
ga('send', 'pageview');