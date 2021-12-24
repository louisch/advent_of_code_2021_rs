suppose o and s2 share a beacon b (where o is the origin)

then, (o - b) + (s2 - b) = s2

suppose a scanner gives list b_i

and two scanners have some common intersecting subset BC that is the same beacons with len(BC) >= 12.

then we have, for example (letting s1 be the origin)

-b_i + f(b_j) = s2 (where i is the index in s1's list, and j is the index in s2's list)
for i in 12 of s1's indices and
for j in 12 of s2's indices
and f is the function that rotates s2's basis vectors


# new idea

The distances between the same two beacons remain the same regardless of the coordinate system, so for one scanner we can save the distances between every beacon with every other one (n^2 where n is about 30 so around 900 entries per scanner, which is quite small), then use this distance matrix with other scanners distance matrices (calculated the same way), and for rows in both matrices that have >= 11 distances that are the same, that indicates those two are the same beacon (and also that there likely are 11 other beacons that are shared between those). Trying to find which of the 24 possible orientations the second scanner is in is another problem, however.
