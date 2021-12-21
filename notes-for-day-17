v_x = min(abs(v_x0 - t), 0)
v_y = v_y - t


target_x_min <= x <= target_x_y

a = -1
dv/dt = -1
v = v_0 - t
dx/dt = v_0 - t
x = v_0*t - t^2/2
7 - 1/2 = 6.5
12 - 2 = 10


max_s = 1/2 (u * (u + 1))
diff_ut = abs(u - t)
s = max_s - (1/2 (diff_ut * (diff_ut + 1))) (in case of x, s = max_s after t = u)
2s = (u(u + 1)) - (abs(u - t)(abs(u - t) + 1))


x = 1/2 * (v_0 * (v_0 + 1)  -  (v_0 - t) * (v_0 - t + 1))

1/2 * (v_0^2 + v_0  -  v_0^2 + 2 * v_0t - t^2 - v_0 + t)

1/2 * (2 * v_0t - t^2 + t)

(when v_0 = t) max_x = 1/2 * (v_0 * (v_0 + 1))


e.g. when v_0 = 7
at t = 1, x = (14 - 1 + 1) / 2 = 7
at t = 2, x = (28 - 4 + 2) / 2 = 13
at t = 3, x = (14 - 1 + 1) / 2 = 7
at t = 4, x = (28 - 4 + 2) / 2 = 13

min_x < t * (2v_0 - t + 1) / 2
2min_x < t(2v_0 - t + 1)
0 <= -t^2 + (2v_0 + 1)t - 2min_x
0 >= t^2 - (2v_0 + 1)t + 2min_x

2max_x >= t(2v_0 - t + 1)
0 >= -t^2 + (2v_0 + 1)t - 2max_x
0 <= t^2 - (2v_0 + 1)t + 2max_x

Inequalities:
0 >= t^2 - (2v_0 + 1)t + 2min_x
0 <= t^2 - (2v_0 + 1)t + 2max_x

simplified to
t^2 - bt + 2c = 0 (where b = 2v_0 + 1 and c = min_x or max_x)
factored would be (t + 1)(t - 2)

solutions are
t = ((2v_0 + 1) +- sqrt((2v_0 + 1) ^ 2 - 8k)) / 4

or t = (-b +- sqrt(b^2 - 8c)) / 4
(as long as b^2 > 8c this has real solutions)
i.e. as long as (2v_0 + 1) ^ 2 > 8 * min_x and max_x
or c < (2v_0 + 1)^2 / 8


t = (-b +- sqrt(b^2 - 8c)) / 4 = (+-sqrt(b^2 - 8c) - b) / 4

The negative solution here, (-sqrt(b^2 - 8c) - b) / 4 is uninteresting to us because we are not interested in t < 0.
So only looking at the positive solution, t = (sqrt(b^2 - 8c) - b) / 4
Plugging this back into the quadratic formula,
((sqrt(b^2 - 8c) - b) / 4) ^ 2 - b((sqrt(b^2 - 8c) - b) / 4) + 2c
((sqrt(b^2 - 8c) - b) ^ 2 / 16) - b*sqrt(b^2 - 8c)/4 - b^2/4 + 2c

((b^2 - 8c) - 2bsqrt(b^2 - 8c) + b^2) / 16 - b*sqrt(b^2 - 8c)/4 - b^2/4 + 2c
simplifying sqrt(b^2 - 8c) to d for time being,
((b^2 - 8c) - 2bd + b^2) / 16 - bd/4 - b^2/4 + 2c
((b^2 - 8c) - 2bd + b^2) - 4bd - 4b^2 + 32c
b^2 - 6bd - 3b^2 + 24c
24c - 6bd - 2b^2
expanding d back out,
24c - 6b*sqrt(b^2 - 8c) - 2b^2
12c - 3b*sqrt(b^2 - 8c) - b^2

and therefore,
12min_x - 3b*sqrt(b^2 - 8min_x) - b^2 <= 0
12max_x - 3b*sqrt(b^2 - 8max_x) - b^2 >= 0

e.g. for min_x = -10, max_x = -5
-120 - 3b*sqrt(b^2 + 80) - b^2 <= 0
 -60 - 3b*sqrt(b^2 + 40) - b^2 >= 0
where b = 2v_0 + 1
and b^2 therefore = 4v_0^2 + 4v_0 + 1
-120 - (12v_0^2 + 12v_0 + 3)*sqrt(4v_0^2 + 4v_0 + 81) - 4v_0^2 - 4v_0 - 1 <= 0



(20 - t + 1) * t / 2 = 0
20 - t + 1 = 0
t = 21

(2v - t + 1) * t / 2 >= min
(2v - t + 1) * t / 2 <= max

2v - t + 1 >= 2min / t
2v >= t - 1 + 2min / t
v >= (t - 1) / 2 + min / t

2v - t + 1 <= 2max / t
v <= (t - 1) / 2 + max / t



(20 - t + 1) * t / 2 >= -10
(20 - t + 1) * t / 2 <= -5

(200 - t + 1)t / 2 = 0
t = 20

(2 * vx - 201 + 1) * 201 / 2
(2vx - 200) * 201 / 2 <= 30
(2vx - 200) * 201 <= 60
2vx - 200 <= 60 / 201
vx - 100 <= 30 / 201
vx <= 100 + 30 / 201
vx <= ~100.1

(2vx - 200) * 201 / 2 >= 20
(2vx - 200) * 201 >= 40
2vx - 200 >= 40 / 201
vx - 100 >= 10 / 201
vx >= 100 + 10 / 201
vx >= 10.476

(20 


1/2 AT^2 + UT - S = 0
AT^2 + 2UT - 2S = 0
(-(2U) +- sqrt((2U)^2 - 4A(-2S))) / 2A
(-2U +- sqrt(4U^2 + 8AS)) / 2A
(-2U +- sqrt(4(U^2 + 2AS))) / 2A
(-2U +- 2sqrt(U^2 + 2AS)) / 2A
(-U +- sqrt(U^2 + 2AS)) / A

S = UT + 1/2 AT^2
S = UT - 1/2 T^2
T^2 - 2UT + 2S = 0
