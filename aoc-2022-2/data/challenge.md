Day 2 - Advent of Code 2022window.addEventListener('click', function(e,s,r){if(e.target.nodeName==='CODE'&&e.detail===3){s=window.getSelection();s.removeAllRanges();r=document.createRange();r.selectNodeContents(e.target);s.addRange(r);}});

[Advent of Code](/)
==========

* [[About]](/2022/about)
* [[Events]](/2022/events)
* [[Shop]](https://teespring.com/stores/advent-of-code)
* [[Settings]](/2022/settings)
* [[Log Out]](/2022/auth/logout)

@snamellit [(AoC++)](/2022/support) 2\*

//[2022](/2022)
==========

* [[Calendar]](/2022)
* [[AoC++]](/2022/support)
* [[Sponsors]](/2022/sponsors)
* [[Leaderboard]](/2022/leaderboard)
* [[Stats]](/2022/stats)

Our [sponsors](/2022/sponsors) help make Advent of Code possible:

[Optiver](https://optiver.com/advent-of-code) - Love solving puzzles? So do we! We're hiring engineers to code trading systems with sub-nanosecond performance. Get ready for daily challenges, continuous learning and the freedom to bring your software solutions to life

\--- Day 2: Rock Paper Scissors ---
----------

The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack storage, a giant [Rock Paper Scissors](https://en.wikipedia.org/wiki/Rock_paper_scissors) tournament is already in progress.

Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players choose the same shape, the round instead ends in a draw.

Appreciative of your help yesterday, one Elf gives you an *encrypted strategy guide* (your puzzle input) that they say will be sure to help you win. "The first column is what your opponent is going to play: `A` for Rock, `B` for Paper, and `C` for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.

The second column, you reason, must be what you should play in response: `X` for Rock, `Y` for Paper, and `Z` for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.

The winner of the whole tournament is the player with the highest score. Your *total score* is the sum of your scores for each round. The score for a single round is the score for the *shape you selected* (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the *outcome of the round* (0 if you lost, 3 if the round was a draw, and 6 if you won).

Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.

For example, suppose you were given the following strategy guide:

```
A Y
B X
C Z

```

This strategy guide predicts and recommends the following:

* In the first round, your opponent will choose Rock (`A`), and you should choose Paper (`Y`). This ends in a win for you with a score of *8* (2 because you chose Paper + 6 because you won).
* In the second round, your opponent will choose Paper (`B`), and you should choose Rock (`X`). This ends in a loss for you with a score of *1* (1 + 0).
* The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = *6*.

In this example, if you were to follow the strategy guide, you would get a total score of `*15*` (8 + 1 + 6).

*What would your total score be if everything goes exactly according to your strategy guide?*

To begin, [get your puzzle input](2/input).

Answer:

You can also [Shareon [Twitter](https://twitter.com/intent/tweet?text=%22Rock+Paper+Scissors%22+%2D+Day+2+%2D+Advent+of+Code+2022&url=https%3A%2F%2Fadventofcode%2Ecom%2F2022%2Fday%2F2&related=ericwastl&hashtags=AdventOfCode) [Mastodon](javascript:void(0);)] this puzzle.

(function(i,s,o,g,r,a,m){i['GoogleAnalyticsObject']=r;i[r]=i[r]||function(){
(i[r].q=i[r].q||[]).push(arguments)},i[r].l=1\*new Date();a=s.createElement(o),
m=s.getElementsByTagName(o)[0];a.async=1;a.src=g;m.parentNode.insertBefore(a,m)
})(window,document,'script','//www.google-analytics.com/analytics.js','ga');
ga('create', 'UA-69522494-1', 'auto');
ga('set', 'anonymizeIp', true);
ga('send', 'pageview');