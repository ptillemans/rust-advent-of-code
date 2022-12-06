Day 6 - Advent of Code 2022window.addEventListener('click', function(e,s,r){if(e.target.nodeName==='CODE'&&e.detail===3){s=window.getSelection();s.removeAllRanges();r=document.createRange();r.selectNodeContents(e.target);s.addRange(r);}});

[Advent of Code](/)
==========

* [[About]](/2022/about)
* [[Events]](/2022/events)
* [[Shop]](https://teespring.com/stores/advent-of-code)
* [[Settings]](/2022/settings)
* [[Log Out]](/2022/auth/logout)

@snamellit [(AoC++)](/2022/support) 10\*

{year=\>[2022](/2022)}
==========

* [[Calendar]](/2022)
* [[AoC++]](/2022/support)
* [[Sponsors]](/2022/sponsors)
* [[Leaderboard]](/2022/leaderboard)
* [[Stats]](/2022/stats)

Our [sponsors](/2022/sponsors) help make Advent of Code possible:

[Ahrefs](https://ahrefs.com/) - Work on the next general purpose search engine, a world-class crawler, and real big data. Leveraging bleeding-edge hardware and advanced programming technologies. From anywhere in the world. OCaml, ReasonML, Dlang, C++

\--- Day 6: Tuning Trouble ---
----------

The preparations are finally complete; you and the Elves leave camp on foot and begin to make your way toward the *star* fruit grove.

As you move through the dense undergrowth, one of the Elves gives you a handheld *device*. He says that it has many fancy features, but the most important one to set up right now is the *communication system*.

However, because he's heard you have [significant](/2016/day/6) [experience](/2016/day/25) [dealing](/2019/day/7) [with](/2019/day/9) [signal-based](/2019/day/16) [systems](/2021/day/25), he convinced the other Elves that it would be okay to give you their one malfunctioning device - surely you'll have no problem fixing it.

As if inspired by comedic timing, the device emits a few colorful sparks.

To be able to communicate with the Elves, the device needs to *lock on to their signal*. The signal is a series of seemingly-random characters that the device receives one at a time.

To fix the communication system, you need to add a subroutine to the device that detects a *start-of-packet marker* in the datastream. In the protocol being used by the Elves, the start of a packet is indicated by a sequence of *four characters that are all different*.

The device will send your subroutine a datastream buffer (your puzzle input); your subroutine needs to identify the first position where the four most recently received characters were all different. Specifically, it needs to report the number of characters from the beginning of the buffer to the end of the first such four-character marker.

For example, suppose you receive the following datastream buffer:

```
mjqjpqmgbljsphdztnvjfqwrcgsmlb
```

After the first three characters (`mjq`) have been received, there haven't been enough characters received yet to find the marker. The first time a marker could occur is after the fourth character is received, making the most recent four characters `mjqj`. Because `j` is repeated, this isn't a marker.

The first time a marker appears is after the *seventh* character arrives. Once it does, the last four characters received are `jpqm`, which are all different. In this case, your subroutine should report the value `*7*`, because the first start-of-packet marker is complete after 7 characters have been processed.

Here are a few more examples:

* `bvwbjplbgvbhsrlpgdmjqwftvncz`: first marker after character `*5*`
* `nppdvjthqldpwncqszvftbrmjlhg`: first marker after character `*6*`
* `nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg`: first marker after character `*10*`
* `zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw`: first marker after character `*11*`

*How many characters need to be processed before the first start-of-packet marker is detected?*

To begin, [get your puzzle input](6/input).

Answer:

You can also [Shareon [Twitter](https://twitter.com/intent/tweet?text=%22Tuning+Trouble%22+%2D+Day+6+%2D+Advent+of+Code+2022&url=https%3A%2F%2Fadventofcode%2Ecom%2F2022%2Fday%2F6&related=ericwastl&hashtags=AdventOfCode) [Mastodon](javascript:void(0);)] this puzzle.

(function(i,s,o,g,r,a,m){i['GoogleAnalyticsObject']=r;i[r]=i[r]||function(){
(i[r].q=i[r].q||[]).push(arguments)},i[r].l=1\*new Date();a=s.createElement(o),
m=s.getElementsByTagName(o)[0];a.async=1;a.src=g;m.parentNode.insertBefore(a,m)
})(window,document,'script','//www.google-analytics.com/analytics.js','ga');
ga('create', 'UA-69522494-1', 'auto');
ga('set', 'anonymizeIp', true);
ga('send', 'pageview');