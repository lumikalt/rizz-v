    li a0 55743235
    li a1 1

# 5!
factorial:
    beqz a0 end
    mul a1 a1 a0
    addi a0 a0 -1
    j factorial

end:
    nop
