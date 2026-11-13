	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.private_extern	__ZN3std2rt10lang_start17h371def2b53aace49E
	.globl	__ZN3std2rt10lang_start17h371def2b53aace49E
	.p2align	2
__ZN3std2rt10lang_start17h371def2b53aace49E:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x8, x0
	str	x1, [sp, #8]
	mov	x0, x2
	ldr	x2, [sp, #8]
	str	x0, [sp, #16]
	mov	x4, x3
	ldr	x3, [sp, #16]
	sub	x0, x29, #8
	stur	x8, [x29, #-8]
	adrp	x1, l_anon.ab3f16de80b9391f4c6c1b3ef9659191.0@PAGE
	add	x1, x1, l_anon.ab3f16de80b9391f4c6c1b3ef9659191.0@PAGEOFF
	bl	__ZN3std2rt19lang_start_internal17h48672da3ca81ba76E
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h1371405820f64b54E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__ZN3std3sys9backtrace28__rust_begin_short_backtrace17h6ab42c08998eab18E
	bl	__ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hedcf0ded32917458E
	and	w0, w0, #0xff
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN3std3sys9backtrace28__rust_begin_short_backtrace17h6ab42c08998eab18E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN4core3ops8function6FnOnce9call_once17h795afa561e8d2424E
	; InlineAsm Start
	; InlineAsm End
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3fmt2rt8Argument11new_display17hc42019839acfc02aE:
	.cfi_startproc
	sub	sp, sp, #16
	.cfi_def_cfa_offset 16
	str	x0, [sp]
	adrp	x9, __ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i64$GT$3fmt17h3e342cb7627a056eE@GOTPAGE
	ldr	x9, [x9, __ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i64$GT$3fmt17h3e342cb7627a056eE@GOTPAGEOFF]
	str	x9, [sp, #8]
	ldr	q0, [sp]
	str	q0, [x8]
	add	sp, sp, #16
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3fmt9Arguments3new17hcc4f3cea23b7568eE:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17habb07ae265f553ebE:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__ZN4core3ops8function6FnOnce9call_once17hfd721ac84653d86dE
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ops8function6FnOnce9call_once17h795afa561e8d2424E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	blr	x0
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ops8function6FnOnce9call_once17hfd721ac84653d86dE:
Lfunc_begin0:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception0
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x8, x0
	add	x0, sp, #16
	str	x8, [sp, #16]
Ltmp1:
	bl	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h1371405820f64b54E
	str	w0, [sp, #12]
Ltmp2:
	b	LBB7_3
LBB7_1:
	ldur	x0, [x29, #-16]
	bl	__Unwind_Resume
LBB7_2:
Ltmp3:
	stur	x0, [x29, #-16]
	mov	x8, x1
	stur	w8, [x29, #-8]
	b	LBB7_1
LBB7_3:
	ldr	w0, [sp, #12]
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
Lfunc_end0:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table7:
Lexception0:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end0-Lcst_begin0
Lcst_begin0:
	.uleb128 Ltmp1-Lfunc_begin0
	.uleb128 Ltmp2-Ltmp1
	.uleb128 Ltmp3-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp2-Lfunc_begin0
	.uleb128 Lfunc_end0-Ltmp2
	.byte	0
	.byte	0
Lcst_end0:
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.private_extern	__ZN4main4main17h59ae6b2ff26e2dfdE
	.globl	__ZN4main4main17h59ae6b2ff26e2dfdE
	.p2align	2
__ZN4main4main17h59ae6b2ff26e2dfdE:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	mov	w8, #25
	adds	x8, x8, #35
	str	x8, [sp]
	cset	w8, vs
	tbnz	w8, #0, LBB8_2
	b	LBB8_1
LBB8_1:
	ldr	x8, [sp]
	add	x0, sp, #8
	str	x8, [sp, #8]
	sub	x8, x29, #16
	bl	__ZN4core3fmt2rt8Argument11new_display17hc42019839acfc02aE
	ldur	q0, [x29, #-16]
	add	x1, sp, #16
	str	q0, [sp, #16]
	adrp	x0, l_anon.ab3f16de80b9391f4c6c1b3ef9659191.3@PAGE
	add	x0, x0, l_anon.ab3f16de80b9391f4c6c1b3ef9659191.3@PAGEOFF
	bl	__ZN4core3fmt9Arguments3new17hcc4f3cea23b7568eE
	bl	__ZN3std2io5stdio6_print17h171e44f9366e5f4bE
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB8_2:
	.cfi_restore_state
	adrp	x0, l_anon.ab3f16de80b9391f4c6c1b3ef9659191.2@PAGE
	add	x0, x0, l_anon.ab3f16de80b9391f4c6c1b3ef9659191.2@PAGEOFF
	bl	__ZN4core9panicking11panic_const24panic_const_add_overflow17h176493eeac452e6fE
	.cfi_endproc

	.p2align	2
__ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hedcf0ded32917458E:
	.cfi_startproc
	mov	w0, #0
	ret
	.cfi_endproc

	.globl	_main
	.p2align	2
_main:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x2, x1
	mov	x8, x0
	sxtw	x1, w8
	adrp	x0, __ZN4main4main17h59ae6b2ff26e2dfdE@PAGE
	add	x0, x0, __ZN4main4main17h59ae6b2ff26e2dfdE@PAGEOFF
	mov	w3, #0
	bl	__ZN3std2rt10lang_start17h371def2b53aace49E
	ldp	x29, x30, [sp], #16
	ret
	.cfi_endproc

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.ab3f16de80b9391f4c6c1b3ef9659191.0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17habb07ae265f553ebE
	.quad	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h1371405820f64b54E
	.quad	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h1371405820f64b54E

	.section	__TEXT,__cstring,cstring_literals
l_anon.ab3f16de80b9391f4c6c1b3ef9659191.1:
	.asciz	"main.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.ab3f16de80b9391f4c6c1b3ef9659191.2:
	.quad	l_anon.ab3f16de80b9391f4c6c1b3ef9659191.1
	.asciz	"\007\000\000\000\000\000\000\000\005\000\000\000\022\000\000"

	.section	__TEXT,__cstring,cstring_literals
l_anon.ab3f16de80b9391f4c6c1b3ef9659191.3:
	.asciz	"\300\001\n"

.subsections_via_symbols
