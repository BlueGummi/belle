.start $300
<<<<<<< HEAD
	mov %r5, #31
	mov %r4, #7
	mul %r5, %r4 ; 31 x 7 is in r5
	mov %r4, #100
	add %r4, #51 ; 151
	mul %r5, %r4
	int #11
	jz @loop
loop:
	add %r0, #1
	int #0
	cmp %r0, %r5
	int #13
	jz @loop
	int #11
	jz @sub_loop
	ret
sub_loop:
	add %r0, #-1
	int #0
	cmp %r0, %r1
	int #13
	jz @sub_loop
	int #11
	jz @loop
	ret
=======
		mov %r5, #31
		mov %r4, #7

		mov %r4, #100

		mul %r5, %r4
		set #1
		jz @loop
loop:
		add %r0, #-1
		int #0
		cmp %r0, %r5
		int #11
		jz @loop
		set #1
		jz @sub_loop
		ret
sub_loop:
		add %r0, #-1
		int #0
		cmp %r0, %r1
		int #11
		jz @sub_loop
		set #1
		jz @loop
		ret
>>>>>>> 8ff5f48a34ec8fe5530eb48126ea28002ff95b58
