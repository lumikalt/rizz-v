    li a0 5
    li a1 1

factorial:
    beqz a0 end
    mul a1 a1 a0
    addi a0 a0 -1
    bneqz factorial
end:
