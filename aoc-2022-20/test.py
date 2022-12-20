#!/usr/bin/env python3
buf = [(pos, int(line)) for pos, line in enumerate(open('data/input.txt').readlines())]
for orig_pos, number in buf.copy(): # traverse in input order
    from_idx = buf.index((orig_pos, number))
    to_idx = (from_idx + number) % (len(buf)-1)
    buf.insert(to_idx, buf.pop(from_idx))
zero_idx = [x[1] for x in buf].index(0)
print(sum(buf[(zero_idx+offset)%len(buf)][1] for offset in [1000,2000,3000]))

