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


