	.word 10
msg:
start:
	mov r6, 0 ; move 0 into register 6
	mov r4, 1 ; move 1 into register 4
	push r6
	push r4 ; push initial Fibonacci values onto call stack
	int 8
	int 40
	cmp r0, 1
	cmp r0, 2
	mov r1, 0
	mov r5, 0 ; clear register 5
	pop r4
	pop r6 ; retrieve values
	add r5, r4 ; Fibonacci computation
	add r5, r6
	mov r6, r4
	mov r4, r5
	push r6
	push r4 ; push updated Fibonacci values back onto stack
	cmp r0, r2
	int 5
	add r1, 1
	add r2, 1
finish:
	pop r4
	div r7, r6 ; golden ratio
	int 8
	int 7
	hlt
	int 8
	hlt
