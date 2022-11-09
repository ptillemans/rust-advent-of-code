Day 1 - Advent of Code 2018window.addEventListener('click', function(e,s,r){if(e.target.nodeName==='CODE'&&e.detail===3){s=window.getSelection();s.removeAllRanges();r=document.createRange();r.selectNodeContents(e.target);s.addRange(r);}});

[Advent of Code](/)
==========

* [[About]](/2018/about)
* [[Events]](/2018/events)
* [[Shop]](https://teespring.com/stores/advent-of-code)
* [[Settings]](/2018/settings)
* [[Log Out]](/2018/auth/logout)

@snamellit

$year=[2018](/2018);
==========

* [[Calendar]](/2018)
* [[AoC++]](/2018/support)
* [[Sponsors]](/2018/sponsors)
* [[Leaderboard]](/2018/leaderboard)
* [[Stats]](/2018/stats)

Our [sponsors](/2018/sponsors) help make Advent of Code possible:

[Alfie by Prodo](https://aoc.prodo.ai/) - a more immediate, feedback-driven coding experience. Try our online JavaScript playground with Advent of Code!

\--- Day 1: Chronal Calibration ---
----------

"We've detected some temporal anomalies," one of Santa's Elves at the Temporal Anomaly Research and Detection Instrument Station tells you. She sounded pretty worried when she called you down here. "At 500-year intervals into the past, someone has been changing Santa's history!"

"The good news is that the changes won't propagate to our time stream for another 25 days, and we have a device" - she attaches something to your wrist - "that will let you fix the changes with no such propagation delay. It's configured to send you 500 years further into the past every few days; that was the best we could do on such short notice."

"The bad news is that we are detecting roughly *fifty* anomalies throughout time; the device will indicate fixed anomalies with *stars*. The other bad news is that we only have one device and you're the best person for the job! Good lu--" She taps a button on the device and you suddenly feel like you're falling. To save Christmas, you need to get all *fifty stars* by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants *one star*. Good luck!

After feeling like you've been falling for a few minutes, you look at the device's tiny screen. "Error: Device must be calibrated before first use. Frequency drift detected. Cannot maintain destination lock." Below the message, the device shows a sequence of changes in frequency (your puzzle input). A value like `+6` means the current frequency increases by `6`; a value like `-3` means the current frequency decreases by `3`.

For example, if the device displays frequency changes of `+1, -2, +3, +1`, then starting from a frequency of zero, the following changes would occur:

* Current frequency ` 0`, change of `+1`; resulting frequency ` 1`.
* Current frequency ` 1`, change of `-2`; resulting frequency `-1`.
* Current frequency `-1`, change of `+3`; resulting frequency ` 2`.
* Current frequency ` 2`, change of `+1`; resulting frequency ` 3`.

In this example, the resulting frequency is `3`.

Here are other example situations:

* `+1, +1, +1` results in ` 3`
* `+1, +1, -2` results in ` 0`
* `-1, -2, -3` results in `-6`

Starting with a frequency of zero, *what is the resulting frequency* after all of the changes in frequency have been applied?

To begin, [get your puzzle input](1/input).

Answer:

You can also [Shareon [Twitter](https://twitter.com/intent/tweet?text=%22Chronal+Calibration%22+%2D+Day+1+%2D+Advent+of+Code+2018&url=https%3A%2F%2Fadventofcode%2Ecom%2F2018%2Fday%2F1&related=ericwastl&hashtags=AdventOfCode) [Mastodon](javascript:void(0);)] this puzzle.

(function(i,s,o,g,r,a,m){i['GoogleAnalyticsObject']=r;i[r]=i[r]||function(){
(i[r].q=i[r].q||[]).push(arguments)},i[r].l=1\*new Date();a=s.createElement(o),
m=s.getElementsByTagName(o)[0];a.async=1;a.src=g;m.parentNode.insertBefore(a,m)
})(window,document,'script','//www.google-analytics.com/analytics.js','ga');
ga('create', 'UA-69522494-1', 'auto');
ga('set', 'anonymizeIp', true);
ga('send', 'pageview');