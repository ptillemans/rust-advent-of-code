Day 23 - Advent of Code 2023window.addEventListener('click', function(e,s,r){if(e.target.nodeName==='CODE'&&e.detail===3){s=window.getSelection();s.removeAllRanges();r=document.createRange();r.selectNodeContents(e.target);s.addRange(r);}});

[Advent of Code](/)
==========

* [[About]](/2023/about)
* [[Events]](/2023/events)
* [[Shop]](https://teespring.com/stores/advent-of-code)
* [[Settings]](/2023/settings)
* [[Log Out]](/2023/auth/logout)

Peter Tillemans [(AoC++)](/2023/support) 44\*

/^[2023](/2023)$/
==========

* [[Calendar]](/2023)
* [[AoC++]](/2023/support)
* [[Sponsors]](/2023/sponsors)
* [[Leaderboard]](/2023/leaderboard)
* [[Stats]](/2023/stats)

Our [sponsors](/2023/sponsors) help make Advent of Code possible:

[JPMorgan Chase](https://www.jpmorgan.com/) - With over 57,000 technologists globally and an annual tech spend of $15 billion, JPMorgan Chase is dedicated to improving the design, analytics, coding and testing that goes into creating high quality software products.

\--- Day 23: A Long Walk ---
----------

The Elves resume water filtering operations! Clean water starts flowing over the edge of Island Island.

They offer to help *you* go over the edge of Island Island, too! Just hold on tight to one end of this impossibly long rope and they'll lower you down a safe distance from the massive waterfall you just created.

As you finally reach Snow Island, you see that the water isn't really reaching the ground: it's being *absorbed by the air* itself. It looks like you'll finally have a little downtime while the moisture builds up to snow-producing levels. Snow Island is pretty scenic, even without any snow; why not take a walk?

There's a map of nearby hiking trails (your puzzle input) that indicates *paths* (`.`), *forest* (`#`), and steep *slopes* (`^`, `>`, `v`, and `<`).

For example:

```
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#

```

You're currently on the single path tile in the top row; your goal is to reach the single path tile in the bottom row. Because of all the mist from the waterfall, the slopes are probably quite *icy*; if you step onto a slope tile, your next step must be *downhill* (in the direction the arrow is pointing). To make sure you have the most scenic hike possible, *never step onto the same tile twice*. What is the longest hike you can take?

In the example above, the longest hike you can take is marked with `O`, and your starting position is marked `S`:

```
#S#####################
#OOOOOOO#########...###
#######O#########.#.###
###OOOOO#OOO>.###.#.###
###O#####O#O#.###.#.###
###OOOOO#O#O#.....#...#
###v###O#O#O#########.#
###...#O#O#OOOOOOO#...#
#####.#O#O#######O#.###
#.....#O#O#OOOOOOO#...#
#.#####O#O#O#########v#
#.#...#OOO#OOO###OOOOO#
#.#.#v#######O###O###O#
#...#.>.#...>OOO#O###O#
#####v#.#.###v#O#O###O#
#.....#...#...#O#O#OOO#
#.#########.###O#O#O###
#...###...#...#OOO#O###
###.###.#.###v#####O###
#...#...#.#.>.>.#.>O###
#.###.###.#.###.#.#O###
#.....###...###...#OOO#
#####################O#

```

This hike contains `*94*` steps. (The other possible hikes you could have taken were `90`, `86`, `82`, `82`, and `74` steps long.)

Find the longest hike you can take through the hiking trails listed on your map. *How many steps long is the longest hike?*

To begin, [get your puzzle input](23/input).

Answer:

You can also [Shareon [Twitter](https://twitter.com/intent/tweet?text=%22A+Long+Walk%22+%2D+Day+23+%2D+Advent+of+Code+2023&url=https%3A%2F%2Fadventofcode%2Ecom%2F2023%2Fday%2F23&related=ericwastl&hashtags=AdventOfCode) [Mastodon](javascript:void(0);)] this puzzle.

(function(i,s,o,g,r,a,m){i['GoogleAnalyticsObject']=r;i[r]=i[r]||function(){
(i[r].q=i[r].q||[]).push(arguments)},i[r].l=1\*new Date();a=s.createElement(o),
m=s.getElementsByTagName(o)[0];a.async=1;a.src=g;m.parentNode.insertBefore(a,m)
})(window,document,'script','//www.google-analytics.com/analytics.js','ga');
ga('create', 'UA-69522494-1', 'auto');
ga('set', 'anonymizeIp', true);
ga('send', 'pageview');