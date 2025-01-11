	push 2
begin:
	pop r5 ; avoiding overflow
	add r3, 1 ; accumulator
	cmp r3, 30
next:	
	mov r3, 10
	mov r0, 0
	mov r1, 50
	int 8
	int 9
	int 12
	int 0
	int 12
	pop r4
	pop r4
	mov r1, 8
done:
	hlt
