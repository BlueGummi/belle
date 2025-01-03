.ssp [40]  ; set stack pointer to addr 49
.sbp [40]  ; set base pointer to addr 49
.start [41] ; program starts at addr 50
	jmp @start
msg:
    .asciiz "The golden ratio is: "
msg_end:
    .asciiz "Enter how many numbers to calculate: "
start:
	mov r6, 0 ; move 0 into register 6
	mov r4, 1 ; move 1 into register 7
	push r6
	push r4 ; push initial Fibonacci values onto call stack
	mov r0, @msg_end
	mov r1, @start
	int 8
	int 40
	mov r1, 0
	jmp @fib_loop ; perform a jump into the loop
fib_loop:
	pop $140
	mov r5, 0 ; clear register 5
	pop r4
	pop r6 ; retrieve values
	add r5, r4 ; Fibonacci computation
	add r5, r6
	mov r6, r4
	mov r4, r5
	push r6
	push r4 ; push updated Fibonacci values back onto stack
	jo @finish ; jump if a value overflowed
	cmp r0, r2
	jz @finish
	int 5
	st &r1, r5
	add r1, 1
	add r2, 1
	jmp @fib_loop ; continue Fibonacci calculation
finish:
	add r1, -2
	mov r7, &r1 ; get back most recent value
	add r1, -1
	mov r6, &r1 ; get back second most recent value 
	div r7, r6 ; golden ratio
	mov r0, @msg
	mov r1, @msg_end
	int 8
	int 7
	hlt
