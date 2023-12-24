Day 24 - Advent of Code 2023window.addEventListener('click', function(e,s,r){if(e.target.nodeName==='CODE'&&e.detail===3){s=window.getSelection();s.removeAllRanges();r=document.createRange();r.selectNodeContents(e.target);s.addRange(r);}});

[Advent of Code](/)
==========

* [[About]](/2023/about)
* [[Events]](/2023/events)
* [[Shop]](https://teespring.com/stores/advent-of-code)
* [[Settings]](/2023/settings)
* [[Log Out]](/2023/auth/logout)

Peter Tillemans [(AoC++)](/2023/support) 46\*

λy.[2023](/2023)
==========

* [[Calendar]](/2023)
* [[AoC++]](/2023/support)
* [[Sponsors]](/2023/sponsors)
* [[Leaderboard]](/2023/leaderboard)
* [[Stats]](/2023/stats)

Our [sponsors](/2023/sponsors) help make Advent of Code possible:

[Kotlin by JetBrains](https://kotlinlang.org/) - Jingle bells, Kotlin pals, coding all the way! Solve puzzles, catch our daily livestreams for expert guidance, embrace the joy of Kotlin, and engage with a fantastic community. Happy holidays and happy coding!

\--- Day 24: Never Tell Me The Odds ---
----------

It seems like something is going wrong with the snow-making process. Instead of forming snow, the water that's been absorbed into the air seems to be forming [hail](https://en.wikipedia.org/wiki/Hail)!

Maybe there's something you can do to break up the hailstones?

Due to strong, probably-magical winds, the hailstones are all flying through the air in perfectly linear trajectories. You make a note of each hailstone's *position* and *velocity* (your puzzle input). For example:

```
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3

```

Each line of text corresponds to the position and velocity of a single hailstone. The positions indicate where the hailstones are *right now* (at time `0`). The velocities are constant and indicate exactly how far each hailstone will move in *one nanosecond*.

Each line of text uses the format `px py pz @ vx vy vz`. For instance, the hailstone specified by `20, 19, 15 @ 1, -5, -3` has initial X position `20`, Y position `19`, Z position `15`, X velocity `1`, Y velocity `-5`, and Z velocity `-3`. After one nanosecond, the hailstone would be at `21, 14, 12`.

Perhaps you won't have to do anything. How likely are the hailstones to collide with each other and smash into tiny ice crystals?

To estimate this, consider only the X and Y axes; *ignore the Z axis*. Looking *forward in time*, how many of the hailstones' *paths* will intersect within a test area? (The hailstones themselves don't have to collide, just test for intersections between the paths they will trace.)

In this example, look for intersections that happen with an X and Y position each at least `7` and at most `27`; in your actual data, you'll need to check a much larger test area. Comparing all pairs of hailstones' future paths produces the following results:

```
Hailstone A: 19, 13, 30 @ -2, 1, -2
Hailstone B: 18, 19, 22 @ -1, -1, -2
Hailstones' paths will cross inside the test area (at x=14.333, y=15.333).

Hailstone A: 19, 13, 30 @ -2, 1, -2
Hailstone B: 20, 25, 34 @ -2, -2, -4
Hailstones' paths will cross inside the test area (at x=11.667, y=16.667).

Hailstone A: 19, 13, 30 @ -2, 1, -2
Hailstone B: 12, 31, 28 @ -1, -2, -1
Hailstones' paths will cross outside the test area (at x=6.2, y=19.4).

Hailstone A: 19, 13, 30 @ -2, 1, -2
Hailstone B: 20, 19, 15 @ 1, -5, -3
Hailstones' paths crossed in the past for hailstone A.

Hailstone A: 18, 19, 22 @ -1, -1, -2
Hailstone B: 20, 25, 34 @ -2, -2, -4
Hailstones' paths are parallel; they never intersect.

Hailstone A: 18, 19, 22 @ -1, -1, -2
Hailstone B: 12, 31, 28 @ -1, -2, -1
Hailstones' paths will cross outside the test area (at x=-6, y=-5).

Hailstone A: 18, 19, 22 @ -1, -1, -2
Hailstone B: 20, 19, 15 @ 1, -5, -3
Hailstones' paths crossed in the past for both hailstones.

Hailstone A: 20, 25, 34 @ -2, -2, -4
Hailstone B: 12, 31, 28 @ -1, -2, -1
Hailstones' paths will cross outside the test area (at x=-2, y=3).

Hailstone A: 20, 25, 34 @ -2, -2, -4
Hailstone B: 20, 19, 15 @ 1, -5, -3
Hailstones' paths crossed in the past for hailstone B.

Hailstone A: 12, 31, 28 @ -1, -2, -1
Hailstone B: 20, 19, 15 @ 1, -5, -3
Hailstones' paths crossed in the past for both hailstones.

```

So, in this example, `*2*` hailstones' future paths cross inside the boundaries of the test area.

However, you'll need to search a much larger test area if you want to see if any hailstones might collide. Look for intersections that happen with an X and Y position each at least `200000000000000` and at most `400000000000000`. Disregard the Z axis entirely.

Considering only the X and Y axes, check all pairs of hailstones' future paths for intersections. *How many of these intersections occur within the test area?*

To begin, [get your puzzle input](24/input).

Answer:

You can also [Shareon [Twitter](https://twitter.com/intent/tweet?text=%22Never+Tell+Me+The+Odds%22+%2D+Day+24+%2D+Advent+of+Code+2023&url=https%3A%2F%2Fadventofcode%2Ecom%2F2023%2Fday%2F24&related=ericwastl&hashtags=AdventOfCode) [Mastodon](javascript:void(0);)] this puzzle.

(function(i,s,o,g,r,a,m){i['GoogleAnalyticsObject']=r;i[r]=i[r]||function(){
(i[r].q=i[r].q||[]).push(arguments)},i[r].l=1\*new Date();a=s.createElement(o),
m=s.getElementsByTagName(o)[0];a.async=1;a.src=g;m.parentNode.insertBefore(a,m)
})(window,document,'script','//www.google-analytics.com/analytics.js','ga');
ga('create', 'UA-69522494-1', 'auto');
ga('set', 'anonymizeIp', true);
ga('send', 'pageview');