    li a0 5
    li a1 1
    lui a2 0x3f800000

    add a2 a0 a1

    fcvt.s.w fa0 a0
    fcvt.s.w fa1 a1

    fadd.s fa2 fa0 fa1
    fdiv.s fa3 fa2 fa0
