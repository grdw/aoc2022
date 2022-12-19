## Step 1: Make a tree.

This tree looks like such in the example:

```
    BB
   /  \
 AA   CC
  |\  /
  | DD -- EE -- FF -- GG -- HH
  |
 II
   \
    JJ
```

The tree is bi-directional. So you have these nodes:

```
{AA, BB, CC, DD, EE, FF, GG, HH, II, JJ}
```

And these edges:

```
{
  AA-BB, BB-AA,
  AA-DD, DD-AA,
  AA-II, II-AA,
  BB-CC, CC-BB,
  CC-DD, DD-CC,
  etc.
}
```

## Step 2: Dijkstra again? Or some other node-traversal algorithm
You start at 'AA', and the idea is to move from one node to the route with
the highest total valve rate (Dijkstra in reverse). However, this is Dijkstra
with a twist, because it's like: reverse-"Dijkstra" every node, incl. travel
time.

Yeah, it's not Dijkstra because there's not a "destination" . Just a start.
The end result here is that all the valves are open (or "How many valves can
you open in 30 minutes?"). Not sure if you needs BFS or DFS.

# Scribbles:

Every minute you can do a bunch of moves:

1. Idle (Doing nothing)
2. Move to one of the adjacent nodes
3. Open the valve of the node you're currently on

Perhaps the trick is in the 30 minutes? I mean, you can yolo run to all of them aka brute-forcing the solution.
How many possible ways can you navigate this route in 30 minutes?

Imagine you only have this graph:

```
  B -- C -- E
 /    /
A -- D
```

You start at `A`. Then within 30 minutes you can:

Minute 1:
At A choose:
1. Move from A to D
2. Move from A to B
3. Open valve A (if A has a flow rate of 0, don't pick)

Minute 2 (went with option 1)
At D you can:
1. Move from D to C
2. Move from D back to A (but why?)
3. Open valve D

Minute 2 (went with option 2)
At B you can:
1. Move from B to C
2. Move from B back to A (but why?)
3. Open valve B

Minute 2 (went with option 3).
Opened valve A

I guess you're going to get a lot of vectors with options of moves, opens and idles. As soon as all the valves are opened there's not a lot of options left, so the rest will just be idle.

So in my head this is:

```
[
  [Move(AA), Open(AA), Move(DD), Open(DD), etc.],
  [Move(AA), Move(DD), Open(DD), etc.]
]
```

Then you have to calculate all of the 'flow rates' and sum them and pick
the maximum

Now, it's just a question of recursion.
