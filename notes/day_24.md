inp w
mul x 0  (set x = 0) (this happens every single time)
add x z  (x = z) (no-op as z = 0) (this also happens every single time) (essentially x gets set to z from previous digit)
mod x 26 (x = z mod 26) (no-op) (happens every time) (x gets set to z mod 26)
div z 1  (no-op) (this is 
add x 14 (x = 14)
eql x w  (x = 0) (because inp is always < 10)
eql x 0  (x = 1)
mul y 0  (no-op)
add y 25 (y = 25)
mul y x  (no-op because x = 1)
add y 1  (y = 26)
mul z y  (no-op because z is still 0)
mul y 0  (y = 0)
add y w  (y = inp0)
add y 8  (y = inp0 + 8)
mul y x  (no-op because x = 1)
add z y  (z = inp0 + 8)
The rest of the instructions are always the same.

In summary:
inp w    (w is inp)
mul x 0  (x = 0)
add x z  (x = z_l)
mod x 26 (x = z_l mod 26)
div z k  (z /= 1 or 26, k is always 1 or 26, see table)
add x a  (see table for a)
eql x w  (x is 1 if cond)
eql x 0  (x is 0 if cond) (invert x)
mul y 0  (y = 0)
add y 25 (y = 25)
mul y x  (y = 25 if !cond)
add y 1  (y = 26 if !cond, else 1)
mul z y  (z *= 26 if !cond, else noop)
mul y 0  (y = 0)
add y w  (y = inp)
add y b  (y = inp + b, see table for b)
mul y x  (y = inp + b if !cond, else 0)
add z y  (z += inp + b if !cond, else noop)
where cond means "if (z_l mod 26) + a == inp_current"

Table:
     0  1  2   3  4  5   6  7  8  9 10 11 12 13
  +--------------------------------------------
a | 14 15 13 -10 14 -3 -14 12 14 12 -6 -6 -2 -9
b |  8 11  2  11  1  5  10  6  1 11  9 14 11  2
k |  1  1  1  26  1 26  26  1  1  1 26 26 26 26

Which can be summarised as:
- find cond = if (z_l mod 26) + a == inp (z_l for 0 is 0) z_l mod 26 = inp_l + b_l, so this is actually inp_l + b_l + a == inp. (inp + b < 26 for all b because b < 17, so inp + b will not be affected by the mod 26)
- divide z by 26 (sometimes)
- if !cond, z = (z_l * 26) + inp + b

In other words, z is an accumulator/stack that adds on inp_i + b_i from each digit, and through a combination of dividing by 26, and making cond true, all 14 digits must be removed from z somehow. There are 7 divisions, so at least 7 more of the cond checks must be true.
A divide will remove the last existing layer of the stack, and a cond will prevent the current layer from being added. This difference means that cond is the only thing that can remove the last layer (layer 13/the 14th layer) from being added.
Divide happens after cond is checked, so even if divide removes the last layer, the cond check that layer still uses the inp_i + b_i of that layer being removed.

It is helpful to list out all the digits where cond may become true:
3: inp_2 must be 9, inp_3 must be 1, then 9 + 2 - 10 == 1
5: inp_4 + 1 - 3 == inp_5, so inp_4 - 2 == inp_5 makes cond true
6: if 2,3,4,5 gone, inp_1 + 11 - 14 == inp_6, or inp_1 - 3 == inp_6
10: inp_9 + 11 - 6 == inp_10, so inp_9 + 5 == inp_10
11: if both 9 and 10 are gone, inp_8 + 1 - 6 == inp_11, or inp_8 - 5 == inp_11
12: if 8,9,10,11 gone, inp_7 + 6 - 2 == inp_12 or inp_7 + 4 == inp_12
13: if 1,2,3,4,5,6,7,8,9,10,11,12 gone, inp_0 + 8 - 9 == inp_13, or inp_0 - 1 == inp_13

Also it is helpful to list which digits each divide 26 can actually affect.
3: can only affect 2, as 0, 1 and 2 cannot make cond true, so the divide here will remove 2.
5: can only affect 4, as 4 has a = 14, which cannot make cond true regardless of values of b and inp, so 4 will always be added, then removed by this division.
6: if both 3 and 5 are skipped by cond, then this removes 1
10: removes 9 which must be added
11: if both 9, 10 removed, this removes 8
12: if 8,9,10,11 gone, removes 7
13: if 1,2,3,4,5,6,7,8,9,10,11,12 gone, this removes 0

As there are maximum 7 cases for cond to be true, this gives us our constraints:
inp_0 - 1 == inp_13
inp_1 - 3 == inp_6
inp_2 - 8 == inp_3 (i.e. inp_2 == 9, inp_3 == 1)
inp_4 - 2 == inp_5
inp_7 + 4 == inp_12
inp_8 - 5 == inp_11
inp_9 + 5 == inp_10
