Day 5 - Advent of Code 2022window.addEventListener('click', function(e,s,r){if(e.target.nodeName==='CODE'&&e.detail===3){s=window.getSelection();s.removeAllRanges();r=document.createRange();r.selectNodeContents(e.target);s.addRange(r);}});

[Advent of Code](/)
==========

* [[About]](/2022/about)
* [[Events]](/2022/events)
* [[Shop]](https://teespring.com/stores/advent-of-code)
* [[Settings]](/2022/settings)
* [[Log Out]](/2022/auth/logout)

@snamellit [(AoC++)](/2022/support) 8\*

{year=\>[2022](/2022)}
==========

* [[Calendar]](/2022)
* [[AoC++]](/2022/support)
* [[Sponsors]](/2022/sponsors)
* [[Leaderboard]](/2022/leaderboard)
* [[Stats]](/2022/stats)

Our [sponsors](/2022/sponsors) help make Advent of Code possible:

[Instadeep](https://www.instadeep.com/about-us/careers/) - Research + Deep RL + HPC =\> Solve Real-World Problems with AI

\--- Day 5: Supply Stacks ---
----------

The expedition can depart as soon as the final supplies have been unloaded from the ships. Supplies are stored in stacks of marked *crates*, but because the needed supplies are buried under many other crates, the crates need to be rearranged.

The ship has a *giant cargo crane* capable of moving crates between stacks. To ensure none of the crates get crushed or fall over, the crane operator will rearrange them in a series of carefully-planned steps. After the crates are rearranged, the desired crates will be at the top of each stack.

The Elves don't want to interrupt the crane operator during this delicate procedure, but they forgot to ask her *which* crate will end up where, and they want to be ready to unload them as soon as possible so they can embark.

They do, however, have a drawing of the starting stacks of crates *and* the rearrangement procedure (your puzzle input). For example:

```
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2

```

In this example, there are three stacks of crates. Stack 1 contains two crates: crate `Z` is on the bottom, and crate `N` is on top. Stack 2 contains three crates; from bottom to top, they are crates `M`, `C`, and `D`. Finally, stack 3 contains a single crate, `P`.

Then, the rearrangement procedure is given. In each step of the procedure, a quantity of crates is moved from one stack to a different stack. In the first step of the above rearrangement procedure, one crate is moved from stack 2 to stack 1, resulting in this configuration:

```
[D]        
[N] [C]    
[Z] [M] [P]
 1   2   3

```

In the second step, three crates are moved from stack 1 to stack 3. Crates are moved *one at a time*, so the first crate to be moved (`D`) ends up below the second and third crates:

```
        [Z]
        [N]
    [C] [D]
    [M] [P]
 1   2   3

```

Then, both crates are moved from stack 2 to stack 1. Again, because crates are moved *one at a time*, crate `C` ends up below crate `M`:

```
        [Z]
        [N]
[M]     [D]
[C]     [P]
 1   2   3

```

Finally, one crate is moved from stack 1 to stack 2:

```
        [Z]
        [N]
        [D]
[C] [M] [P]
 1   2   3

```

The Elves just need to know *which crate will end up on top of each stack*; in this example, the top crates are `C` in stack 1, `M` in stack 2, and `Z` in stack 3, so you should combine these together and give the Elves the message `*CMZ*`.

*After the rearrangement procedure completes, what crate ends up on top of each stack?*

To begin, [get your puzzle input](5/input).

Answer:

You can also [Shareon [Twitter](https://twitter.com/intent/tweet?text=%22Supply+Stacks%22+%2D+Day+5+%2D+Advent+of+Code+2022&url=https%3A%2F%2Fadventofcode%2Ecom%2F2022%2Fday%2F5&related=ericwastl&hashtags=AdventOfCode) [Mastodon](javascript:void(0);)] this puzzle.

(function(i,s,o,g,r,a,m){i['GoogleAnalyticsObject']=r;i[r]=i[r]||function(){
(i[r].q=i[r].q||[]).push(arguments)},i[r].l=1\*new Date();a=s.createElement(o),
m=s.getElementsByTagName(o)[0];a.async=1;a.src=g;m.parentNode.insertBefore(a,m)
})(window,document,'script','//www.google-analytics.com/analytics.js','ga');
ga('create', 'UA-69522494-1', 'auto');
ga('set', 'anonymizeIp', true);
ga('send', 'pageview');