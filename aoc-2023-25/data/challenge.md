Day 25 - Advent of Code 2023window.addEventListener('click', function(e,s,r){if(e.target.nodeName==='CODE'&&e.detail===3){s=window.getSelection();s.removeAllRanges();r=document.createRange();r.selectNodeContents(e.target);s.addRange(r);}});

[Advent of Code](/)
==========

* [[About]](/2023/about)
* [[Events]](/2023/events)
* [[Shop]](https://teespring.com/stores/advent-of-code)
* [[Settings]](/2023/settings)
* [[Log Out]](/2023/auth/logout)

Peter Tillemans [(AoC++)](/2023/support) 48\*

/\*[2023](/2023)\*/
==========

* [[Calendar]](/2023)
* [[AoC++]](/2023/support)
* [[Sponsors]](/2023/sponsors)
* [[Leaderboard]](/2023/leaderboard)
* [[Stats]](/2023/stats)

Our [sponsors](/2023/sponsors) help make Advent of Code possible:

[THE MERGE](https://merge.berlin/) - The Developer Experience Conference in Berlin, June 2024 (created by the co-founder of GitHub)

\--- Day 25: Snowverload ---
----------

*Still* somehow without snow, you go to the last place you haven't checked: the center of Snow Island, directly below the waterfall.

Here, someone has clearly been trying to fix the problem. Scattered everywhere are hundreds of weather machines, almanacs, communication modules, hoof prints, machine parts, mirrors, lenses, and so on.

Somehow, everything has been *wired together* into a massive snow-producing apparatus, but nothing seems to be running. You check a tiny screen on one of the communication modules: `Error 2023`. It doesn't say what `Error 2023` means, but it *does* have the phone number for a support line printed on it.

"Hi, you've reached Weather Machines And So On, Inc. How can I help you?" You explain the situation.

"Error 2023, you say? Why, that's a power overload error, of course! It means you have too many components plugged in. Try unplugging some components and--" You explain that there are hundreds of components here and you're in a bit of a hurry.

"Well, let's see how bad it is; do you see a *big red reset button* somewhere? It should be on its own module. If you push it, it probably won't fix anything, but it'll report how overloaded things are." After a minute or two, you find the reset button; it's so big that it takes two hands just to get enough leverage to push it. Its screen then displays:

```
SYSTEM OVERLOAD!

Connected components would require
power equal to at least 100 stars!

```

"Wait, *how* many components did you say are plugged in? With that much equipment, you could produce snow for an *entire*--" You disconnect the call.

You have nowhere near that many stars - you need to find a way to disconnect at least half of the equipment here, but it's already Christmas! You only have time to disconnect *three wires*.

Fortunately, someone left a wiring diagram (your puzzle input) that shows *how the components are connected*. For example:

```
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr

```

Each line shows the *name of a component*, a colon, and then *a list of other components* to which that component is connected. Connections aren't directional; `abc: xyz` and `xyz: abc` both represent the same configuration. Each connection between two components is represented only once, so some components might only ever appear on the left or right side of a colon.

In this example, if you disconnect the wire between `hfx`/`pzl`, the wire between `bvb`/`cmg`, and the wire between `nvd`/`jqt`, you will *divide the components into two separate, disconnected groups*:

* `*9*` components: `cmg`, `frs`, `lhk`, `lsr`, `nvd`, `pzl`, `qnr`, `rsh`, and `rzs`.
* `*6*` components: `bvb`, `hfx`, `jqt`, `ntq`, `rhn`, and `xhk`.

Multiplying the sizes of these groups together produces `*54*`.

Find the three wires you need to disconnect in order to divide the components into two separate groups. *What do you get if you multiply the sizes of these two groups together?*

To begin, [get your puzzle input](25/input).

Answer:

You can also [Shareon [Twitter](https://twitter.com/intent/tweet?text=%22Snowverload%22+%2D+Day+25+%2D+Advent+of+Code+2023&url=https%3A%2F%2Fadventofcode%2Ecom%2F2023%2Fday%2F25&related=ericwastl&hashtags=AdventOfCode) [Mastodon](javascript:void(0);)] this puzzle.

(function(i,s,o,g,r,a,m){i['GoogleAnalyticsObject']=r;i[r]=i[r]||function(){
(i[r].q=i[r].q||[]).push(arguments)},i[r].l=1\*new Date();a=s.createElement(o),
m=s.getElementsByTagName(o)[0];a.async=1;a.src=g;m.parentNode.insertBefore(a,m)
})(window,document,'script','//www.google-analytics.com/analytics.js','ga');
ga('create', 'UA-69522494-1', 'auto');
ga('set', 'anonymizeIp', true);
ga('send', 'pageview');