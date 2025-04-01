# addi, R(1) = R(0) + 1
li r1, 1
# addi, R(2) = R(0) + 6
li r2, 6
# addi, R(3) = R(0) + 1
li r3, 1
loop:
# mul, R(1) = R(1) * R(3)
mul r1, r1, r3
# addi, R(3) = R(3) + 1
addi r3, r3, 1
# bne, if (R(3) != R(2)) pc += -0x8
bne r3, r2, loop
# halt
li r0, 0
