# r0 - register for trash (checks that produce no value)
# r1 - address of start of array
# r2 - address of end of array
# r3 - address of last element
subq r3 <- r2, 1
subq r1 <- r1, 1
outer_loop:
addq r1 <- r1, 1
xor r0 <- r1, r3
# Jump if zero
brq-9 outer_loop_exit
# r4 - j index (copy value of r1)
addq r4 <- r1, 0
inner_loop:
addq r4 <- r4, 1
xor r0 <- r4, r2
# Jump if zero
brq-9 outer_loop
# r5 = *r1
# r6 = *r4
ldq r5 <- r1, 0
ldq r6 <- r4, 0
sub r0 <- r5, r6
# Jump if left less then right
brq-10 inner_loop
stq r5 -> r4, 0
stq r6 -> r1, 0
# Unconditional jump
brq-0 inner_loop
#
outer_loop_exit: