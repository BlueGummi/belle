	mov r5, 31
	mov r4, 7
	mul r5, r4 ; 31 x 7 is in r5
	mov r4, 100
	add r4, 51 ; 151
	mul r5, r4
	mul r5, 2
	mov r4, 0
	pop r6 ; pop off return address
	add r4, 1
	cmp r4, r5
	pop r6 ; pop off that return address
	cmp r4, r3
	pop r6
	add r7, 1
	cmp r7, 48
	pop r6
	add r2, 1
	cmp r2, 48
print:
	add r3, 1
	cmp r3, 1 ; change this number to determine how many seconds the program takes to execute
	mov r2, 0
	mov r7, 0
	ret
end:
	hlt
