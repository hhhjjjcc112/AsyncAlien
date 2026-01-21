
../target/x86_64-unknown-none/release/x86_64-boot:	file format elf64-x86-64

Disassembly of section .text:

ffff800000200000 <.text>:
ffff800000200000: 89 c7                	mov	edi, eax
ffff800000200002: 89 de                	mov	esi, ebx
ffff800000200004: eb 22                	jmp	0xffff800000200028 <.text+0x28>
ffff800000200006: 66 90                	nop
ffff800000200008: 02 b0 ad 1b 02 00    	add	dh, byte ptr [rax + 0x21bad]
ffff80000020000e: 01 00                	add	dword ptr [rax], eax
ffff800000200010: fc                   	cld
ffff800000200011: 4f 51                	push	r9
ffff800000200013: e4 08                	in	al, 0x8
ffff800000200015: 00 20                	add	byte ptr [rax], ah
ffff800000200017: 00 00                	add	byte ptr [rax], al
ffff800000200019: 00 20                	add	byte ptr [rax], ah
ffff80000020001b: 00 00                	add	byte ptr [rax], al
ffff80000020001d: 80 20 00             	and	byte ptr [rax], 0x0
ffff800000200020: 00 80 24 00 00 00    	add	byte ptr [rax + 0x24], al
ffff800000200026: 20 00                	and	byte ptr [rax], al
ffff800000200028: 0f 01 15 00 20 20 00 	lgdt	[rip + 0x202000]        # 0xffff80000040202f
ffff80000020002f: 66 b8 18 00          	mov	ax, 0x18
ffff800000200033: 66 8e d0             	mov	ss, ax
ffff800000200036: 66 8e d8             	mov	ds, ax
ffff800000200039: 66 8e c0             	mov	es, ax
ffff80000020003c: 66 8e e0             	mov	fs, ax
ffff80000020003f: 66 8e e8             	mov	gs, ax
ffff800000200042: b8 a0 00 00 00       	mov	eax, 0xa0
ffff800000200047: 0f 22 e0             	mov	cr4, rax
ffff80000020004a: 8d 05 00 40 20 00    	lea	eax, [rip + 0x204000]   # 0xffff800000404050
ffff800000200050: 0f 22 d8             	mov	cr3, rax
ffff800000200053: b9 80 00 00 c0       	mov	ecx, 0xc0000080
ffff800000200058: ba 00 00 00 00       	mov	edx, 0x0
ffff80000020005d: b8 00 09 00 00       	mov	eax, 0x900
ffff800000200062: 0f 30                	wrmsr
ffff800000200064: b8 23 00 01 80       	mov	eax, 0x80010023
ffff800000200069: 0f 22 c0             	mov	cr0, rax
ffff80000020006c: ea                   	<unknown>
ffff80000020006d: b7 00                	mov	bh, 0x0
ffff80000020006f: 20 00                	and	byte ptr [rax], al
ffff800000200071: 10 00                	adc	byte ptr [rax], al
ffff800000200073: 66 b8 18 00          	mov	ax, 0x18
ffff800000200077: 66 8e d0             	mov	ss, ax
ffff80000020007a: 66 8e d8             	mov	ds, ax
ffff80000020007d: 66 8e c0             	mov	es, ax
ffff800000200080: 66 8e e0             	mov	fs, ax
ffff800000200083: 66 8e e8             	mov	gs, ax
ffff800000200086: b8 a0 00 00 00       	mov	eax, 0xa0
ffff80000020008b: 0f 22 e0             	mov	cr4, rax
ffff80000020008e: 8d 05 00 40 20 00    	lea	eax, [rip + 0x204000]   # 0xffff800000404094
ffff800000200094: 0f 22 d8             	mov	cr3, rax
ffff800000200097: b9 80 00 00 c0       	mov	ecx, 0xc0000080
ffff80000020009c: ba 00 00 00 00       	mov	edx, 0x0
ffff8000002000a1: b8 00 09 00 00       	mov	eax, 0x900
ffff8000002000a6: 0f 30                	wrmsr
ffff8000002000a8: b8 23 00 01 80       	mov	eax, 0x80010023
ffff8000002000ad: 0f 22 c0             	mov	cr0, rax
ffff8000002000b0: ea                   	<unknown>
ffff8000002000b1: e8 00 20 00 10       	call	0xffff8000102020b6
ffff8000002000b6: 00 66 31             	add	byte ptr [rsi + 0x31], ah
ffff8000002000b9: c0 66 8e d0          	shl	byte ptr [rsi - 0x72], 0xd0
ffff8000002000bd: 66 8e d8             	mov	ds, ax
ffff8000002000c0: 66 8e c0             	mov	es, ax
ffff8000002000c3: 66 8e e0             	mov	fs, ax
ffff8000002000c6: 66 8e e8             	mov	gs, ax
ffff8000002000c9: 48 bc 00 80 20 00 00 80 ff ff	movabs	rsp, -0x7fffffdf8000
ffff8000002000d3: 48 81 c4 00 00 04 00 	add	rsp, 0x40000
ffff8000002000da: 48 b8 10 04 20 00 00 80 ff ff	movabs	rax, -0x7fffffdffbf0
ffff8000002000e4: ff d0                	call	rax
ffff8000002000e6: eb 34                	jmp	0xffff80000020011c <.text+0x11c>
ffff8000002000e8: 66 31 c0             	xor	ax, ax
ffff8000002000eb: 66 8e d0             	mov	ss, ax
ffff8000002000ee: 66 8e d8             	mov	ds, ax
ffff8000002000f1: 66 8e c0             	mov	es, ax
ffff8000002000f4: 66 8e e0             	mov	fs, ax
ffff8000002000f7: 66 8e e8             	mov	gs, ax
ffff8000002000fa: 48 b8 00 00 00 00 00 80 ff ff	movabs	rax, -0x800000000000
ffff800000200104: 48 01 c4             	add	rsp, rax
ffff800000200107: 48 c7 c7 02 b0 ad 2b 	mov	rdi, 0x2badb002
ffff80000020010e: 48 b8 d0 06 20 00 00 80 ff ff	movabs	rax, -0x7fffffdff930
ffff800000200118: ff d0                	call	rax
ffff80000020011a: eb 00                	jmp	0xffff80000020011c <.text+0x11c>
ffff80000020011c: f4                   	hlt
ffff80000020011d: eb fd                	jmp	0xffff80000020011c <.text+0x11c>
ffff80000020011f: cc                   	int3
ffff800000200120: 55                   	push	rbp
ffff800000200121: 48 89 e5             	mov	rbp, rsp
ffff800000200124: 41 57                	push	r15
ffff800000200126: 41 56                	push	r14
ffff800000200128: 41 55                	push	r13
ffff80000020012a: 41 54                	push	r12
ffff80000020012c: 53                   	push	rbx
ffff80000020012d: 48 83 ec 68          	sub	rsp, 0x68
ffff800000200131: 48 8b 07             	mov	rax, qword ptr [rdi]
ffff800000200134: 48 8b 08             	mov	rcx, qword ptr [rax]
ffff800000200137: 48 89 4d a0          	mov	qword ptr [rbp - 0x60], rcx
ffff80000020013b: 4c 8b 60 08          	mov	r12, qword ptr [rax + 0x8]
ffff80000020013f: 48 8b 1e             	mov	rbx, qword ptr [rsi]
ffff800000200142: 4c 8b 76 08          	mov	r14, qword ptr [rsi + 0x8]
ffff800000200146: 4d 8b 6e 18          	mov	r13, qword ptr [r14 + 0x18]
ffff80000020014a: 48 8d 35 9a 20 00 00 	lea	rsi, [rip + 0x209a]     # 0xffff8000002021eb
ffff800000200151: ba 0c 00 00 00       	mov	edx, 0xc
ffff800000200156: 48 89 df             	mov	rdi, rbx
ffff800000200159: 41 ff d5             	call	r13
ffff80000020015c: 41 b7 01             	mov	r15b, 0x1
ffff80000020015f: 84 c0                	test	al, al
ffff800000200161: 0f 85 da 00 00 00    	jne	0xffff800000200241 <.text+0x241>
ffff800000200167: 49 8d 44 24 10       	lea	rax, [r12 + 0x10]
ffff80000020016c: 4c 89 65 a8          	mov	qword ptr [rbp - 0x58], r12
ffff800000200170: 49 83 c4 14          	add	r12, 0x14
ffff800000200174: 48 8d 0d f5 0c 00 00 	lea	rcx, [rip + 0xcf5]      # 0xffff800000200e70 <.text+0xe70>
ffff80000020017b: 48 89 4d b0          	mov	qword ptr [rbp - 0x50], rcx
ffff80000020017f: 48 89 45 b8          	mov	qword ptr [rbp - 0x48], rax
ffff800000200183: 48 8d 05 e6 05 00 00 	lea	rax, [rip + 0x5e6]      # 0xffff800000200770 <.text+0x770>
ffff80000020018a: 48 89 45 c0          	mov	qword ptr [rbp - 0x40], rax
ffff80000020018e: 4c 89 65 c8          	mov	qword ptr [rbp - 0x38], r12
ffff800000200192: 48 89 45 d0          	mov	qword ptr [rbp - 0x30], rax
ffff800000200196: 48 8d 05 6b 6f 00 00 	lea	rax, [rip + 0x6f6b]     # 0xffff800000207108
ffff80000020019d: 48 89 85 70 ff ff ff 	mov	qword ptr [rbp - 0x90], rax
ffff8000002001a4: 48 c7 85 78 ff ff ff 03 00 00 00     	mov	qword ptr [rbp - 0x88], 0x3
ffff8000002001af: 48 c7 45 90 00 00 00 00      	mov	qword ptr [rbp - 0x70], 0x0
ffff8000002001b7: 48 8d 45 a8          	lea	rax, [rbp - 0x58]
ffff8000002001bb: 48 89 45 80          	mov	qword ptr [rbp - 0x80], rax
ffff8000002001bf: 48 c7 45 88 03 00 00 00      	mov	qword ptr [rbp - 0x78], 0x3
ffff8000002001c7: 48 8d 95 70 ff ff ff 	lea	rdx, [rbp - 0x90]
ffff8000002001ce: 48 89 df             	mov	rdi, rbx
ffff8000002001d1: 4c 89 f6             	mov	rsi, r14
ffff8000002001d4: e8 57 07 00 00       	call	0xffff800000200930 <.text+0x930>
ffff8000002001d9: 84 c0                	test	al, al
ffff8000002001db: 75 64                	jne	0xffff800000200241 <.text+0x241>
ffff8000002001dd: 48 8d 35 13 20 00 00 	lea	rsi, [rip + 0x2013]     # 0xffff8000002021f7
ffff8000002001e4: ba 02 00 00 00       	mov	edx, 0x2
ffff8000002001e9: 48 89 df             	mov	rdi, rbx
ffff8000002001ec: 41 ff d5             	call	r13
ffff8000002001ef: 84 c0                	test	al, al
ffff8000002001f1: 75 4e                	jne	0xffff800000200241 <.text+0x241>
ffff8000002001f3: 48 8b 55 a0          	mov	rdx, qword ptr [rbp - 0x60]
ffff8000002001f7: 48 8b 42 28          	mov	rax, qword ptr [rdx + 0x28]
ffff8000002001fb: 48 89 45 d0          	mov	qword ptr [rbp - 0x30], rax
ffff8000002001ff: 48 8b 42 20          	mov	rax, qword ptr [rdx + 0x20]
ffff800000200203: 48 89 45 c8          	mov	qword ptr [rbp - 0x38], rax
ffff800000200207: 48 8b 42 18          	mov	rax, qword ptr [rdx + 0x18]
ffff80000020020b: 48 89 45 c0          	mov	qword ptr [rbp - 0x40], rax
ffff80000020020f: 48 8b 42 10          	mov	rax, qword ptr [rdx + 0x10]
ffff800000200213: 48 89 45 b8          	mov	qword ptr [rbp - 0x48], rax
ffff800000200217: 48 8b 0a             	mov	rcx, qword ptr [rdx]
ffff80000020021a: 48 8b 42 08          	mov	rax, qword ptr [rdx + 0x8]
ffff80000020021e: 48 89 45 b0          	mov	qword ptr [rbp - 0x50], rax
ffff800000200222: 48 89 4d a8          	mov	qword ptr [rbp - 0x58], rcx
ffff800000200226: 48 83 f8 01          	cmp	rax, 0x1
ffff80000020022a: 74 03                	je	0xffff80000020022f <.text+0x22f>
ffff80000020022c: 48 85 c0             	test	rax, rax
ffff80000020022f: 48 8d 55 a8          	lea	rdx, [rbp - 0x58]
ffff800000200233: 48 89 df             	mov	rdi, rbx
ffff800000200236: 4c 89 f6             	mov	rsi, r14
ffff800000200239: e8 f2 06 00 00       	call	0xffff800000200930 <.text+0x930>
ffff80000020023e: 41 89 c7             	mov	r15d, eax
ffff800000200241: 44 89 f8             	mov	eax, r15d
ffff800000200244: 48 83 c4 68          	add	rsp, 0x68
ffff800000200248: 5b                   	pop	rbx
ffff800000200249: 41 5c                	pop	r12
ffff80000020024b: 41 5d                	pop	r13
ffff80000020024d: 41 5e                	pop	r14
ffff80000020024f: 41 5f                	pop	r15
ffff800000200251: 5d                   	pop	rbp
ffff800000200252: c3                   	ret
ffff800000200253: cc                   	int3
ffff800000200254: cc                   	int3
ffff800000200255: cc                   	int3
ffff800000200256: cc                   	int3
ffff800000200257: cc                   	int3
ffff800000200258: cc                   	int3
ffff800000200259: cc                   	int3
ffff80000020025a: cc                   	int3
ffff80000020025b: cc                   	int3
ffff80000020025c: cc                   	int3
ffff80000020025d: cc                   	int3
ffff80000020025e: cc                   	int3
ffff80000020025f: cc                   	int3
ffff800000200260: 55                   	push	rbp
ffff800000200261: 48 89 e5             	mov	rbp, rsp
ffff800000200264: 48 83 ec 10          	sub	rsp, 0x10
ffff800000200268: c7 45 fc 00 00 00 00 	mov	dword ptr [rbp - 0x4], 0x0
ffff80000020026f: 81 fe 80 00 00 00    	cmp	esi, 0x80
ffff800000200275: 73 0e                	jae	0xffff800000200285 <.text+0x285>
ffff800000200277: 40 88 75 fc          	mov	byte ptr [rbp - 0x4], sil
ffff80000020027b: ba 01 00 00 00       	mov	edx, 0x1
ffff800000200280: e9 85 00 00 00       	jmp	0xffff80000020030a <.text+0x30a>
ffff800000200285: 89 f0                	mov	eax, esi
ffff800000200287: 81 fe 00 08 00 00    	cmp	esi, 0x800
ffff80000020028d: 73 1b                	jae	0xffff8000002002aa <.text+0x2aa>
ffff80000020028f: c1 e8 06             	shr	eax, 0x6
ffff800000200292: 0c c0                	or	al, -0x40
ffff800000200294: 88 45 fc             	mov	byte ptr [rbp - 0x4], al
ffff800000200297: 40 80 e6 3f          	and	sil, 0x3f
ffff80000020029b: 40 80 ce 80          	or	sil, -0x80
ffff80000020029f: 40 88 75 fd          	mov	byte ptr [rbp - 0x3], sil
ffff8000002002a3: ba 02 00 00 00       	mov	edx, 0x2
ffff8000002002a8: eb 60                	jmp	0xffff80000020030a <.text+0x30a>
ffff8000002002aa: 81 fe 00 00 01 00    	cmp	esi, 0x10000
ffff8000002002b0: 73 27                	jae	0xffff8000002002d9 <.text+0x2d9>
ffff8000002002b2: c1 e8 0c             	shr	eax, 0xc
ffff8000002002b5: 0c e0                	or	al, -0x20
ffff8000002002b7: 88 45 fc             	mov	byte ptr [rbp - 0x4], al
ffff8000002002ba: 89 f0                	mov	eax, esi
ffff8000002002bc: c1 e8 06             	shr	eax, 0x6
ffff8000002002bf: 24 3f                	and	al, 0x3f
ffff8000002002c1: 0c 80                	or	al, -0x80
ffff8000002002c3: 88 45 fd             	mov	byte ptr [rbp - 0x3], al
ffff8000002002c6: 40 80 e6 3f          	and	sil, 0x3f
ffff8000002002ca: 40 80 ce 80          	or	sil, -0x80
ffff8000002002ce: 40 88 75 fe          	mov	byte ptr [rbp - 0x2], sil
ffff8000002002d2: ba 03 00 00 00       	mov	edx, 0x3
ffff8000002002d7: eb 31                	jmp	0xffff80000020030a <.text+0x30a>
ffff8000002002d9: c1 e8 12             	shr	eax, 0x12
ffff8000002002dc: 0c f0                	or	al, -0x10
ffff8000002002de: 88 45 fc             	mov	byte ptr [rbp - 0x4], al
ffff8000002002e1: 89 f0                	mov	eax, esi
ffff8000002002e3: c1 e8 0c             	shr	eax, 0xc
ffff8000002002e6: 24 3f                	and	al, 0x3f
ffff8000002002e8: 0c 80                	or	al, -0x80
ffff8000002002ea: 88 45 fd             	mov	byte ptr [rbp - 0x3], al
ffff8000002002ed: 89 f0                	mov	eax, esi
ffff8000002002ef: c1 e8 06             	shr	eax, 0x6
ffff8000002002f2: 24 3f                	and	al, 0x3f
ffff8000002002f4: 0c 80                	or	al, -0x80
ffff8000002002f6: 88 45 fe             	mov	byte ptr [rbp - 0x2], al
ffff8000002002f9: 40 80 e6 3f          	and	sil, 0x3f
ffff8000002002fd: 40 80 ce 80          	or	sil, -0x80
ffff800000200301: 40 88 75 ff          	mov	byte ptr [rbp - 0x1], sil
ffff800000200305: ba 04 00 00 00       	mov	edx, 0x4
ffff80000020030a: 48 8d 75 fc          	lea	rsi, [rbp - 0x4]
ffff80000020030e: e8 fd 10 00 00       	call	0xffff800000201410 <.text+0x1410>
ffff800000200313: 31 c0                	xor	eax, eax
ffff800000200315: 48 83 c4 10          	add	rsp, 0x10
ffff800000200319: 5d                   	pop	rbp
ffff80000020031a: c3                   	ret
ffff80000020031b: cc                   	int3
ffff80000020031c: cc                   	int3
ffff80000020031d: cc                   	int3
ffff80000020031e: cc                   	int3
ffff80000020031f: cc                   	int3
ffff800000200320: 55                   	push	rbp
ffff800000200321: 48 89 e5             	mov	rbp, rsp
ffff800000200324: 48 89 f2             	mov	rdx, rsi
ffff800000200327: 48 8d 35 d2 6c 00 00 	lea	rsi, [rip + 0x6cd2]     # 0xffff800000207000
ffff80000020032e: 5d                   	pop	rbp
ffff80000020032f: e9 fc 05 00 00       	jmp	0xffff800000200930 <.text+0x930>
ffff800000200334: cc                   	int3
ffff800000200335: cc                   	int3
ffff800000200336: cc                   	int3
ffff800000200337: cc                   	int3
ffff800000200338: cc                   	int3
ffff800000200339: cc                   	int3
ffff80000020033a: cc                   	int3
ffff80000020033b: cc                   	int3
ffff80000020033c: cc                   	int3
ffff80000020033d: cc                   	int3
ffff80000020033e: cc                   	int3
ffff80000020033f: cc                   	int3
ffff800000200340: 55                   	push	rbp
ffff800000200341: 48 89 e5             	mov	rbp, rsp
ffff800000200344: 48 8b 3e             	mov	rdi, qword ptr [rsi]
ffff800000200347: 48 8b 46 08          	mov	rax, qword ptr [rsi + 0x8]
ffff80000020034b: 48 8b 40 18          	mov	rax, qword ptr [rax + 0x18]
ffff80000020034f: 48 8d 35 d2 1c 00 00 	lea	rsi, [rip + 0x1cd2]     # 0xffff800000202028
ffff800000200356: ba 05 00 00 00       	mov	edx, 0x5
ffff80000020035b: 5d                   	pop	rbp
ffff80000020035c: ff e0                	jmp	rax
ffff80000020035e: cc                   	int3
ffff80000020035f: cc                   	int3
ffff800000200360: 55                   	push	rbp
ffff800000200361: 48 89 e5             	mov	rbp, rsp
ffff800000200364: 53                   	push	rbx
ffff800000200365: 48 83 ec 58          	sub	rsp, 0x58
ffff800000200369: 48 89 7d e8          	mov	qword ptr [rbp - 0x18], rdi
ffff80000020036d: 48 8d 45 e8          	lea	rax, [rbp - 0x18]
ffff800000200371: 48 89 45 d8          	mov	qword ptr [rbp - 0x28], rax
ffff800000200375: 48 8d 05 a4 fd ff ff 	lea	rax, [rip - 0x25c]      # 0xffff800000200120 <.text+0x120>
ffff80000020037c: 48 89 45 e0          	mov	qword ptr [rbp - 0x20], rax
ffff800000200380: 48 8d 05 e9 6c 00 00 	lea	rax, [rip + 0x6ce9]     # 0xffff800000207070
ffff800000200387: 48 89 45 a8          	mov	qword ptr [rbp - 0x58], rax
ffff80000020038b: 48 c7 45 b0 02 00 00 00      	mov	qword ptr [rbp - 0x50], 0x2
ffff800000200393: 48 c7 45 c8 00 00 00 00      	mov	qword ptr [rbp - 0x38], 0x0
ffff80000020039b: 48 8d 45 d8          	lea	rax, [rbp - 0x28]
ffff80000020039f: 48 89 45 b8          	mov	qword ptr [rbp - 0x48], rax
ffff8000002003a3: 48 c7 45 c0 01 00 00 00      	mov	qword ptr [rbp - 0x40], 0x1
ffff8000002003ab: 9c                   	pushfq
ffff8000002003ac: 5b                   	pop	rbx
ffff8000002003ad: fa                   	cli
ffff8000002003ae: b1 01                	mov	cl, 0x1
ffff8000002003b0: 31 c0                	xor	eax, eax
ffff8000002003b2: f0                   	lock
ffff8000002003b3: 0f b0 0d 76 6c 00 00 	cmpxchg	byte ptr [rip + 0x6c76], cl # 0xffff800000207030
ffff8000002003ba: 75 06                	jne	0xffff8000002003c2 <.text+0x3c2>
ffff8000002003bc: eb 11                	jmp	0xffff8000002003cf <.text+0x3cf>
ffff8000002003be: 66 90                	nop
ffff8000002003c0: f3 90                	pause
ffff8000002003c2: 0f b6 05 67 6c 00 00 	movzx	eax, byte ptr [rip + 0x6c67] # 0xffff800000207030
ffff8000002003c9: 84 c0                	test	al, al
ffff8000002003cb: 75 f3                	jne	0xffff8000002003c0 <.text+0x3c0>
ffff8000002003cd: eb e1                	jmp	0xffff8000002003b0 <.text+0x3b0>
ffff8000002003cf: 48 8d 3d 5c 6c 00 00 	lea	rdi, [rip + 0x6c5c]     # 0xffff800000207032
ffff8000002003d6: 48 8d 35 23 6c 00 00 	lea	rsi, [rip + 0x6c23]     # 0xffff800000207000
ffff8000002003dd: 48 8d 55 a8          	lea	rdx, [rbp - 0x58]
ffff8000002003e1: e8 4a 05 00 00       	call	0xffff800000200930 <.text+0x930>
ffff8000002003e6: 84 c0                	test	al, al
ffff8000002003e8: 75 1a                	jne	0xffff800000200404 <.text+0x404>
ffff8000002003ea: c6 05 3f 6c 00 00 00 	mov	byte ptr [rip + 0x6c3f], 0x0 # 0xffff800000207030
ffff8000002003f1: f7 c3 00 02 00 00    	test	ebx, 0x200
ffff8000002003f7: 75 03                	jne	0xffff8000002003fc <.text+0x3fc>
ffff8000002003f9: fa                   	cli
ffff8000002003fa: eb 04                	jmp	0xffff800000200400 <.text+0x400>
ffff8000002003fc: fb                   	sti
ffff8000002003fd: 0f 1f 00             	nop	dword ptr [rax]
ffff800000200400: f3 90                	pause
ffff800000200402: eb fc                	jmp	0xffff800000200400 <.text+0x400>
ffff800000200404: 48 8d 7d f7          	lea	rdi, [rbp - 0x9]
ffff800000200408: e8 73 04 00 00       	call	0xffff800000200880 <.text+0x880>
ffff80000020040d: cc                   	int3
ffff80000020040e: cc                   	int3
ffff80000020040f: cc                   	int3
ffff800000200410: 55                   	push	rbp
ffff800000200411: 48 89 e5             	mov	rbp, rsp
ffff800000200414: 41 57                	push	r15
ffff800000200416: 41 56                	push	r14
ffff800000200418: 41 54                	push	r12
ffff80000020041a: 53                   	push	rbx
ffff80000020041b: 48 83 ec 60          	sub	rsp, 0x60
ffff80000020041f: 48 89 7d a0          	mov	qword ptr [rbp - 0x60], rdi
ffff800000200423: 48 89 75 88          	mov	qword ptr [rbp - 0x78], rsi
ffff800000200427: 9c                   	pushfq
ffff800000200428: 41 58                	pop	r8
ffff80000020042a: fa                   	cli
ffff80000020042b: b1 01                	mov	cl, 0x1
ffff80000020042d: 0f 1f 00             	nop	dword ptr [rax]
ffff800000200430: 31 c0                	xor	eax, eax
ffff800000200432: f0                   	lock
ffff800000200433: 0f b0 0d f6 6b 00 00 	cmpxchg	byte ptr [rip + 0x6bf6], cl # 0xffff800000207030
ffff80000020043a: 75 06                	jne	0xffff800000200442 <.text+0x442>
ffff80000020043c: eb 11                	jmp	0xffff80000020044f <.text+0x44f>
ffff80000020043e: 66 90                	nop
ffff800000200440: f3 90                	pause
ffff800000200442: 0f b6 05 e7 6b 00 00 	movzx	eax, byte ptr [rip + 0x6be7] # 0xffff800000207030
ffff800000200449: 84 c0                	test	al, al
ffff80000020044b: 75 f3                	jne	0xffff800000200440 <.text+0x440>
ffff80000020044d: eb e1                	jmp	0xffff800000200430 <.text+0x430>
ffff80000020044f: 0f b7 35 dc 6b 00 00 	movzx	esi, word ptr [rip + 0x6bdc] # 0xffff800000207032
ffff800000200456: 8d 4e 01             	lea	ecx, [rsi + 0x1]
ffff800000200459: 31 c0                	xor	eax, eax
ffff80000020045b: 89 ca                	mov	edx, ecx
ffff80000020045d: ee                   	out	dx, al
ffff80000020045e: 8d 7e 03             	lea	edi, [rsi + 0x3]
ffff800000200461: b0 80                	mov	al, -0x80
ffff800000200463: 89 fa                	mov	edx, edi
ffff800000200465: ee                   	out	dx, al
ffff800000200466: b0 03                	mov	al, 0x3
ffff800000200468: 89 f2                	mov	edx, esi
ffff80000020046a: ee                   	out	dx, al
ffff80000020046b: 31 c0                	xor	eax, eax
ffff80000020046d: 89 ca                	mov	edx, ecx
ffff80000020046f: ee                   	out	dx, al
ffff800000200470: b0 03                	mov	al, 0x3
ffff800000200472: 89 fa                	mov	edx, edi
ffff800000200474: ee                   	out	dx, al
ffff800000200475: 8d 56 02             	lea	edx, [rsi + 0x2]
ffff800000200478: b0 c7                	mov	al, -0x39
ffff80000020047a: ee                   	out	dx, al
ffff80000020047b: 83 c6 04             	add	esi, 0x4
ffff80000020047e: b0 0b                	mov	al, 0xb
ffff800000200480: 89 f2                	mov	edx, esi
ffff800000200482: ee                   	out	dx, al
ffff800000200483: b0 01                	mov	al, 0x1
ffff800000200485: 89 ca                	mov	edx, ecx
ffff800000200487: ee                   	out	dx, al
ffff800000200488: c6 05 a1 6b 00 00 00 	mov	byte ptr [rip + 0x6ba1], 0x0 # 0xffff800000207030
ffff80000020048f: 41 f7 c0 00 02 00 00 	test	r8d, 0x200
ffff800000200496: 75 03                	jne	0xffff80000020049b <.text+0x49b>
ffff800000200498: fa                   	cli
ffff800000200499: eb 01                	jmp	0xffff80000020049c <.text+0x49c>
ffff80000020049b: fb                   	sti
ffff80000020049c: 48 81 7d a0 02 b0 ad 2b      	cmp	qword ptr [rbp - 0x60], 0x2badb002
ffff8000002004a4: 0f 85 c9 01 00 00    	jne	0xffff800000200673 <.text+0x673>
ffff8000002004aa: 48 8d 05 df 6b 00 00 	lea	rax, [rip + 0x6bdf]     # 0xffff800000207090
ffff8000002004b1: 48 89 45 a8          	mov	qword ptr [rbp - 0x58], rax
ffff8000002004b5: 48 c7 45 b0 01 00 00 00      	mov	qword ptr [rbp - 0x50], 0x1
ffff8000002004bd: 48 c7 45 c8 00 00 00 00      	mov	qword ptr [rbp - 0x38], 0x0
ffff8000002004c5: 48 c7 45 b8 08 00 00 00      	mov	qword ptr [rbp - 0x48], 0x8
ffff8000002004cd: 48 c7 45 c0 00 00 00 00      	mov	qword ptr [rbp - 0x40], 0x0
ffff8000002004d5: 9c                   	pushfq
ffff8000002004d6: 5b                   	pop	rbx
ffff8000002004d7: fa                   	cli
ffff8000002004d8: b1 01                	mov	cl, 0x1
ffff8000002004da: 66 0f 1f 44 00 00    	nop	word ptr [rax + rax]
ffff8000002004e0: 31 c0                	xor	eax, eax
ffff8000002004e2: f0                   	lock
ffff8000002004e3: 0f b0 0d 46 6b 00 00 	cmpxchg	byte ptr [rip + 0x6b46], cl # 0xffff800000207030
ffff8000002004ea: 75 06                	jne	0xffff8000002004f2 <.text+0x4f2>
ffff8000002004ec: eb 11                	jmp	0xffff8000002004ff <.text+0x4ff>
ffff8000002004ee: 66 90                	nop
ffff8000002004f0: f3 90                	pause
ffff8000002004f2: 0f b6 05 37 6b 00 00 	movzx	eax, byte ptr [rip + 0x6b37] # 0xffff800000207030
ffff8000002004f9: 84 c0                	test	al, al
ffff8000002004fb: 75 f3                	jne	0xffff8000002004f0 <.text+0x4f0>
ffff8000002004fd: eb e1                	jmp	0xffff8000002004e0 <.text+0x4e0>
ffff8000002004ff: 48 8d 3d 2c 6b 00 00 	lea	rdi, [rip + 0x6b2c]     # 0xffff800000207032
ffff800000200506: 48 8d 35 f3 6a 00 00 	lea	rsi, [rip + 0x6af3]     # 0xffff800000207000
ffff80000020050d: 48 8d 55 a8          	lea	rdx, [rbp - 0x58]
ffff800000200511: e8 1a 04 00 00       	call	0xffff800000200930 <.text+0x930>
ffff800000200516: 84 c0                	test	al, al
ffff800000200518: 0f 85 4c 01 00 00    	jne	0xffff80000020066a <.text+0x66a>
ffff80000020051e: c6 05 0b 6b 00 00 00 	mov	byte ptr [rip + 0x6b0b], 0x0 # 0xffff800000207030
ffff800000200525: f7 c3 00 02 00 00    	test	ebx, 0x200
ffff80000020052b: 75 03                	jne	0xffff800000200530 <.text+0x530>
ffff80000020052d: fa                   	cli
ffff80000020052e: eb 01                	jmp	0xffff800000200531 <.text+0x531>
ffff800000200530: fb                   	sti
ffff800000200531: 48 8d 45 a0          	lea	rax, [rbp - 0x60]
ffff800000200535: 48 89 45 90          	mov	qword ptr [rbp - 0x70], rax
ffff800000200539: 48 8d 1d a0 01 00 00 	lea	rbx, [rip + 0x1a0]      # 0xffff8000002006e0 <.text+0x6e0>
ffff800000200540: 48 89 5d 98          	mov	qword ptr [rbp - 0x68], rbx
ffff800000200544: 48 8d 05 55 6b 00 00 	lea	rax, [rip + 0x6b55]     # 0xffff8000002070a0
ffff80000020054b: 48 89 45 a8          	mov	qword ptr [rbp - 0x58], rax
ffff80000020054f: 48 c7 45 b0 02 00 00 00      	mov	qword ptr [rbp - 0x50], 0x2
ffff800000200557: 4c 8d 35 4a 1b 00 00 	lea	r14, [rip + 0x1b4a]     # 0xffff8000002020a8
ffff80000020055e: 4c 89 75 c8          	mov	qword ptr [rbp - 0x38], r14
ffff800000200562: 48 c7 45 d0 01 00 00 00      	mov	qword ptr [rbp - 0x30], 0x1
ffff80000020056a: 4c 8d 7d 90          	lea	r15, [rbp - 0x70]
ffff80000020056e: 4c 89 7d b8          	mov	qword ptr [rbp - 0x48], r15
ffff800000200572: 48 c7 45 c0 01 00 00 00      	mov	qword ptr [rbp - 0x40], 0x1
ffff80000020057a: 9c                   	pushfq
ffff80000020057b: 41 5c                	pop	r12
ffff80000020057d: fa                   	cli
ffff80000020057e: b1 01                	mov	cl, 0x1
ffff800000200580: 31 c0                	xor	eax, eax
ffff800000200582: f0                   	lock
ffff800000200583: 0f b0 0d a6 6a 00 00 	cmpxchg	byte ptr [rip + 0x6aa6], cl # 0xffff800000207030
ffff80000020058a: 75 06                	jne	0xffff800000200592 <.text+0x592>
ffff80000020058c: eb 11                	jmp	0xffff80000020059f <.text+0x59f>
ffff80000020058e: 66 90                	nop
ffff800000200590: f3 90                	pause
ffff800000200592: 0f b6 05 97 6a 00 00 	movzx	eax, byte ptr [rip + 0x6a97] # 0xffff800000207030
ffff800000200599: 84 c0                	test	al, al
ffff80000020059b: 75 f3                	jne	0xffff800000200590 <.text+0x590>
ffff80000020059d: eb e1                	jmp	0xffff800000200580 <.text+0x580>
ffff80000020059f: 48 8d 3d 8c 6a 00 00 	lea	rdi, [rip + 0x6a8c]     # 0xffff800000207032
ffff8000002005a6: 48 8d 35 53 6a 00 00 	lea	rsi, [rip + 0x6a53]     # 0xffff800000207000
ffff8000002005ad: 48 8d 55 a8          	lea	rdx, [rbp - 0x58]
ffff8000002005b1: e8 7a 03 00 00       	call	0xffff800000200930 <.text+0x930>
ffff8000002005b6: 84 c0                	test	al, al
ffff8000002005b8: 0f 85 ac 00 00 00    	jne	0xffff80000020066a <.text+0x66a>
ffff8000002005be: c6 05 6b 6a 00 00 00 	mov	byte ptr [rip + 0x6a6b], 0x0 # 0xffff800000207030
ffff8000002005c5: 41 f7 c4 00 02 00 00 	test	r12d, 0x200
ffff8000002005cc: 75 03                	jne	0xffff8000002005d1 <.text+0x5d1>
ffff8000002005ce: fa                   	cli
ffff8000002005cf: eb 01                	jmp	0xffff8000002005d2 <.text+0x5d2>
ffff8000002005d1: fb                   	sti
ffff8000002005d2: 48 8d 45 88          	lea	rax, [rbp - 0x78]
ffff8000002005d6: 48 89 45 90          	mov	qword ptr [rbp - 0x70], rax
ffff8000002005da: 48 89 5d 98          	mov	qword ptr [rbp - 0x68], rbx
ffff8000002005de: 48 8d 05 db 6a 00 00 	lea	rax, [rip + 0x6adb]     # 0xffff8000002070c0
ffff8000002005e5: 48 89 45 a8          	mov	qword ptr [rbp - 0x58], rax
ffff8000002005e9: 48 c7 45 b0 02 00 00 00      	mov	qword ptr [rbp - 0x50], 0x2
ffff8000002005f1: 4c 89 75 c8          	mov	qword ptr [rbp - 0x38], r14
ffff8000002005f5: 48 c7 45 d0 01 00 00 00      	mov	qword ptr [rbp - 0x30], 0x1
ffff8000002005fd: 4c 89 7d b8          	mov	qword ptr [rbp - 0x48], r15
ffff800000200601: 48 c7 45 c0 01 00 00 00      	mov	qword ptr [rbp - 0x40], 0x1
ffff800000200609: 9c                   	pushfq
ffff80000020060a: 5b                   	pop	rbx
ffff80000020060b: fa                   	cli
ffff80000020060c: b1 01                	mov	cl, 0x1
ffff80000020060e: 66 90                	nop
ffff800000200610: 31 c0                	xor	eax, eax
ffff800000200612: f0                   	lock
ffff800000200613: 0f b0 0d 16 6a 00 00 	cmpxchg	byte ptr [rip + 0x6a16], cl # 0xffff800000207030
ffff80000020061a: 75 06                	jne	0xffff800000200622 <.text+0x622>
ffff80000020061c: eb 11                	jmp	0xffff80000020062f <.text+0x62f>
ffff80000020061e: 66 90                	nop
ffff800000200620: f3 90                	pause
ffff800000200622: 0f b6 05 07 6a 00 00 	movzx	eax, byte ptr [rip + 0x6a07] # 0xffff800000207030
ffff800000200629: 84 c0                	test	al, al
ffff80000020062b: 75 f3                	jne	0xffff800000200620 <.text+0x620>
ffff80000020062d: eb e1                	jmp	0xffff800000200610 <.text+0x610>
ffff80000020062f: 48 8d 3d fc 69 00 00 	lea	rdi, [rip + 0x69fc]     # 0xffff800000207032
ffff800000200636: 48 8d 35 c3 69 00 00 	lea	rsi, [rip + 0x69c3]     # 0xffff800000207000
ffff80000020063d: 48 8d 55 a8          	lea	rdx, [rbp - 0x58]
ffff800000200641: e8 ea 02 00 00       	call	0xffff800000200930 <.text+0x930>
ffff800000200646: 84 c0                	test	al, al
ffff800000200648: 75 20                	jne	0xffff80000020066a <.text+0x66a>
ffff80000020064a: c6 05 df 69 00 00 00 	mov	byte ptr [rip + 0x69df], 0x0 # 0xffff800000207030
ffff800000200651: f7 c3 00 02 00 00    	test	ebx, 0x200
ffff800000200657: 75 03                	jne	0xffff80000020065c <.text+0x65c>
ffff800000200659: fa                   	cli
ffff80000020065a: eb 01                	jmp	0xffff80000020065d <.text+0x65d>
ffff80000020065c: fb                   	sti
ffff80000020065d: 48 83 c4 60          	add	rsp, 0x60
ffff800000200661: 5b                   	pop	rbx
ffff800000200662: 41 5c                	pop	r12
ffff800000200664: 41 5e                	pop	r14
ffff800000200666: 41 5f                	pop	r15
ffff800000200668: 5d                   	pop	rbp
ffff800000200669: c3                   	ret
ffff80000020066a: 48 8d 7d df          	lea	rdi, [rbp - 0x21]
ffff80000020066e: e8 0d 02 00 00       	call	0xffff800000200880 <.text+0x880>
ffff800000200673: 48 8d 45 a0          	lea	rax, [rbp - 0x60]
ffff800000200677: 48 89 45 90          	mov	qword ptr [rbp - 0x70], rax
ffff80000020067b: 48 8d 05 5e 00 00 00 	lea	rax, [rip + 0x5e]       # 0xffff8000002006e0 <.text+0x6e0>
ffff800000200682: 48 89 45 98          	mov	qword ptr [rbp - 0x68], rax
ffff800000200686: 48 8d 05 53 6a 00 00 	lea	rax, [rip + 0x6a53]     # 0xffff8000002070e0
ffff80000020068d: 48 89 45 a8          	mov	qword ptr [rbp - 0x58], rax
ffff800000200691: 48 c7 45 b0 01 00 00 00      	mov	qword ptr [rbp - 0x50], 0x1
ffff800000200699: 48 8d 05 08 1a 00 00 	lea	rax, [rip + 0x1a08]     # 0xffff8000002020a8
ffff8000002006a0: 48 89 45 c8          	mov	qword ptr [rbp - 0x38], rax
ffff8000002006a4: 48 c7 45 d0 01 00 00 00      	mov	qword ptr [rbp - 0x30], 0x1
ffff8000002006ac: 48 8d 45 90          	lea	rax, [rbp - 0x70]
ffff8000002006b0: 48 89 45 b8          	mov	qword ptr [rbp - 0x48], rax
ffff8000002006b4: 48 c7 45 c0 01 00 00 00      	mov	qword ptr [rbp - 0x40], 0x1
ffff8000002006bc: 48 8d 35 2d 6a 00 00 	lea	rsi, [rip + 0x6a2d]     # 0xffff8000002070f0
ffff8000002006c3: 48 8d 7d a8          	lea	rdi, [rbp - 0x58]
ffff8000002006c7: e8 44 02 00 00       	call	0xffff800000200910 <.text+0x910>
ffff8000002006cc: cc                   	int3
ffff8000002006cd: cc                   	int3
ffff8000002006ce: cc                   	int3
ffff8000002006cf: cc                   	int3
ffff8000002006d0: 55                   	push	rbp
ffff8000002006d1: 48 89 e5             	mov	rbp, rsp
ffff8000002006d4: f3 90                	pause
ffff8000002006d6: 5d                   	pop	rbp
ffff8000002006d7: c3                   	ret
ffff8000002006d8: cc                   	int3
ffff8000002006d9: cc                   	int3
ffff8000002006da: cc                   	int3
ffff8000002006db: cc                   	int3
ffff8000002006dc: cc                   	int3
ffff8000002006dd: cc                   	int3
ffff8000002006de: cc                   	int3
ffff8000002006df: cc                   	int3
ffff8000002006e0: 55                   	push	rbp
ffff8000002006e1: 48 89 e5             	mov	rbp, rsp
ffff8000002006e4: 48 81 ec 80 00 00 00 	sub	rsp, 0x80
ffff8000002006eb: 48 89 f0             	mov	rax, rsi
ffff8000002006ee: 48 8b 0f             	mov	rcx, qword ptr [rdi]
ffff8000002006f1: bf 81 00 00 00       	mov	edi, 0x81
ffff8000002006f6: 48 89 ce             	mov	rsi, rcx
ffff8000002006f9: 0f 1f 80 00 00 00 00 	nop	dword ptr [rax]
ffff800000200700: 48 89 fa             	mov	rdx, rdi
ffff800000200703: 48 c1 ee 04          	shr	rsi, 0x4
ffff800000200707: 89 cf                	mov	edi, ecx
ffff800000200709: 40 80 e7 0f          	and	dil, 0xf
ffff80000020070d: 44 8d 47 30          	lea	r8d, [rdi + 0x30]
ffff800000200711: 44 8d 4f 57          	lea	r9d, [rdi + 0x57]
ffff800000200715: 40 80 ff 0a          	cmp	dil, 0xa
ffff800000200719: 41 0f b6 f8          	movzx	edi, r8b
ffff80000020071d: 45 0f b6 c1          	movzx	r8d, r9b
ffff800000200721: 44 0f 42 c7          	cmovb	r8d, edi
ffff800000200725: 44 88 84 15 7e ff ff ff      	mov	byte ptr [rbp + rdx - 0x82], r8b
ffff80000020072d: 48 8d 7a ff          	lea	rdi, [rdx - 0x1]
ffff800000200731: 48 83 f9 0f          	cmp	rcx, 0xf
ffff800000200735: 48 89 f1             	mov	rcx, rsi
ffff800000200738: 77 c6                	ja	0xffff800000200700 <.text+0x700>
ffff80000020073a: 48 83 c2 fe          	add	rdx, -0x2
ffff80000020073e: 48 8d 0c 2a          	lea	rcx, [rdx + rbp]
ffff800000200742: 48 83 c1 80          	add	rcx, -0x80
ffff800000200746: 41 b8 81 00 00 00    	mov	r8d, 0x81
ffff80000020074c: 49 29 f8             	sub	r8, rdi
ffff80000020074f: 48 8d 35 ca 19 00 00 	lea	rsi, [rip + 0x19ca]     # 0xffff800000202120
ffff800000200756: ba 02 00 00 00       	mov	edx, 0x2
ffff80000020075b: 48 89 c7             	mov	rdi, rax
ffff80000020075e: e8 2d 04 00 00       	call	0xffff800000200b90 <.text+0xb90>
ffff800000200763: 48 81 c4 80 00 00 00 	add	rsp, 0x80
ffff80000020076a: 5d                   	pop	rbp
ffff80000020076b: c3                   	ret
ffff80000020076c: cc                   	int3
ffff80000020076d: cc                   	int3
ffff80000020076e: cc                   	int3
ffff80000020076f: cc                   	int3
ffff800000200770: 55                   	push	rbp
ffff800000200771: 48 89 e5             	mov	rbp, rsp
ffff800000200774: 41 56                	push	r14
ffff800000200776: 53                   	push	rbx
ffff800000200777: 48 83 ec 10          	sub	rsp, 0x10
ffff80000020077b: 48 89 f0             	mov	rax, rsi
ffff80000020077e: 8b 37                	mov	esi, dword ptr [rdi]
ffff800000200780: b9 0a 00 00 00       	mov	ecx, 0xa
ffff800000200785: 48 8d 15 96 19 00 00 	lea	rdx, [rip + 0x1996]     # 0xffff800000202122
ffff80000020078c: 89 f7                	mov	edi, esi
ffff80000020078e: 81 fe e8 03 00 00    	cmp	esi, 0x3e8
ffff800000200794: 72 71                	jb	0xffff800000200807 <.text+0x807>
ffff800000200796: 41 b9 0a 00 00 00    	mov	r9d, 0xa
ffff80000020079c: 41 b8 59 17 b7 d1    	mov	r8d, 0xd1b71759
ffff8000002007a2: 41 89 f2             	mov	r10d, esi
ffff8000002007a5: 66 66 2e 0f 1f 84 00 00 00 00 00     	nop	word ptr cs:[rax + rax]
ffff8000002007b0: 49 8d 49 fc          	lea	rcx, [r9 - 0x4]
ffff8000002007b4: 44 89 d7             	mov	edi, r10d
ffff8000002007b7: 49 0f af f8          	imul	rdi, r8
ffff8000002007bb: 48 c1 ef 2d          	shr	rdi, 0x2d
ffff8000002007bf: 44 69 df 10 27 00 00 	imul	r11d, edi, 0x2710
ffff8000002007c6: 44 89 d3             	mov	ebx, r10d
ffff8000002007c9: 44 29 db             	sub	ebx, r11d
ffff8000002007cc: 44 69 db 7b 14 00 00 	imul	r11d, ebx, 0x147b
ffff8000002007d3: 41 c1 eb 13          	shr	r11d, 0x13
ffff8000002007d7: 45 6b f3 64          	imul	r14d, r11d, 0x64
ffff8000002007db: 44 29 f3             	sub	ebx, r14d
ffff8000002007de: 46 0f b7 1c 5a       	movzx	r11d, word ptr [rdx + 2*r11]
ffff8000002007e3: 66 46 89 5c 0d e2    	mov	word ptr [rbp + r9 - 0x1e], r11w
ffff8000002007e9: 44 0f b7 db          	movzx	r11d, bx
ffff8000002007ed: 46 0f b7 1c 5a       	movzx	r11d, word ptr [rdx + 2*r11]
ffff8000002007f2: 66 46 89 5c 0d e4    	mov	word ptr [rbp + r9 - 0x1c], r11w
ffff8000002007f8: 49 89 c9             	mov	r9, rcx
ffff8000002007fb: 41 81 fa 7f 96 98 00 	cmp	r10d, 0x98967f
ffff800000200802: 41 89 fa             	mov	r10d, edi
ffff800000200805: 77 a9                	ja	0xffff8000002007b0 <.text+0x7b0>
ffff800000200807: 83 ff 09             	cmp	edi, 0x9
ffff80000020080a: 76 2d                	jbe	0xffff800000200839 <.text+0x839>
ffff80000020080c: 44 0f b7 c7          	movzx	r8d, di
ffff800000200810: 41 c1 e8 02          	shr	r8d, 0x2
ffff800000200814: 45 69 c0 7b 14 00 00 	imul	r8d, r8d, 0x147b
ffff80000020081b: 41 c1 e8 11          	shr	r8d, 0x11
ffff80000020081f: 45 6b c8 64          	imul	r9d, r8d, 0x64
ffff800000200823: 44 29 cf             	sub	edi, r9d
ffff800000200826: 0f b7 ff             	movzx	edi, di
ffff800000200829: 0f b7 3c 7a          	movzx	edi, word ptr [rdx + 2*rdi]
ffff80000020082d: 66 89 7c 0d e4       	mov	word ptr [rbp + rcx - 0x1c], di
ffff800000200832: 48 83 c1 fe          	add	rcx, -0x2
ffff800000200836: 44 89 c7             	mov	edi, r8d
ffff800000200839: 85 f6                	test	esi, esi
ffff80000020083b: 74 04                	je	0xffff800000200841 <.text+0x841>
ffff80000020083d: 85 ff                	test	edi, edi
ffff80000020083f: 74 0f                	je	0xffff800000200850 <.text+0x850>
ffff800000200841: 83 e7 0f             	and	edi, 0xf
ffff800000200844: 0f b6 54 7a 01       	movzx	edx, byte ptr [rdx + 2*rdi + 0x1]
ffff800000200849: 88 54 0d e5          	mov	byte ptr [rbp + rcx - 0x1b], dl
ffff80000020084d: 48 ff c9             	dec	rcx
ffff800000200850: 41 b8 0a 00 00 00    	mov	r8d, 0xa
ffff800000200856: 49 29 c8             	sub	r8, rcx
ffff800000200859: 48 01 e9             	add	rcx, rbp
ffff80000020085c: 48 83 c1 e6          	add	rcx, -0x1a
ffff800000200860: be 01 00 00 00       	mov	esi, 0x1
ffff800000200865: 48 89 c7             	mov	rdi, rax
ffff800000200868: 31 d2                	xor	edx, edx
ffff80000020086a: e8 21 03 00 00       	call	0xffff800000200b90 <.text+0xb90>
ffff80000020086f: 48 83 c4 10          	add	rsp, 0x10
ffff800000200873: 5b                   	pop	rbx
ffff800000200874: 41 5e                	pop	r14
ffff800000200876: 5d                   	pop	rbp
ffff800000200877: c3                   	ret
ffff800000200878: cc                   	int3
ffff800000200879: cc                   	int3
ffff80000020087a: cc                   	int3
ffff80000020087b: cc                   	int3
ffff80000020087c: cc                   	int3
ffff80000020087d: cc                   	int3
ffff80000020087e: cc                   	int3
ffff80000020087f: cc                   	int3
ffff800000200880: 55                   	push	rbp
ffff800000200881: 48 89 e5             	mov	rbp, rsp
ffff800000200884: 48 83 ec 70          	sub	rsp, 0x70
ffff800000200888: 48 8d 05 9e 17 00 00 	lea	rax, [rip + 0x179e]     # 0xffff80000020202d
ffff80000020088f: 48 89 45 f0          	mov	qword ptr [rbp - 0x10], rax
ffff800000200893: 48 c7 45 f8 2b 00 00 00      	mov	qword ptr [rbp - 0x8], 0x2b
ffff80000020089b: 48 89 7d e0          	mov	qword ptr [rbp - 0x20], rdi
ffff80000020089f: 48 8d 05 92 67 00 00 	lea	rax, [rip + 0x6792]     # 0xffff800000207038
ffff8000002008a6: 48 89 45 e8          	mov	qword ptr [rbp - 0x18], rax
ffff8000002008aa: 48 8d 45 f0          	lea	rax, [rbp - 0x10]
ffff8000002008ae: 48 89 45 c0          	mov	qword ptr [rbp - 0x40], rax
ffff8000002008b2: 48 8d 05 b7 05 00 00 	lea	rax, [rip + 0x5b7]      # 0xffff800000200e70 <.text+0xe70>
ffff8000002008b9: 48 89 45 c8          	mov	qword ptr [rbp - 0x38], rax
ffff8000002008bd: 48 8d 45 e0          	lea	rax, [rbp - 0x20]
ffff8000002008c1: 48 89 45 d0          	mov	qword ptr [rbp - 0x30], rax
ffff8000002008c5: 48 8d 05 84 05 00 00 	lea	rax, [rip + 0x584]      # 0xffff800000200e50 <.text+0xe50>
ffff8000002008cc: 48 89 45 d8          	mov	qword ptr [rbp - 0x28], rax
ffff8000002008d0: 48 8d 05 61 68 00 00 	lea	rax, [rip + 0x6861]     # 0xffff800000207138
ffff8000002008d7: 48 89 45 90          	mov	qword ptr [rbp - 0x70], rax
ffff8000002008db: 48 c7 45 98 02 00 00 00      	mov	qword ptr [rbp - 0x68], 0x2
ffff8000002008e3: 48 c7 45 b0 00 00 00 00      	mov	qword ptr [rbp - 0x50], 0x0
ffff8000002008eb: 48 8d 45 c0          	lea	rax, [rbp - 0x40]
ffff8000002008ef: 48 89 45 a0          	mov	qword ptr [rbp - 0x60], rax
ffff8000002008f3: 48 c7 45 a8 02 00 00 00      	mov	qword ptr [rbp - 0x58], 0x2
ffff8000002008fb: 48 8d 35 56 67 00 00 	lea	rsi, [rip + 0x6756]     # 0xffff800000207058
ffff800000200902: 48 8d 7d 90          	lea	rdi, [rbp - 0x70]
ffff800000200906: e8 05 00 00 00       	call	0xffff800000200910 <.text+0x910>
ffff80000020090b: cc                   	int3
ffff80000020090c: cc                   	int3
ffff80000020090d: cc                   	int3
ffff80000020090e: cc                   	int3
ffff80000020090f: cc                   	int3
ffff800000200910: 55                   	push	rbp
ffff800000200911: 48 89 e5             	mov	rbp, rsp
ffff800000200914: 48 83 ec 20          	sub	rsp, 0x20
ffff800000200918: 48 89 7d e8          	mov	qword ptr [rbp - 0x18], rdi
ffff80000020091c: 48 89 75 f0          	mov	qword ptr [rbp - 0x10], rsi
ffff800000200920: 66 c7 45 f8 01 00    	mov	word ptr [rbp - 0x8], 0x1
ffff800000200926: 48 8d 7d e8          	lea	rdi, [rbp - 0x18]
ffff80000020092a: e8 31 fa ff ff       	call	0xffff800000200360 <.text+0x360>
ffff80000020092f: cc                   	int3
ffff800000200930: 55                   	push	rbp
ffff800000200931: 48 89 e5             	mov	rbp, rsp
ffff800000200934: 41 57                	push	r15
ffff800000200936: 41 56                	push	r14
ffff800000200938: 41 55                	push	r13
ffff80000020093a: 41 54                	push	r12
ffff80000020093c: 53                   	push	rbx
ffff80000020093d: 48 83 ec 38          	sub	rsp, 0x38
ffff800000200941: b8 20 00 00 e0       	mov	eax, 0xe0000020
ffff800000200946: 48 89 45 c8          	mov	qword ptr [rbp - 0x38], rax
ffff80000020094a: 48 89 7d b8          	mov	qword ptr [rbp - 0x48], rdi
ffff80000020094e: 48 89 75 c0          	mov	qword ptr [rbp - 0x40], rsi
ffff800000200952: 4c 8b 72 20          	mov	r14, qword ptr [rdx + 0x20]
ffff800000200956: 4d 85 f6             	test	r14, r14
ffff800000200959: 0f 84 48 01 00 00    	je	0xffff800000200aa7 <.text+0xaa7>
ffff80000020095f: 48 8b 4a 28          	mov	rcx, qword ptr [rdx + 0x28]
ffff800000200963: 48 85 c9             	test	rcx, rcx
ffff800000200966: 0f 84 e4 01 00 00    	je	0xffff800000200b50 <.text+0xb50>
ffff80000020096c: 48 8d 04 49          	lea	rax, [rcx + 2*rcx]
ffff800000200970: 48 c1 e0 04          	shl	rax, 0x4
ffff800000200974: 4c 01 f0             	add	rax, r14
ffff800000200977: 48 89 45 a8          	mov	qword ptr [rbp - 0x58], rax
ffff80000020097b: 49 8d 46 30          	lea	rax, [r14 + 0x30]
ffff80000020097f: 4c 8b 2a             	mov	r13, qword ptr [rdx]
ffff800000200982: 48 89 55 b0          	mov	qword ptr [rbp - 0x50], rdx
ffff800000200986: 48 8b 5a 10          	mov	rbx, qword ptr [rdx + 0x10]
ffff80000020098a: 48 ff c9             	dec	rcx
ffff80000020098d: 49 b8 ff ff ff ff ff ff ff 0f	movabs	r8, 0xfffffffffffffff
ffff800000200997: 49 21 c8             	and	r8, rcx
ffff80000020099a: 49 ff c0             	inc	r8
ffff80000020099d: 49 83 c5 08          	add	r13, 0x8
ffff8000002009a1: 45 31 e4             	xor	r12d, r12d
ffff8000002009a4: 4c 89 45 d0          	mov	qword ptr [rbp - 0x30], r8
ffff8000002009a8: 0f 1f 84 00 00 00 00 00      	nop	dword ptr [rax + rax]
ffff8000002009b0: 49 89 c7             	mov	r15, rax
ffff8000002009b3: 49 8b 55 00          	mov	rdx, qword ptr [r13]
ffff8000002009b7: 48 85 d2             	test	rdx, rdx
ffff8000002009ba: 74 17                	je	0xffff8000002009d3 <.text+0x9d3>
ffff8000002009bc: 48 8b 7d b8          	mov	rdi, qword ptr [rbp - 0x48]
ffff8000002009c0: 48 8b 45 c0          	mov	rax, qword ptr [rbp - 0x40]
ffff8000002009c4: 49 8b 75 f8          	mov	rsi, qword ptr [r13 - 0x8]
ffff8000002009c8: ff 50 18             	call	qword ptr [rax + 0x18]
ffff8000002009cb: 84 c0                	test	al, al
ffff8000002009cd: 0f 85 79 01 00 00    	jne	0xffff800000200b4c <.text+0xb4c>
ffff8000002009d3: 41 0f b7 46 10       	movzx	eax, word ptr [r14 + 0x10]
ffff8000002009d8: 85 c0                	test	eax, eax
ffff8000002009da: 74 34                	je	0xffff800000200a10 <.text+0xa10>
ffff8000002009dc: 83 f8 01             	cmp	eax, 0x1
ffff8000002009df: 75 4f                	jne	0xffff800000200a30 <.text+0xa30>
ffff8000002009e1: 49 8b 46 18          	mov	rax, qword ptr [r14 + 0x18]
ffff8000002009e5: 48 c1 e0 04          	shl	rax, 0x4
ffff8000002009e9: 0f b7 44 03 08       	movzx	eax, word ptr [rbx + rax + 0x8]
ffff8000002009ee: 41 0f b7 0e          	movzx	ecx, word ptr [r14]
ffff8000002009f2: 83 f9 02             	cmp	ecx, 0x2
ffff8000002009f5: 74 27                	je	0xffff800000200a1e <.text+0xa1e>
ffff8000002009f7: 83 f9 01             	cmp	ecx, 0x1
ffff8000002009fa: 75 44                	jne	0xffff800000200a40 <.text+0xa40>
ffff8000002009fc: 49 8b 4e 08          	mov	rcx, qword ptr [r14 + 0x8]
ffff800000200a00: 48 c1 e1 04          	shl	rcx, 0x4
ffff800000200a04: 0f b7 4c 0b 08       	movzx	ecx, word ptr [rbx + rcx + 0x8]
ffff800000200a09: eb 3a                	jmp	0xffff800000200a45 <.text+0xa45>
ffff800000200a0b: 0f 1f 44 00 00       	nop	dword ptr [rax + rax]
ffff800000200a10: 41 0f b7 46 12       	movzx	eax, word ptr [r14 + 0x12]
ffff800000200a15: 41 0f b7 0e          	movzx	ecx, word ptr [r14]
ffff800000200a19: 83 f9 02             	cmp	ecx, 0x2
ffff800000200a1c: 75 d9                	jne	0xffff8000002009f7 <.text+0x9f7>
ffff800000200a1e: 31 c9                	xor	ecx, ecx
ffff800000200a20: eb 23                	jmp	0xffff800000200a45 <.text+0xa45>
ffff800000200a22: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
ffff800000200a30: 31 c0                	xor	eax, eax
ffff800000200a32: 41 0f b7 0e          	movzx	ecx, word ptr [r14]
ffff800000200a36: 83 f9 02             	cmp	ecx, 0x2
ffff800000200a39: 75 bc                	jne	0xffff8000002009f7 <.text+0x9f7>
ffff800000200a3b: eb e1                	jmp	0xffff800000200a1e <.text+0xa1e>
ffff800000200a3d: 0f 1f 00             	nop	dword ptr [rax]
ffff800000200a40: 41 0f b7 4e 02       	movzx	ecx, word ptr [r14 + 0x2]
ffff800000200a45: 41 8b 56 28          	mov	edx, dword ptr [r14 + 0x28]
ffff800000200a49: 4d 8b 46 20          	mov	r8, qword ptr [r14 + 0x20]
ffff800000200a4d: 49 c1 e0 04          	shl	r8, 0x4
ffff800000200a51: 89 55 c8             	mov	dword ptr [rbp - 0x38], edx
ffff800000200a54: 66 89 45 cc          	mov	word ptr [rbp - 0x34], ax
ffff800000200a58: 66 89 4d ce          	mov	word ptr [rbp - 0x32], cx
ffff800000200a5c: 4a 8b 3c 03          	mov	rdi, qword ptr [rbx + r8]
ffff800000200a60: 48 8d 75 b8          	lea	rsi, [rbp - 0x48]
ffff800000200a64: 42 ff 54 03 08       	call	qword ptr [rbx + r8 + 0x8]
ffff800000200a69: 84 c0                	test	al, al
ffff800000200a6b: 0f 85 db 00 00 00    	jne	0xffff800000200b4c <.text+0xb4c>
ffff800000200a71: 49 ff c4             	inc	r12
ffff800000200a74: 49 8d 47 30          	lea	rax, [r15 + 0x30]
ffff800000200a78: 4c 3b 7d a8          	cmp	r15, qword ptr [rbp - 0x58]
ffff800000200a7c: 49 0f 44 c7          	cmove	rax, r15
ffff800000200a80: 49 83 c5 10          	add	r13, 0x10
ffff800000200a84: 4d 89 fe             	mov	r14, r15
ffff800000200a87: 4c 8b 45 d0          	mov	r8, qword ptr [rbp - 0x30]
ffff800000200a8b: 4d 39 e0             	cmp	r8, r12
ffff800000200a8e: 0f 85 1c ff ff ff    	jne	0xffff8000002009b0 <.text+0x9b0>
ffff800000200a94: 48 8b 55 b0          	mov	rdx, qword ptr [rbp - 0x50]
ffff800000200a98: 4c 3b 42 08          	cmp	r8, qword ptr [rdx + 0x8]
ffff800000200a9c: 0f 82 b7 00 00 00    	jb	0xffff800000200b59 <.text+0xb59>
ffff800000200aa2: e9 d5 00 00 00       	jmp	0xffff800000200b7c <.text+0xb7c>
ffff800000200aa7: 48 8b 4a 18          	mov	rcx, qword ptr [rdx + 0x18]
ffff800000200aab: 48 85 c9             	test	rcx, rcx
ffff800000200aae: 0f 84 9c 00 00 00    	je	0xffff800000200b50 <.text+0xb50>
ffff800000200ab4: 4c 8b 6a 10          	mov	r13, qword ptr [rdx + 0x10]
ffff800000200ab8: 48 89 cb             	mov	rbx, rcx
ffff800000200abb: 48 c1 e3 04          	shl	rbx, 0x4
ffff800000200abf: 4c 01 eb             	add	rbx, r13
ffff800000200ac2: 49 8d 45 10          	lea	rax, [r13 + 0x10]
ffff800000200ac6: 48 89 55 b0          	mov	qword ptr [rbp - 0x50], rdx
ffff800000200aca: 4c 8b 3a             	mov	r15, qword ptr [rdx]
ffff800000200acd: 48 ff c9             	dec	rcx
ffff800000200ad0: 48 ba ff ff ff ff ff ff ff 0f	movabs	rdx, 0xfffffffffffffff
ffff800000200ada: 48 21 ca             	and	rdx, rcx
ffff800000200add: 48 ff c2             	inc	rdx
ffff800000200ae0: 48 89 55 d0          	mov	qword ptr [rbp - 0x30], rdx
ffff800000200ae4: 49 83 c7 08          	add	r15, 0x8
ffff800000200ae8: 45 31 e4             	xor	r12d, r12d
ffff800000200aeb: 0f 1f 44 00 00       	nop	dword ptr [rax + rax]
ffff800000200af0: 49 89 c6             	mov	r14, rax
ffff800000200af3: 49 8b 17             	mov	rdx, qword ptr [r15]
ffff800000200af6: 48 85 d2             	test	rdx, rdx
ffff800000200af9: 74 13                	je	0xffff800000200b0e <.text+0xb0e>
ffff800000200afb: 48 8b 7d b8          	mov	rdi, qword ptr [rbp - 0x48]
ffff800000200aff: 48 8b 45 c0          	mov	rax, qword ptr [rbp - 0x40]
ffff800000200b03: 49 8b 77 f8          	mov	rsi, qword ptr [r15 - 0x8]
ffff800000200b07: ff 50 18             	call	qword ptr [rax + 0x18]
ffff800000200b0a: 84 c0                	test	al, al
ffff800000200b0c: 75 3e                	jne	0xffff800000200b4c <.text+0xb4c>
ffff800000200b0e: 49 8b 7d 00          	mov	rdi, qword ptr [r13]
ffff800000200b12: 48 8d 75 b8          	lea	rsi, [rbp - 0x48]
ffff800000200b16: 41 ff 55 08          	call	qword ptr [r13 + 0x8]
ffff800000200b1a: 84 c0                	test	al, al
ffff800000200b1c: 75 2e                	jne	0xffff800000200b4c <.text+0xb4c>
ffff800000200b1e: 49 ff c4             	inc	r12
ffff800000200b21: 31 c0                	xor	eax, eax
ffff800000200b23: 49 39 de             	cmp	r14, rbx
ffff800000200b26: 0f 95 c0             	setne	al
ffff800000200b29: c1 e0 04             	shl	eax, 0x4
ffff800000200b2c: 4c 01 f0             	add	rax, r14
ffff800000200b2f: 49 83 c7 10          	add	r15, 0x10
ffff800000200b33: 4d 89 f5             	mov	r13, r14
ffff800000200b36: 4c 39 65 d0          	cmp	qword ptr [rbp - 0x30], r12
ffff800000200b3a: 75 b4                	jne	0xffff800000200af0 <.text+0xaf0>
ffff800000200b3c: 48 8b 55 b0          	mov	rdx, qword ptr [rbp - 0x50]
ffff800000200b40: 4c 8b 45 d0          	mov	r8, qword ptr [rbp - 0x30]
ffff800000200b44: 4c 3b 42 08          	cmp	r8, qword ptr [rdx + 0x8]
ffff800000200b48: 72 0f                	jb	0xffff800000200b59 <.text+0xb59>
ffff800000200b4a: eb 30                	jmp	0xffff800000200b7c <.text+0xb7c>
ffff800000200b4c: b0 01                	mov	al, 0x1
ffff800000200b4e: eb 2e                	jmp	0xffff800000200b7e <.text+0xb7e>
ffff800000200b50: 45 31 c0             	xor	r8d, r8d
ffff800000200b53: 4c 3b 42 08          	cmp	r8, qword ptr [rdx + 0x8]
ffff800000200b57: 73 23                	jae	0xffff800000200b7c <.text+0xb7c>
ffff800000200b59: 48 8b 02             	mov	rax, qword ptr [rdx]
ffff800000200b5c: 49 c1 e0 04          	shl	r8, 0x4
ffff800000200b60: 48 8b 7d b8          	mov	rdi, qword ptr [rbp - 0x48]
ffff800000200b64: 48 8b 4d c0          	mov	rcx, qword ptr [rbp - 0x40]
ffff800000200b68: 4a 8b 34 00          	mov	rsi, qword ptr [rax + r8]
ffff800000200b6c: 4a 8b 54 00 08       	mov	rdx, qword ptr [rax + r8 + 0x8]
ffff800000200b71: ff 51 18             	call	qword ptr [rcx + 0x18]
ffff800000200b74: 89 c1                	mov	ecx, eax
ffff800000200b76: b0 01                	mov	al, 0x1
ffff800000200b78: 84 c9                	test	cl, cl
ffff800000200b7a: 75 02                	jne	0xffff800000200b7e <.text+0xb7e>
ffff800000200b7c: 31 c0                	xor	eax, eax
ffff800000200b7e: 48 83 c4 38          	add	rsp, 0x38
ffff800000200b82: 5b                   	pop	rbx
ffff800000200b83: 41 5c                	pop	r12
ffff800000200b85: 41 5d                	pop	r13
ffff800000200b87: 41 5e                	pop	r14
ffff800000200b89: 41 5f                	pop	r15
ffff800000200b8b: 5d                   	pop	rbp
ffff800000200b8c: c3                   	ret
ffff800000200b8d: cc                   	int3
ffff800000200b8e: cc                   	int3
ffff800000200b8f: cc                   	int3
ffff800000200b90: 55                   	push	rbp
ffff800000200b91: 48 89 e5             	mov	rbp, rsp
ffff800000200b94: 41 57                	push	r15
ffff800000200b96: 41 56                	push	r14
ffff800000200b98: 41 55                	push	r13
ffff800000200b9a: 41 54                	push	r12
ffff800000200b9c: 53                   	push	rbx
ffff800000200b9d: 48 83 ec 48          	sub	rsp, 0x48
ffff800000200ba1: 4d 89 c4             	mov	r12, r8
ffff800000200ba4: 48 89 4d c0          	mov	qword ptr [rbp - 0x40], rcx
ffff800000200ba8: 8b 5f 10             	mov	ebx, dword ptr [rdi + 0x10]
ffff800000200bab: 41 89 de             	mov	r14d, ebx
ffff800000200bae: 41 81 e6 00 00 20 00 	and	r14d, 0x200000
ffff800000200bb5: b8 00 00 11 00       	mov	eax, 0x110000
ffff800000200bba: b9 2b 00 00 00       	mov	ecx, 0x2b
ffff800000200bbf: 0f 44 c8             	cmove	ecx, eax
ffff800000200bc2: 89 4d d4             	mov	dword ptr [rbp - 0x2c], ecx
ffff800000200bc5: 41 c1 ee 15          	shr	r14d, 0x15
ffff800000200bc9: 4d 01 c6             	add	r14, r8
ffff800000200bcc: f7 c3 00 00 80 00    	test	ebx, 0x800000
ffff800000200bd2: 75 4f                	jne	0xffff800000200c23 <.text+0xc23>
ffff800000200bd4: 31 f6                	xor	esi, esi
ffff800000200bd6: 44 0f b7 6f 14       	movzx	r13d, word ptr [rdi + 0x14]
ffff800000200bdb: 4d 39 ee             	cmp	r14, r13
ffff800000200bde: 72 6f                	jb	0xffff800000200c4f <.text+0xc4f>
ffff800000200be0: 48 8b 1f             	mov	rbx, qword ptr [rdi]
ffff800000200be3: 4c 8b 7f 08          	mov	r15, qword ptr [rdi + 0x8]
ffff800000200be7: 48 89 df             	mov	rdi, rbx
ffff800000200bea: 48 89 f1             	mov	rcx, rsi
ffff800000200bed: 4c 89 fe             	mov	rsi, r15
ffff800000200bf0: 49 89 d0             	mov	r8, rdx
ffff800000200bf3: 8b 55 d4             	mov	edx, dword ptr [rbp - 0x2c]
ffff800000200bf6: e8 f5 01 00 00       	call	0xffff800000200df0 <.text+0xdf0>
ffff800000200bfb: b1 01                	mov	cl, 0x1
ffff800000200bfd: 84 c0                	test	al, al
ffff800000200bff: 0f 85 d3 01 00 00    	jne	0xffff800000200dd8 <.text+0xdd8>
ffff800000200c05: 49 8b 47 18          	mov	rax, qword ptr [r15 + 0x18]
ffff800000200c09: 48 89 df             	mov	rdi, rbx
ffff800000200c0c: 48 8b 75 c0          	mov	rsi, qword ptr [rbp - 0x40]
ffff800000200c10: 4c 89 e2             	mov	rdx, r12
ffff800000200c13: 48 83 c4 48          	add	rsp, 0x48
ffff800000200c17: 5b                   	pop	rbx
ffff800000200c18: 41 5c                	pop	r12
ffff800000200c1a: 41 5d                	pop	r13
ffff800000200c1c: 41 5e                	pop	r14
ffff800000200c1e: 41 5f                	pop	r15
ffff800000200c20: 5d                   	pop	rbp
ffff800000200c21: ff e0                	jmp	rax
ffff800000200c23: 31 c0                	xor	eax, eax
ffff800000200c25: 48 85 d2             	test	rdx, rdx
ffff800000200c28: 74 18                	je	0xffff800000200c42 <.text+0xc42>
ffff800000200c2a: 80 3e c0             	cmp	byte ptr [rsi], -0x40
ffff800000200c2d: 0f 9d c0             	setge	al
ffff800000200c30: 48 83 fa 01          	cmp	rdx, 0x1
ffff800000200c34: 74 0c                	je	0xffff800000200c42 <.text+0xc42>
ffff800000200c36: 31 c9                	xor	ecx, ecx
ffff800000200c38: 80 7e 01 c0          	cmp	byte ptr [rsi + 0x1], -0x40
ffff800000200c3c: 0f 9d c1             	setge	cl
ffff800000200c3f: 48 01 c8             	add	rax, rcx
ffff800000200c42: 49 01 c6             	add	r14, rax
ffff800000200c45: 44 0f b7 6f 14       	movzx	r13d, word ptr [rdi + 0x14]
ffff800000200c4a: 4d 39 ee             	cmp	r14, r13
ffff800000200c4d: 73 91                	jae	0xffff800000200be0 <.text+0xbe0>
ffff800000200c4f: f7 c3 00 00 00 01    	test	ebx, 0x1000000
ffff800000200c55: 4c 89 65 b0          	mov	qword ptr [rbp - 0x50], r12
ffff800000200c59: 75 32                	jne	0xffff800000200c8d <.text+0xc8d>
ffff800000200c5b: 45 89 e8             	mov	r8d, r13d
ffff800000200c5e: 45 29 f0             	sub	r8d, r14d
ffff800000200c61: 89 d8                	mov	eax, ebx
ffff800000200c63: c1 e8 1d             	shr	eax, 0x1d
ffff800000200c66: 83 e0 03             	and	eax, 0x3
ffff800000200c69: 48 8d 0d 98 13 00 00 	lea	rcx, [rip + 0x1398]     # 0xffff800000202008
ffff800000200c70: 48 63 04 81          	movsxd	rax, dword ptr [rcx + 4*rax]
ffff800000200c74: 48 01 c8             	add	rax, rcx
ffff800000200c77: 48 89 55 98          	mov	qword ptr [rbp - 0x68], rdx
ffff800000200c7b: 48 89 75 a0          	mov	qword ptr [rbp - 0x60], rsi
ffff800000200c7f: 44 89 45 bc          	mov	dword ptr [rbp - 0x44], r8d
ffff800000200c83: ff e0                	jmp	rax
ffff800000200c85: 44 89 c0             	mov	eax, r8d
ffff800000200c88: e9 8b 00 00 00       	jmp	0xffff800000200d18 <.text+0xd18>
ffff800000200c8d: 4c 8b 67 10          	mov	r12, qword ptr [rdi + 0x10]
ffff800000200c91: 44 89 e0             	mov	eax, r12d
ffff800000200c94: 25 00 00 e0 9f       	and	eax, 0x9fe00000
ffff800000200c99: 0d 30 00 00 20       	or	eax, 0x20000030
ffff800000200c9e: 89 47 10             	mov	dword ptr [rdi + 0x10], eax
ffff800000200ca1: 4c 8b 3f             	mov	r15, qword ptr [rdi]
ffff800000200ca4: 48 89 7d c8          	mov	qword ptr [rbp - 0x38], rdi
ffff800000200ca8: 48 8b 5f 08          	mov	rbx, qword ptr [rdi + 0x8]
ffff800000200cac: 4c 89 ff             	mov	rdi, r15
ffff800000200caf: 48 89 f1             	mov	rcx, rsi
ffff800000200cb2: 48 89 de             	mov	rsi, rbx
ffff800000200cb5: 49 89 d0             	mov	r8, rdx
ffff800000200cb8: 8b 55 d4             	mov	edx, dword ptr [rbp - 0x2c]
ffff800000200cbb: e8 30 01 00 00       	call	0xffff800000200df0 <.text+0xdf0>
ffff800000200cc0: b1 01                	mov	cl, 0x1
ffff800000200cc2: 84 c0                	test	al, al
ffff800000200cc4: 0f 85 0e 01 00 00    	jne	0xffff800000200dd8 <.text+0xdd8>
ffff800000200cca: 45 29 f5             	sub	r13d, r14d
ffff800000200ccd: 41 ff c5             	inc	r13d
ffff800000200cd0: 66 41 ff cd          	dec	r13w
ffff800000200cd4: 74 11                	je	0xffff800000200ce7 <.text+0xce7>
ffff800000200cd6: 4c 89 ff             	mov	rdi, r15
ffff800000200cd9: be 30 00 00 00       	mov	esi, 0x30
ffff800000200cde: ff 53 20             	call	qword ptr [rbx + 0x20]
ffff800000200ce1: 84 c0                	test	al, al
ffff800000200ce3: 74 eb                	je	0xffff800000200cd0 <.text+0xcd0>
ffff800000200ce5: eb 6e                	jmp	0xffff800000200d55 <.text+0xd55>
ffff800000200ce7: 4c 89 ff             	mov	rdi, r15
ffff800000200cea: 48 8b 75 c0          	mov	rsi, qword ptr [rbp - 0x40]
ffff800000200cee: 48 8b 55 b0          	mov	rdx, qword ptr [rbp - 0x50]
ffff800000200cf2: ff 53 18             	call	qword ptr [rbx + 0x18]
ffff800000200cf5: 84 c0                	test	al, al
ffff800000200cf7: b1 01                	mov	cl, 0x1
ffff800000200cf9: 0f 85 d9 00 00 00    	jne	0xffff800000200dd8 <.text+0xdd8>
ffff800000200cff: 48 8b 45 c8          	mov	rax, qword ptr [rbp - 0x38]
ffff800000200d03: 4c 89 60 10          	mov	qword ptr [rax + 0x10], r12
ffff800000200d07: 31 c9                	xor	ecx, ecx
ffff800000200d09: e9 ca 00 00 00       	jmp	0xffff800000200dd8 <.text+0xdd8>
ffff800000200d0e: 31 c0                	xor	eax, eax
ffff800000200d10: eb 06                	jmp	0xffff800000200d18 <.text+0xd18>
ffff800000200d12: 41 0f b7 c0          	movzx	eax, r8w
ffff800000200d16: d1 e8                	shr	eax
ffff800000200d18: 81 e3 ff ff 1f 00    	and	ebx, 0x1fffff
ffff800000200d1e: 48 8b 0f             	mov	rcx, qword ptr [rdi]
ffff800000200d21: 48 89 4d c8          	mov	qword ptr [rbp - 0x38], rcx
ffff800000200d25: 4c 8b 67 08          	mov	r12, qword ptr [rdi + 0x8]
ffff800000200d29: 48 89 45 a8          	mov	qword ptr [rbp - 0x58], rax
ffff800000200d2d: 44 8d 78 01          	lea	r15d, [rax + 0x1]
ffff800000200d31: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
ffff800000200d40: 66 41 ff cf          	dec	r15w
ffff800000200d44: 74 13                	je	0xffff800000200d59 <.text+0xd59>
ffff800000200d46: 48 8b 7d c8          	mov	rdi, qword ptr [rbp - 0x38]
ffff800000200d4a: 89 de                	mov	esi, ebx
ffff800000200d4c: 41 ff 54 24 20       	call	qword ptr [r12 + 0x20]
ffff800000200d51: 84 c0                	test	al, al
ffff800000200d53: 74 eb                	je	0xffff800000200d40 <.text+0xd40>
ffff800000200d55: b1 01                	mov	cl, 0x1
ffff800000200d57: eb 7f                	jmp	0xffff800000200dd8 <.text+0xdd8>
ffff800000200d59: 4c 8b 7d c8          	mov	r15, qword ptr [rbp - 0x38]
ffff800000200d5d: 4c 89 ff             	mov	rdi, r15
ffff800000200d60: 4c 89 e6             	mov	rsi, r12
ffff800000200d63: 8b 55 d4             	mov	edx, dword ptr [rbp - 0x2c]
ffff800000200d66: 48 8b 4d a0          	mov	rcx, qword ptr [rbp - 0x60]
ffff800000200d6a: 4c 8b 45 98          	mov	r8, qword ptr [rbp - 0x68]
ffff800000200d6e: e8 7d 00 00 00       	call	0xffff800000200df0 <.text+0xdf0>
ffff800000200d73: b1 01                	mov	cl, 0x1
ffff800000200d75: 84 c0                	test	al, al
ffff800000200d77: 75 5f                	jne	0xffff800000200dd8 <.text+0xdd8>
ffff800000200d79: 4c 89 ff             	mov	rdi, r15
ffff800000200d7c: 48 8b 75 c0          	mov	rsi, qword ptr [rbp - 0x40]
ffff800000200d80: 48 8b 55 b0          	mov	rdx, qword ptr [rbp - 0x50]
ffff800000200d84: 41 ff 54 24 18       	call	qword ptr [r12 + 0x18]
ffff800000200d89: b1 01                	mov	cl, 0x1
ffff800000200d8b: 84 c0                	test	al, al
ffff800000200d8d: 75 49                	jne	0xffff800000200dd8 <.text+0xdd8>
ffff800000200d8f: 44 8b 7d bc          	mov	r15d, dword ptr [rbp - 0x44]
ffff800000200d93: 48 8b 45 a8          	mov	rax, qword ptr [rbp - 0x58]
ffff800000200d97: 41 29 c7             	sub	r15d, eax
ffff800000200d9a: 44 01 f0             	add	eax, r14d
ffff800000200d9d: 44 29 e8             	sub	eax, r13d
ffff800000200da0: 49 89 c5             	mov	r13, rax
ffff800000200da3: 66 41 be ff ff       	mov	r14w, 0xffff
ffff800000200da8: 0f 1f 84 00 00 00 00 00      	nop	dword ptr [rax + rax]
ffff800000200db0: 43 8d 04 2e          	lea	eax, [r14 + r13]
ffff800000200db4: 66 83 f8 ff          	cmp	ax, -0x1
ffff800000200db8: 74 14                	je	0xffff800000200dce <.text+0xdce>
ffff800000200dba: 48 8b 7d c8          	mov	rdi, qword ptr [rbp - 0x38]
ffff800000200dbe: 89 de                	mov	esi, ebx
ffff800000200dc0: 41 ff 54 24 20       	call	qword ptr [r12 + 0x20]
ffff800000200dc5: 41 ff c6             	inc	r14d
ffff800000200dc8: 84 c0                	test	al, al
ffff800000200dca: 74 e4                	je	0xffff800000200db0 <.text+0xdb0>
ffff800000200dcc: eb 03                	jmp	0xffff800000200dd1 <.text+0xdd1>
ffff800000200dce: 45 89 fe             	mov	r14d, r15d
ffff800000200dd1: 66 45 39 fe          	cmp	r14w, r15w
ffff800000200dd5: 0f 92 c1             	setb	cl
ffff800000200dd8: 89 c8                	mov	eax, ecx
ffff800000200dda: 48 83 c4 48          	add	rsp, 0x48
ffff800000200dde: 5b                   	pop	rbx
ffff800000200ddf: 41 5c                	pop	r12
ffff800000200de1: 41 5d                	pop	r13
ffff800000200de3: 41 5e                	pop	r14
ffff800000200de5: 41 5f                	pop	r15
ffff800000200de7: 5d                   	pop	rbp
ffff800000200de8: c3                   	ret
ffff800000200de9: cc                   	int3
ffff800000200dea: cc                   	int3
ffff800000200deb: cc                   	int3
ffff800000200dec: cc                   	int3
ffff800000200ded: cc                   	int3
ffff800000200dee: cc                   	int3
ffff800000200def: cc                   	int3
ffff800000200df0: 55                   	push	rbp
ffff800000200df1: 48 89 e5             	mov	rbp, rsp
ffff800000200df4: 41 57                	push	r15
ffff800000200df6: 41 56                	push	r14
ffff800000200df8: 41 54                	push	r12
ffff800000200dfa: 53                   	push	rbx
ffff800000200dfb: 4c 89 c3             	mov	rbx, r8
ffff800000200dfe: 49 89 ce             	mov	r14, rcx
ffff800000200e01: 49 89 f7             	mov	r15, rsi
ffff800000200e04: 81 fa 00 00 11 00    	cmp	edx, 0x110000
ffff800000200e0a: 74 14                	je	0xffff800000200e20 <.text+0xe20>
ffff800000200e0c: 49 89 fc             	mov	r12, rdi
ffff800000200e0f: 89 d6                	mov	esi, edx
ffff800000200e11: 41 ff 57 20          	call	qword ptr [r15 + 0x20]
ffff800000200e15: 4c 89 e7             	mov	rdi, r12
ffff800000200e18: 89 c1                	mov	ecx, eax
ffff800000200e1a: b0 01                	mov	al, 0x1
ffff800000200e1c: 84 c9                	test	cl, cl
ffff800000200e1e: 75 1b                	jne	0xffff800000200e3b <.text+0xe3b>
ffff800000200e20: 4d 85 f6             	test	r14, r14
ffff800000200e23: 74 14                	je	0xffff800000200e39 <.text+0xe39>
ffff800000200e25: 49 8b 47 18          	mov	rax, qword ptr [r15 + 0x18]
ffff800000200e29: 4c 89 f6             	mov	rsi, r14
ffff800000200e2c: 48 89 da             	mov	rdx, rbx
ffff800000200e2f: 5b                   	pop	rbx
ffff800000200e30: 41 5c                	pop	r12
ffff800000200e32: 41 5e                	pop	r14
ffff800000200e34: 41 5f                	pop	r15
ffff800000200e36: 5d                   	pop	rbp
ffff800000200e37: ff e0                	jmp	rax
ffff800000200e39: 31 c0                	xor	eax, eax
ffff800000200e3b: 5b                   	pop	rbx
ffff800000200e3c: 41 5c                	pop	r12
ffff800000200e3e: 41 5e                	pop	r14
ffff800000200e40: 41 5f                	pop	r15
ffff800000200e42: 5d                   	pop	rbp
ffff800000200e43: c3                   	ret
ffff800000200e44: cc                   	int3
ffff800000200e45: cc                   	int3
ffff800000200e46: cc                   	int3
ffff800000200e47: cc                   	int3
ffff800000200e48: cc                   	int3
ffff800000200e49: cc                   	int3
ffff800000200e4a: cc                   	int3
ffff800000200e4b: cc                   	int3
ffff800000200e4c: cc                   	int3
ffff800000200e4d: cc                   	int3
ffff800000200e4e: cc                   	int3
ffff800000200e4f: cc                   	int3
ffff800000200e50: 55                   	push	rbp
ffff800000200e51: 48 89 e5             	mov	rbp, rsp
ffff800000200e54: 48 8b 07             	mov	rax, qword ptr [rdi]
ffff800000200e57: 48 8b 4f 08          	mov	rcx, qword ptr [rdi + 0x8]
ffff800000200e5b: 48 8b 49 18          	mov	rcx, qword ptr [rcx + 0x18]
ffff800000200e5f: 48 89 c7             	mov	rdi, rax
ffff800000200e62: 5d                   	pop	rbp
ffff800000200e63: ff e1                	jmp	rcx
ffff800000200e65: cc                   	int3
ffff800000200e66: cc                   	int3
ffff800000200e67: cc                   	int3
ffff800000200e68: cc                   	int3
ffff800000200e69: cc                   	int3
ffff800000200e6a: cc                   	int3
ffff800000200e6b: cc                   	int3
ffff800000200e6c: cc                   	int3
ffff800000200e6d: cc                   	int3
ffff800000200e6e: cc                   	int3
ffff800000200e6f: cc                   	int3
ffff800000200e70: 55                   	push	rbp
ffff800000200e71: 48 89 e5             	mov	rbp, rsp
ffff800000200e74: 41 57                	push	r15
ffff800000200e76: 41 56                	push	r14
ffff800000200e78: 41 55                	push	r13
ffff800000200e7a: 41 54                	push	r12
ffff800000200e7c: 53                   	push	rbx
ffff800000200e7d: 48 83 ec 28          	sub	rsp, 0x28
ffff800000200e81: 4c 8b 3f             	mov	r15, qword ptr [rdi]
ffff800000200e84: 4c 8b 67 08          	mov	r12, qword ptr [rdi + 0x8]
ffff800000200e88: 44 8b 76 10          	mov	r14d, dword ptr [rsi + 0x10]
ffff800000200e8c: 41 f7 c6 00 00 00 18 	test	r14d, 0x18000000
ffff800000200e93: 74 3e                	je	0xffff800000200ed3 <.text+0xed3>
ffff800000200e95: 41 f7 c6 00 00 00 10 	test	r14d, 0x10000000
ffff800000200e9c: 4c 89 7d c8          	mov	qword ptr [rbp - 0x38], r15
ffff800000200ea0: 44 89 75 d4          	mov	dword ptr [rbp - 0x2c], r14d
ffff800000200ea4: 75 40                	jne	0xffff800000200ee6 <.text+0xee6>
ffff800000200ea6: 49 83 fc 20          	cmp	r12, 0x20
ffff800000200eaa: 0f 83 97 00 00 00    	jae	0xffff800000200f47 <.text+0xf47>
ffff800000200eb0: 4d 85 e4             	test	r12, r12
ffff800000200eb3: 0f 84 da 01 00 00    	je	0xffff800000201093 <.text+0x1093>
ffff800000200eb9: 44 89 e0             	mov	eax, r12d
ffff800000200ebc: 83 e0 03             	and	eax, 0x3
ffff800000200ebf: 49 83 fc 04          	cmp	r12, 0x4
ffff800000200ec3: 0f 83 d5 01 00 00    	jae	0xffff80000020109e <.text+0x109e>
ffff800000200ec9: 45 31 ed             	xor	r13d, r13d
ffff800000200ecc: 31 c9                	xor	ecx, ecx
ffff800000200ece: e9 24 02 00 00       	jmp	0xffff8000002010f7 <.text+0x10f7>
ffff800000200ed3: 48 8b 3e             	mov	rdi, qword ptr [rsi]
ffff800000200ed6: 48 8b 46 08          	mov	rax, qword ptr [rsi + 0x8]
ffff800000200eda: 48 8b 40 18          	mov	rax, qword ptr [rax + 0x18]
ffff800000200ede: 4c 89 fe             	mov	rsi, r15
ffff800000200ee1: e9 e5 00 00 00       	jmp	0xffff800000200fcb <.text+0xfcb>
ffff800000200ee6: 44 0f b7 6e 16       	movzx	r13d, word ptr [rsi + 0x16]
ffff800000200eeb: 4d 85 ed             	test	r13, r13
ffff800000200eee: 74 7f                	je	0xffff800000200f6f <.text+0xf6f>
ffff800000200ef0: 48 8b 55 c8          	mov	rdx, qword ptr [rbp - 0x38]
ffff800000200ef4: 49 01 d4             	add	r12, rdx
ffff800000200ef7: 31 c0                	xor	eax, eax
ffff800000200ef9: 45 31 c0             	xor	r8d, r8d
ffff800000200efc: 31 c9                	xor	ecx, ecx
ffff800000200efe: eb 18                	jmp	0xffff800000200f18 <.text+0xf18>
ffff800000200f00: 4c 8d 4a 01          	lea	r9, [rdx + 0x1]
ffff800000200f04: 4d 89 c8             	mov	r8, r9
ffff800000200f07: 49 29 d0             	sub	r8, rdx
ffff800000200f0a: 49 01 f8             	add	r8, rdi
ffff800000200f0d: 48 ff c1             	inc	rcx
ffff800000200f10: 4c 89 ca             	mov	rdx, r9
ffff800000200f13: 49 39 cd             	cmp	r13, rcx
ffff800000200f16: 74 67                	je	0xffff800000200f7f <.text+0xf7f>
ffff800000200f18: 4c 89 c7             	mov	rdi, r8
ffff800000200f1b: 4c 39 e2             	cmp	rdx, r12
ffff800000200f1e: 74 56                	je	0xffff800000200f76 <.text+0xf76>
ffff800000200f20: 44 0f b6 02          	movzx	r8d, byte ptr [rdx]
ffff800000200f24: 45 84 c0             	test	r8b, r8b
ffff800000200f27: 79 d7                	jns	0xffff800000200f00 <.text+0xf00>
ffff800000200f29: 41 80 f8 e0          	cmp	r8b, -0x20
ffff800000200f2d: 72 0c                	jb	0xffff800000200f3b <.text+0xf3b>
ffff800000200f2f: 41 80 f8 f0          	cmp	r8b, -0x10
ffff800000200f33: 72 0c                	jb	0xffff800000200f41 <.text+0xf41>
ffff800000200f35: 4c 8d 4a 04          	lea	r9, [rdx + 0x4]
ffff800000200f39: eb c9                	jmp	0xffff800000200f04 <.text+0xf04>
ffff800000200f3b: 4c 8d 4a 02          	lea	r9, [rdx + 0x2]
ffff800000200f3f: eb c3                	jmp	0xffff800000200f04 <.text+0xf04>
ffff800000200f41: 4c 8d 4a 03          	lea	r9, [rdx + 0x3]
ffff800000200f45: eb bd                	jmp	0xffff800000200f04 <.text+0xf04>
ffff800000200f47: 49 8d 4f 07          	lea	rcx, [r15 + 0x7]
ffff800000200f4b: 48 83 e1 f8          	and	rcx, -0x8
ffff800000200f4f: 48 89 ca             	mov	rdx, rcx
ffff800000200f52: 4c 29 fa             	sub	rdx, r15
ffff800000200f55: 4c 89 e0             	mov	rax, r12
ffff800000200f58: 48 29 d0             	sub	rax, rdx
ffff800000200f5b: 89 c2                	mov	edx, eax
ffff800000200f5d: 83 e2 07             	and	edx, 0x7
ffff800000200f60: 49 89 ca             	mov	r10, rcx
ffff800000200f63: 4d 29 fa             	sub	r10, r15
ffff800000200f66: 75 7b                	jne	0xffff800000200fe3 <.text+0xfe3>
ffff800000200f68: 31 ff                	xor	edi, edi
ffff800000200f6a: e9 48 02 00 00       	jmp	0xffff8000002011b7 <.text+0x11b7>
ffff800000200f6f: 45 31 c0             	xor	r8d, r8d
ffff800000200f72: 31 c0                	xor	eax, eax
ffff800000200f74: eb 09                	jmp	0xffff800000200f7f <.text+0xf7f>
ffff800000200f76: 4c 89 e8             	mov	rax, r13
ffff800000200f79: 48 29 c8             	sub	rax, rcx
ffff800000200f7c: 49 89 f8             	mov	r8, rdi
ffff800000200f7f: 49 29 c5             	sub	r13, rax
ffff800000200f82: 4d 89 c4             	mov	r12, r8
ffff800000200f85: 0f b7 7e 14          	movzx	edi, word ptr [rsi + 0x14]
ffff800000200f89: 49 39 fd             	cmp	r13, rdi
ffff800000200f8c: 73 2e                	jae	0xffff800000200fbc <.text+0xfbc>
ffff800000200f8e: 89 fa                	mov	edx, edi
ffff800000200f90: 44 29 ea             	sub	edx, r13d
ffff800000200f93: 8b 45 d4             	mov	eax, dword ptr [rbp - 0x2c]
ffff800000200f96: c1 e8 1d             	shr	eax, 0x1d
ffff800000200f99: 83 e0 03             	and	eax, 0x3
ffff800000200f9c: 48 8d 0d 75 10 00 00 	lea	rcx, [rip + 0x1075]     # 0xffff800000202018
ffff800000200fa3: 48 63 04 81          	movsxd	rax, dword ptr [rcx + 4*rax]
ffff800000200fa7: 48 01 c8             	add	rax, rcx
ffff800000200faa: 4c 89 65 b8          	mov	qword ptr [rbp - 0x48], r12
ffff800000200fae: 89 55 d0             	mov	dword ptr [rbp - 0x30], edx
ffff800000200fb1: 48 89 7d c0          	mov	qword ptr [rbp - 0x40], rdi
ffff800000200fb5: ff e0                	jmp	rax
ffff800000200fb7: 45 31 ff             	xor	r15d, r15d
ffff800000200fba: eb 4f                	jmp	0xffff80000020100b <.text+0x100b>
ffff800000200fbc: 48 8b 3e             	mov	rdi, qword ptr [rsi]
ffff800000200fbf: 48 8b 46 08          	mov	rax, qword ptr [rsi + 0x8]
ffff800000200fc3: 48 8b 40 18          	mov	rax, qword ptr [rax + 0x18]
ffff800000200fc7: 48 8b 75 c8          	mov	rsi, qword ptr [rbp - 0x38]
ffff800000200fcb: 4c 89 e2             	mov	rdx, r12
ffff800000200fce: 48 83 c4 28          	add	rsp, 0x28
ffff800000200fd2: 5b                   	pop	rbx
ffff800000200fd3: 41 5c                	pop	r12
ffff800000200fd5: 41 5d                	pop	r13
ffff800000200fd7: 41 5e                	pop	r14
ffff800000200fd9: 41 5f                	pop	r15
ffff800000200fdb: 5d                   	pop	rbp
ffff800000200fdc: ff e0                	jmp	rax
ffff800000200fde: 41 89 d7             	mov	r15d, edx
ffff800000200fe1: eb 28                	jmp	0xffff80000020100b <.text+0x100b>
ffff800000200fe3: 45 89 d0             	mov	r8d, r10d
ffff800000200fe6: 41 83 e0 03          	and	r8d, 0x3
ffff800000200fea: 4c 89 ff             	mov	rdi, r15
ffff800000200fed: 48 29 cf             	sub	rdi, rcx
ffff800000200ff0: 48 83 ff fc          	cmp	rdi, -0x4
ffff800000200ff4: 0f 86 4d 01 00 00    	jbe	0xffff800000201147 <.text+0x1147>
ffff800000200ffa: 31 ff                	xor	edi, edi
ffff800000200ffc: 45 31 c9             	xor	r9d, r9d
ffff800000200fff: e9 90 01 00 00       	jmp	0xffff800000201194 <.text+0x1194>
ffff800000201004: 44 0f b7 fa          	movzx	r15d, dx
ffff800000201008: 41 d1 ef             	shr	r15d
ffff80000020100b: 81 65 d4 ff ff 1f 00 	and	dword ptr [rbp - 0x2c], 0x1fffff
ffff800000201012: 4c 8b 26             	mov	r12, qword ptr [rsi]
ffff800000201015: 48 8b 5e 08          	mov	rbx, qword ptr [rsi + 0x8]
ffff800000201019: 45 8d 77 01          	lea	r14d, [r15 + 0x1]
ffff80000020101d: 0f 1f 00             	nop	dword ptr [rax]
ffff800000201020: 4c 89 e7             	mov	rdi, r12
ffff800000201023: 66 41 ff ce          	dec	r14w
ffff800000201027: 74 11                	je	0xffff80000020103a <.text+0x103a>
ffff800000201029: 8b 75 d4             	mov	esi, dword ptr [rbp - 0x2c]
ffff80000020102c: ff 53 20             	call	qword ptr [rbx + 0x20]
ffff80000020102f: 84 c0                	test	al, al
ffff800000201031: 74 ed                	je	0xffff800000201020 <.text+0x1020>
ffff800000201033: b0 01                	mov	al, 0x1
ffff800000201035: e9 fe 00 00 00       	jmp	0xffff800000201138 <.text+0x1138>
ffff80000020103a: 48 8b 75 c8          	mov	rsi, qword ptr [rbp - 0x38]
ffff80000020103e: 48 8b 55 b8          	mov	rdx, qword ptr [rbp - 0x48]
ffff800000201042: ff 53 18             	call	qword ptr [rbx + 0x18]
ffff800000201045: 89 c1                	mov	ecx, eax
ffff800000201047: b0 01                	mov	al, 0x1
ffff800000201049: 84 c9                	test	cl, cl
ffff80000020104b: 0f 85 e7 00 00 00    	jne	0xffff800000201138 <.text+0x1138>
ffff800000201051: 44 29 7d d0          	sub	dword ptr [rbp - 0x30], r15d
ffff800000201055: 45 01 ef             	add	r15d, r13d
ffff800000201058: 44 2b 7d c0          	sub	r15d, dword ptr [rbp - 0x40]
ffff80000020105c: 66 41 be ff ff       	mov	r14w, 0xffff
ffff800000201061: 44 8b 6d d4          	mov	r13d, dword ptr [rbp - 0x2c]
ffff800000201065: 66 66 2e 0f 1f 84 00 00 00 00 00     	nop	word ptr cs:[rax + rax]
ffff800000201070: 43 8d 04 37          	lea	eax, [r15 + r14]
ffff800000201074: 66 83 f8 ff          	cmp	ax, -0x1
ffff800000201078: 0f 84 ac 00 00 00    	je	0xffff80000020112a <.text+0x112a>
ffff80000020107e: 4c 89 e7             	mov	rdi, r12
ffff800000201081: 44 89 ee             	mov	esi, r13d
ffff800000201084: ff 53 20             	call	qword ptr [rbx + 0x20]
ffff800000201087: 41 ff c6             	inc	r14d
ffff80000020108a: 84 c0                	test	al, al
ffff80000020108c: 74 e2                	je	0xffff800000201070 <.text+0x1070>
ffff80000020108e: e9 9d 00 00 00       	jmp	0xffff800000201130 <.text+0x1130>
ffff800000201093: 45 31 e4             	xor	r12d, r12d
ffff800000201096: 45 31 ed             	xor	r13d, r13d
ffff800000201099: e9 e7 fe ff ff       	jmp	0xffff800000200f85 <.text+0xf85>
ffff80000020109e: 44 89 e2             	mov	edx, r12d
ffff8000002010a1: 83 e2 1c             	and	edx, 0x1c
ffff8000002010a4: 45 31 ed             	xor	r13d, r13d
ffff8000002010a7: 31 c9                	xor	ecx, ecx
ffff8000002010a9: 4c 8b 55 c8          	mov	r10, qword ptr [rbp - 0x38]
ffff8000002010ad: 0f 1f 00             	nop	dword ptr [rax]
ffff8000002010b0: 31 ff                	xor	edi, edi
ffff8000002010b2: 41 80 3c 0a c0       	cmp	byte ptr [r10 + rcx], -0x40
ffff8000002010b7: 40 0f 9d c7          	setge	dil
ffff8000002010bb: 4c 01 ef             	add	rdi, r13
ffff8000002010be: 45 31 c0             	xor	r8d, r8d
ffff8000002010c1: 41 80 7c 0a 01 c0    	cmp	byte ptr [r10 + rcx + 0x1], -0x40
ffff8000002010c7: 41 0f 9d c0          	setge	r8b
ffff8000002010cb: 45 31 c9             	xor	r9d, r9d
ffff8000002010ce: 41 80 7c 0a 02 c0    	cmp	byte ptr [r10 + rcx + 0x2], -0x40
ffff8000002010d4: 41 0f 9d c1          	setge	r9b
ffff8000002010d8: 4d 01 c1             	add	r9, r8
ffff8000002010db: 49 01 f9             	add	r9, rdi
ffff8000002010de: 45 31 ed             	xor	r13d, r13d
ffff8000002010e1: 41 80 7c 0a 03 c0    	cmp	byte ptr [r10 + rcx + 0x3], -0x40
ffff8000002010e7: 41 0f 9d c5          	setge	r13b
ffff8000002010eb: 4d 01 cd             	add	r13, r9
ffff8000002010ee: 48 83 c1 04          	add	rcx, 0x4
ffff8000002010f2: 48 39 ca             	cmp	rdx, rcx
ffff8000002010f5: 75 b9                	jne	0xffff8000002010b0 <.text+0x10b0>
ffff8000002010f7: 48 85 c0             	test	rax, rax
ffff8000002010fa: 0f 84 85 fe ff ff    	je	0xffff800000200f85 <.text+0xf85>
ffff800000201100: 48 03 4d c8          	add	rcx, qword ptr [rbp - 0x38]
ffff800000201104: 31 d2                	xor	edx, edx
ffff800000201106: 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
ffff800000201110: 31 ff                	xor	edi, edi
ffff800000201112: 80 3c 11 c0          	cmp	byte ptr [rcx + rdx], -0x40
ffff800000201116: 40 0f 9d c7          	setge	dil
ffff80000020111a: 49 01 fd             	add	r13, rdi
ffff80000020111d: 48 ff c2             	inc	rdx
ffff800000201120: 48 39 d0             	cmp	rax, rdx
ffff800000201123: 75 eb                	jne	0xffff800000201110 <.text+0x1110>
ffff800000201125: e9 5b fe ff ff       	jmp	0xffff800000200f85 <.text+0xf85>
ffff80000020112a: 8b 45 d0             	mov	eax, dword ptr [rbp - 0x30]
ffff80000020112d: 41 89 c6             	mov	r14d, eax
ffff800000201130: 66 44 3b 75 d0       	cmp	r14w, word ptr [rbp - 0x30]
ffff800000201135: 0f 92 c0             	setb	al
ffff800000201138: 48 83 c4 28          	add	rsp, 0x28
ffff80000020113c: 5b                   	pop	rbx
ffff80000020113d: 41 5c                	pop	r12
ffff80000020113f: 41 5d                	pop	r13
ffff800000201141: 41 5e                	pop	r14
ffff800000201143: 41 5f                	pop	r15
ffff800000201145: 5d                   	pop	rbp
ffff800000201146: c3                   	ret
ffff800000201147: 41 83 e2 04          	and	r10d, 0x4
ffff80000020114b: 31 ff                	xor	edi, edi
ffff80000020114d: 45 31 c9             	xor	r9d, r9d
ffff800000201150: 45 31 db             	xor	r11d, r11d
ffff800000201153: 43 80 3c 0f c0       	cmp	byte ptr [r15 + r9], -0x40
ffff800000201158: 41 0f 9d c3          	setge	r11b
ffff80000020115c: 49 01 fb             	add	r11, rdi
ffff80000020115f: 31 ff                	xor	edi, edi
ffff800000201161: 43 80 7c 0f 01 c0    	cmp	byte ptr [r15 + r9 + 0x1], -0x40
ffff800000201167: 40 0f 9d c7          	setge	dil
ffff80000020116b: 31 db                	xor	ebx, ebx
ffff80000020116d: 43 80 7c 0f 02 c0    	cmp	byte ptr [r15 + r9 + 0x2], -0x40
ffff800000201173: 0f 9d c3             	setge	bl
ffff800000201176: 48 01 fb             	add	rbx, rdi
ffff800000201179: 4c 01 db             	add	rbx, r11
ffff80000020117c: 31 ff                	xor	edi, edi
ffff80000020117e: 43 80 7c 0f 03 c0    	cmp	byte ptr [r15 + r9 + 0x3], -0x40
ffff800000201184: 40 0f 9d c7          	setge	dil
ffff800000201188: 48 01 df             	add	rdi, rbx
ffff80000020118b: 49 83 c1 04          	add	r9, 0x4
ffff80000020118f: 4d 39 ca             	cmp	r10, r9
ffff800000201192: 75 bc                	jne	0xffff800000201150 <.text+0x1150>
ffff800000201194: 4d 85 c0             	test	r8, r8
ffff800000201197: 74 1e                	je	0xffff8000002011b7 <.text+0x11b7>
ffff800000201199: 4d 01 f9             	add	r9, r15
ffff80000020119c: 45 31 d2             	xor	r10d, r10d
ffff80000020119f: 90                   	nop
ffff8000002011a0: 45 31 db             	xor	r11d, r11d
ffff8000002011a3: 43 80 3c 11 c0       	cmp	byte ptr [r9 + r10], -0x40
ffff8000002011a8: 41 0f 9d c3          	setge	r11b
ffff8000002011ac: 4c 01 df             	add	rdi, r11
ffff8000002011af: 49 ff c2             	inc	r10
ffff8000002011b2: 4d 39 d0             	cmp	r8, r10
ffff8000002011b5: 75 e9                	jne	0xffff8000002011a0 <.text+0x11a0>
ffff8000002011b7: 48 85 d2             	test	rdx, rdx
ffff8000002011ba: 0f 84 91 00 00 00    	je	0xffff800000201251 <.text+0x1251>
ffff8000002011c0: 49 89 c0             	mov	r8, rax
ffff8000002011c3: 49 83 e0 f8          	and	r8, -0x8
ffff8000002011c7: 45 31 ed             	xor	r13d, r13d
ffff8000002011ca: 42 80 3c 01 c0       	cmp	byte ptr [rcx + r8], -0x40
ffff8000002011cf: 41 0f 9d c5          	setge	r13b
ffff8000002011d3: 83 fa 01             	cmp	edx, 0x1
ffff8000002011d6: 74 7c                	je	0xffff800000201254 <.text+0x1254>
ffff8000002011d8: 45 31 c9             	xor	r9d, r9d
ffff8000002011db: 42 80 7c 01 01 c0    	cmp	byte ptr [rcx + r8 + 0x1], -0x40
ffff8000002011e1: 41 0f 9d c1          	setge	r9b
ffff8000002011e5: 4d 01 cd             	add	r13, r9
ffff8000002011e8: 83 fa 02             	cmp	edx, 0x2
ffff8000002011eb: 74 67                	je	0xffff800000201254 <.text+0x1254>
ffff8000002011ed: 45 31 c9             	xor	r9d, r9d
ffff8000002011f0: 42 80 7c 01 02 c0    	cmp	byte ptr [rcx + r8 + 0x2], -0x40
ffff8000002011f6: 41 0f 9d c1          	setge	r9b
ffff8000002011fa: 4d 01 cd             	add	r13, r9
ffff8000002011fd: 83 fa 03             	cmp	edx, 0x3
ffff800000201200: 74 52                	je	0xffff800000201254 <.text+0x1254>
ffff800000201202: 45 31 c9             	xor	r9d, r9d
ffff800000201205: 42 80 7c 01 03 c0    	cmp	byte ptr [rcx + r8 + 0x3], -0x40
ffff80000020120b: 41 0f 9d c1          	setge	r9b
ffff80000020120f: 4d 01 cd             	add	r13, r9
ffff800000201212: 83 fa 04             	cmp	edx, 0x4
ffff800000201215: 74 3d                	je	0xffff800000201254 <.text+0x1254>
ffff800000201217: 45 31 c9             	xor	r9d, r9d
ffff80000020121a: 42 80 7c 01 04 c0    	cmp	byte ptr [rcx + r8 + 0x4], -0x40
ffff800000201220: 41 0f 9d c1          	setge	r9b
ffff800000201224: 4d 01 cd             	add	r13, r9
ffff800000201227: 83 fa 05             	cmp	edx, 0x5
ffff80000020122a: 74 28                	je	0xffff800000201254 <.text+0x1254>
ffff80000020122c: 45 31 c9             	xor	r9d, r9d
ffff80000020122f: 42 80 7c 01 05 c0    	cmp	byte ptr [rcx + r8 + 0x5], -0x40
ffff800000201235: 41 0f 9d c1          	setge	r9b
ffff800000201239: 4d 01 cd             	add	r13, r9
ffff80000020123c: 83 fa 06             	cmp	edx, 0x6
ffff80000020123f: 74 13                	je	0xffff800000201254 <.text+0x1254>
ffff800000201241: 31 d2                	xor	edx, edx
ffff800000201243: 42 80 7c 01 06 c0    	cmp	byte ptr [rcx + r8 + 0x6], -0x40
ffff800000201249: 0f 9d c2             	setge	dl
ffff80000020124c: 49 01 d5             	add	r13, rdx
ffff80000020124f: eb 03                	jmp	0xffff800000201254 <.text+0x1254>
ffff800000201251: 45 31 ed             	xor	r13d, r13d
ffff800000201254: 49 01 fd             	add	r13, rdi
ffff800000201257: 48 c1 e8 03          	shr	rax, 0x3
ffff80000020125b: 49 b8 01 01 01 01 01 01 01 01	movabs	r8, 0x101010101010101
ffff800000201265: 48 bf ff 00 ff 00 ff 00 ff 00	movabs	rdi, 0xff00ff00ff00ff
ffff80000020126f: eb 53                	jmp	0xffff8000002012c4 <.text+0x12c4>
ffff800000201271: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
ffff800000201280: 31 db                	xor	ebx, ebx
ffff800000201282: 44 89 d1             	mov	ecx, r10d
ffff800000201285: 49 8d 0c c9          	lea	rcx, [r9 + 8*rcx]
ffff800000201289: 4c 29 d0             	sub	rax, r10
ffff80000020128c: 45 89 d3             	mov	r11d, r10d
ffff80000020128f: 41 83 e3 03          	and	r11d, 0x3
ffff800000201293: 49 89 de             	mov	r14, rbx
ffff800000201296: 49 21 fe             	and	r14, rdi
ffff800000201299: 48 c1 eb 08          	shr	rbx, 0x8
ffff80000020129d: 48 21 fb             	and	rbx, rdi
ffff8000002012a0: 4c 01 f3             	add	rbx, r14
ffff8000002012a3: 49 be 01 00 01 00 01 00 01 00	movabs	r14, 0x1000100010001
ffff8000002012ad: 49 0f af de          	imul	rbx, r14
ffff8000002012b1: 48 c1 eb 30          	shr	rbx, 0x30
ffff8000002012b5: 49 01 dd             	add	r13, rbx
ffff8000002012b8: 4d 85 db             	test	r11, r11
ffff8000002012bb: 49 89 d4             	mov	r12, rdx
ffff8000002012be: 0f 85 b9 00 00 00    	jne	0xffff80000020137d <.text+0x137d>
ffff8000002012c4: 48 85 c0             	test	rax, rax
ffff8000002012c7: 0f 84 b8 fc ff ff    	je	0xffff800000200f85 <.text+0xf85>
ffff8000002012cd: 49 89 c9             	mov	r9, rcx
ffff8000002012d0: 4c 89 e2             	mov	rdx, r12
ffff8000002012d3: 48 3d c0 00 00 00    	cmp	rax, 0xc0
ffff8000002012d9: 41 ba c0 00 00 00    	mov	r10d, 0xc0
ffff8000002012df: 4c 0f 42 d0          	cmovb	r10, rax
ffff8000002012e3: 48 83 f8 04          	cmp	rax, 0x4
ffff8000002012e7: 72 97                	jb	0xffff800000201280 <.text+0x1280>
ffff8000002012e9: 44 89 d1             	mov	ecx, r10d
ffff8000002012ec: c1 e9 02             	shr	ecx, 0x2
ffff8000002012ef: 48 c1 e1 05          	shl	rcx, 0x5
ffff8000002012f3: 45 31 db             	xor	r11d, r11d
ffff8000002012f6: 31 db                	xor	ebx, ebx
ffff8000002012f8: 0f 1f 84 00 00 00 00 00      	nop	dword ptr [rax + rax]
ffff800000201300: 4f 8b 3c 19          	mov	r15, qword ptr [r9 + r11]
ffff800000201304: 4f 8b 64 19 08       	mov	r12, qword ptr [r9 + r11 + 0x8]
ffff800000201309: 4d 89 fe             	mov	r14, r15
ffff80000020130c: 49 f7 d6             	not	r14
ffff80000020130f: 49 c1 ee 07          	shr	r14, 0x7
ffff800000201313: 49 c1 ef 06          	shr	r15, 0x6
ffff800000201317: 4d 09 f7             	or	r15, r14
ffff80000020131a: 4d 21 c7             	and	r15, r8
ffff80000020131d: 49 01 df             	add	r15, rbx
ffff800000201320: 4c 89 e3             	mov	rbx, r12
ffff800000201323: 48 f7 d3             	not	rbx
ffff800000201326: 48 c1 eb 07          	shr	rbx, 0x7
ffff80000020132a: 49 c1 ec 06          	shr	r12, 0x6
ffff80000020132e: 49 09 dc             	or	r12, rbx
ffff800000201331: 4d 21 c4             	and	r12, r8
ffff800000201334: 4f 8b 74 19 10       	mov	r14, qword ptr [r9 + r11 + 0x10]
ffff800000201339: 4c 89 f3             	mov	rbx, r14
ffff80000020133c: 48 f7 d3             	not	rbx
ffff80000020133f: 48 c1 eb 07          	shr	rbx, 0x7
ffff800000201343: 49 c1 ee 06          	shr	r14, 0x6
ffff800000201347: 49 09 de             	or	r14, rbx
ffff80000020134a: 4d 21 c6             	and	r14, r8
ffff80000020134d: 4d 01 e6             	add	r14, r12
ffff800000201350: 4d 01 fe             	add	r14, r15
ffff800000201353: 4b 8b 5c 19 18       	mov	rbx, qword ptr [r9 + r11 + 0x18]
ffff800000201358: 49 89 df             	mov	r15, rbx
ffff80000020135b: 49 f7 d7             	not	r15
ffff80000020135e: 49 c1 ef 07          	shr	r15, 0x7
ffff800000201362: 48 c1 eb 06          	shr	rbx, 0x6
ffff800000201366: 4c 09 fb             	or	rbx, r15
ffff800000201369: 4c 21 c3             	and	rbx, r8
ffff80000020136c: 4c 01 f3             	add	rbx, r14
ffff80000020136f: 49 83 c3 20          	add	r11, 0x20
ffff800000201373: 4c 39 d9             	cmp	rcx, r11
ffff800000201376: 75 88                	jne	0xffff800000201300 <.text+0x1300>
ffff800000201378: e9 05 ff ff ff       	jmp	0xffff800000201282 <.text+0x1282>
ffff80000020137d: 41 81 e2 fc 00 00 00 	and	r10d, 0xfc
ffff800000201384: 44 89 d0             	mov	eax, r10d
ffff800000201387: 49 8b 04 c1          	mov	rax, qword ptr [r9 + 8*rax]
ffff80000020138b: 48 89 c1             	mov	rcx, rax
ffff80000020138e: 48 f7 d1             	not	rcx
ffff800000201391: 48 c1 e9 07          	shr	rcx, 0x7
ffff800000201395: 48 c1 e8 06          	shr	rax, 0x6
ffff800000201399: 48 09 c8             	or	rax, rcx
ffff80000020139c: 4c 21 c0             	and	rax, r8
ffff80000020139f: 41 83 fb 01          	cmp	r11d, 0x1
ffff8000002013a3: 74 3e                	je	0xffff8000002013e3 <.text+0x13e3>
ffff8000002013a5: 4b 8b 4c d1 08       	mov	rcx, qword ptr [r9 + 8*r10 + 0x8]
ffff8000002013aa: 48 89 cb             	mov	rbx, rcx
ffff8000002013ad: 48 f7 d3             	not	rbx
ffff8000002013b0: 48 c1 eb 07          	shr	rbx, 0x7
ffff8000002013b4: 48 c1 e9 06          	shr	rcx, 0x6
ffff8000002013b8: 48 09 d9             	or	rcx, rbx
ffff8000002013bb: 4c 21 c1             	and	rcx, r8
ffff8000002013be: 48 01 c8             	add	rax, rcx
ffff8000002013c1: 41 83 fb 02          	cmp	r11d, 0x2
ffff8000002013c5: 74 1c                	je	0xffff8000002013e3 <.text+0x13e3>
ffff8000002013c7: 4b 8b 4c d1 10       	mov	rcx, qword ptr [r9 + 8*r10 + 0x10]
ffff8000002013cc: 49 89 c9             	mov	r9, rcx
ffff8000002013cf: 49 f7 d1             	not	r9
ffff8000002013d2: 49 c1 e9 07          	shr	r9, 0x7
ffff8000002013d6: 48 c1 e9 06          	shr	rcx, 0x6
ffff8000002013da: 4c 09 c9             	or	rcx, r9
ffff8000002013dd: 4c 21 c1             	and	rcx, r8
ffff8000002013e0: 48 01 c8             	add	rax, rcx
ffff8000002013e3: 48 89 c1             	mov	rcx, rax
ffff8000002013e6: 48 21 f9             	and	rcx, rdi
ffff8000002013e9: 48 c1 e8 08          	shr	rax, 0x8
ffff8000002013ed: 48 21 f8             	and	rax, rdi
ffff8000002013f0: 48 01 c8             	add	rax, rcx
ffff8000002013f3: 48 b9 01 00 01 00 01 00 01 00	movabs	rcx, 0x1000100010001
ffff8000002013fd: 48 0f af c1          	imul	rax, rcx
ffff800000201401: 48 c1 e8 30          	shr	rax, 0x30
ffff800000201405: 49 01 c5             	add	r13, rax
ffff800000201408: e9 78 fb ff ff       	jmp	0xffff800000200f85 <.text+0xf85>
ffff80000020140d: cc                   	int3
ffff80000020140e: cc                   	int3
ffff80000020140f: cc                   	int3
ffff800000201410: 48 85 d2             	test	rdx, rdx
ffff800000201413: 0f 84 1d 01 00 00    	je	0xffff800000201536 <.text+0x1536>
ffff800000201419: 55                   	push	rbp
ffff80000020141a: 48 89 e5             	mov	rbp, rsp
ffff80000020141d: 48 89 d1             	mov	rcx, rdx
ffff800000201420: 48 01 f1             	add	rcx, rsi
ffff800000201423: 0f b7 3f             	movzx	edi, word ptr [rdi]
ffff800000201426: 41 89 f8             	mov	r8d, edi
ffff800000201429: 41 83 c0 05          	add	r8d, 0x5
ffff80000020142d: eb 12                	jmp	0xffff800000201441 <.text+0x1441>
ffff80000020142f: 90                   	nop
ffff800000201430: b0 0a                	mov	al, 0xa
ffff800000201432: 89 fa                	mov	edx, edi
ffff800000201434: ee                   	out	dx, al
ffff800000201435: 48 ff c6             	inc	rsi
ffff800000201438: 48 39 ce             	cmp	rsi, rcx
ffff80000020143b: 0f 84 f4 00 00 00    	je	0xffff800000201535 <.text+0x1535>
ffff800000201441: 44 0f b6 0e          	movzx	r9d, byte ptr [rsi]
ffff800000201445: 41 83 f9 08          	cmp	r9d, 0x8
ffff800000201449: 74 10                	je	0xffff80000020145b <.text+0x145b>
ffff80000020144b: 41 83 f9 0a          	cmp	r9d, 0xa
ffff80000020144f: 74 7f                	je	0xffff8000002014d0 <.text+0x14d0>
ffff800000201451: 41 83 f9 7f          	cmp	r9d, 0x7f
ffff800000201455: 0f 85 b5 00 00 00    	jne	0xffff800000201510 <.text+0x1510>
ffff80000020145b: 44 89 c2             	mov	edx, r8d
ffff80000020145e: ec                   	in	al, dx
ffff80000020145f: eb 15                	jmp	0xffff800000201476 <.text+0x1476>
ffff800000201461: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
ffff800000201470: f3 90                	pause
ffff800000201472: 44 89 c2             	mov	edx, r8d
ffff800000201475: ec                   	in	al, dx
ffff800000201476: a8 20                	test	al, 0x20
ffff800000201478: 74 f6                	je	0xffff800000201470 <.text+0x1470>
ffff80000020147a: b0 08                	mov	al, 0x8
ffff80000020147c: 89 fa                	mov	edx, edi
ffff80000020147e: ee                   	out	dx, al
ffff80000020147f: 44 89 c2             	mov	edx, r8d
ffff800000201482: ec                   	in	al, dx
ffff800000201483: eb 11                	jmp	0xffff800000201496 <.text+0x1496>
ffff800000201485: 66 66 2e 0f 1f 84 00 00 00 00 00     	nop	word ptr cs:[rax + rax]
ffff800000201490: f3 90                	pause
ffff800000201492: 44 89 c2             	mov	edx, r8d
ffff800000201495: ec                   	in	al, dx
ffff800000201496: a8 20                	test	al, 0x20
ffff800000201498: 74 f6                	je	0xffff800000201490 <.text+0x1490>
ffff80000020149a: b0 20                	mov	al, 0x20
ffff80000020149c: 89 fa                	mov	edx, edi
ffff80000020149e: ee                   	out	dx, al
ffff80000020149f: 44 89 c2             	mov	edx, r8d
ffff8000002014a2: ec                   	in	al, dx
ffff8000002014a3: eb 11                	jmp	0xffff8000002014b6 <.text+0x14b6>
ffff8000002014a5: 66 66 2e 0f 1f 84 00 00 00 00 00     	nop	word ptr cs:[rax + rax]
ffff8000002014b0: f3 90                	pause
ffff8000002014b2: 44 89 c2             	mov	edx, r8d
ffff8000002014b5: ec                   	in	al, dx
ffff8000002014b6: a8 20                	test	al, 0x20
ffff8000002014b8: 74 f6                	je	0xffff8000002014b0 <.text+0x14b0>
ffff8000002014ba: b0 08                	mov	al, 0x8
ffff8000002014bc: 89 fa                	mov	edx, edi
ffff8000002014be: ee                   	out	dx, al
ffff8000002014bf: e9 71 ff ff ff       	jmp	0xffff800000201435 <.text+0x1435>
ffff8000002014c4: 66 66 66 2e 0f 1f 84 00 00 00 00 00  	nop	word ptr cs:[rax + rax]
ffff8000002014d0: 44 89 c2             	mov	edx, r8d
ffff8000002014d3: ec                   	in	al, dx
ffff8000002014d4: a8 20                	test	al, 0x20
ffff8000002014d6: 75 12                	jne	0xffff8000002014ea <.text+0x14ea>
ffff8000002014d8: 0f 1f 84 00 00 00 00 00      	nop	dword ptr [rax + rax]
ffff8000002014e0: f3 90                	pause
ffff8000002014e2: 44 89 c2             	mov	edx, r8d
ffff8000002014e5: ec                   	in	al, dx
ffff8000002014e6: a8 20                	test	al, 0x20
ffff8000002014e8: 74 f6                	je	0xffff8000002014e0 <.text+0x14e0>
ffff8000002014ea: b0 0d                	mov	al, 0xd
ffff8000002014ec: 89 fa                	mov	edx, edi
ffff8000002014ee: ee                   	out	dx, al
ffff8000002014ef: 44 89 c2             	mov	edx, r8d
ffff8000002014f2: ec                   	in	al, dx
ffff8000002014f3: a8 20                	test	al, 0x20
ffff8000002014f5: 0f 85 35 ff ff ff    	jne	0xffff800000201430 <.text+0x1430>
ffff8000002014fb: 0f 1f 44 00 00       	nop	dword ptr [rax + rax]
ffff800000201500: f3 90                	pause
ffff800000201502: 44 89 c2             	mov	edx, r8d
ffff800000201505: ec                   	in	al, dx
ffff800000201506: a8 20                	test	al, 0x20
ffff800000201508: 74 f6                	je	0xffff800000201500 <.text+0x1500>
ffff80000020150a: e9 21 ff ff ff       	jmp	0xffff800000201430 <.text+0x1430>
ffff80000020150f: 90                   	nop
ffff800000201510: 44 89 c2             	mov	edx, r8d
ffff800000201513: ec                   	in	al, dx
ffff800000201514: a8 20                	test	al, 0x20
ffff800000201516: 75 12                	jne	0xffff80000020152a <.text+0x152a>
ffff800000201518: 0f 1f 84 00 00 00 00 00      	nop	dword ptr [rax + rax]
ffff800000201520: f3 90                	pause
ffff800000201522: 44 89 c2             	mov	edx, r8d
ffff800000201525: ec                   	in	al, dx
ffff800000201526: a8 20                	test	al, 0x20
ffff800000201528: 74 f6                	je	0xffff800000201520 <.text+0x1520>
ffff80000020152a: 44 89 c8             	mov	eax, r9d
ffff80000020152d: 89 fa                	mov	edx, edi
ffff80000020152f: ee                   	out	dx, al
ffff800000201530: e9 00 ff ff ff       	jmp	0xffff800000201435 <.text+0x1435>
ffff800000201535: 5d                   	pop	rbp
ffff800000201536: 31 c0                	xor	eax, eax
ffff800000201538: c3                   	ret
ffff800000201539: cc                   	int3
ffff80000020153a: cc                   	int3
ffff80000020153b: cc                   	int3
ffff80000020153c: cc                   	int3
ffff80000020153d: cc                   	int3
ffff80000020153e: cc                   	int3
ffff80000020153f: cc                   	int3
ffff800000201540: cc                   	int3
ffff800000201541: cc                   	int3
ffff800000201542: cc                   	int3
ffff800000201543: cc                   	int3
ffff800000201544: cc                   	int3
ffff800000201545: cc                   	int3
ffff800000201546: cc                   	int3
ffff800000201547: cc                   	int3
ffff800000201548: cc                   	int3
ffff800000201549: cc                   	int3
ffff80000020154a: cc                   	int3
ffff80000020154b: cc                   	int3
ffff80000020154c: cc                   	int3
ffff80000020154d: cc                   	int3
ffff80000020154e: cc                   	int3
ffff80000020154f: cc                   	int3
ffff800000201550: cc                   	int3
ffff800000201551: cc                   	int3
ffff800000201552: cc                   	int3
ffff800000201553: cc                   	int3
ffff800000201554: cc                   	int3
ffff800000201555: cc                   	int3
ffff800000201556: cc                   	int3
ffff800000201557: cc                   	int3
ffff800000201558: cc                   	int3
ffff800000201559: cc                   	int3
ffff80000020155a: cc                   	int3
ffff80000020155b: cc                   	int3
ffff80000020155c: cc                   	int3
ffff80000020155d: cc                   	int3
ffff80000020155e: cc                   	int3
ffff80000020155f: cc                   	int3
ffff800000201560: cc                   	int3
ffff800000201561: cc                   	int3
ffff800000201562: cc                   	int3
ffff800000201563: cc                   	int3
ffff800000201564: cc                   	int3
ffff800000201565: cc                   	int3
ffff800000201566: cc                   	int3
ffff800000201567: cc                   	int3
ffff800000201568: cc                   	int3
ffff800000201569: cc                   	int3
ffff80000020156a: cc                   	int3
ffff80000020156b: cc                   	int3
ffff80000020156c: cc                   	int3
ffff80000020156d: cc                   	int3
ffff80000020156e: cc                   	int3
ffff80000020156f: cc                   	int3
ffff800000201570: cc                   	int3
ffff800000201571: cc                   	int3
ffff800000201572: cc                   	int3
ffff800000201573: cc                   	int3
ffff800000201574: cc                   	int3
ffff800000201575: cc                   	int3
ffff800000201576: cc                   	int3
ffff800000201577: cc                   	int3
ffff800000201578: cc                   	int3
ffff800000201579: cc                   	int3
ffff80000020157a: cc                   	int3
ffff80000020157b: cc                   	int3
ffff80000020157c: cc                   	int3
ffff80000020157d: cc                   	int3
ffff80000020157e: cc                   	int3
ffff80000020157f: cc                   	int3
ffff800000201580: cc                   	int3
ffff800000201581: cc                   	int3
ffff800000201582: cc                   	int3
ffff800000201583: cc                   	int3
ffff800000201584: cc                   	int3
ffff800000201585: cc                   	int3
ffff800000201586: cc                   	int3
ffff800000201587: cc                   	int3
ffff800000201588: cc                   	int3
ffff800000201589: cc                   	int3
ffff80000020158a: cc                   	int3
ffff80000020158b: cc                   	int3
ffff80000020158c: cc                   	int3
ffff80000020158d: cc                   	int3
ffff80000020158e: cc                   	int3
ffff80000020158f: cc                   	int3
ffff800000201590: cc                   	int3
ffff800000201591: cc                   	int3
ffff800000201592: cc                   	int3
ffff800000201593: cc                   	int3
ffff800000201594: cc                   	int3
ffff800000201595: cc                   	int3
ffff800000201596: cc                   	int3
ffff800000201597: cc                   	int3
ffff800000201598: cc                   	int3
ffff800000201599: cc                   	int3
ffff80000020159a: cc                   	int3
ffff80000020159b: cc                   	int3
ffff80000020159c: cc                   	int3
ffff80000020159d: cc                   	int3
ffff80000020159e: cc                   	int3
ffff80000020159f: cc                   	int3
ffff8000002015a0: cc                   	int3
ffff8000002015a1: cc                   	int3
ffff8000002015a2: cc                   	int3
ffff8000002015a3: cc                   	int3
ffff8000002015a4: cc                   	int3
ffff8000002015a5: cc                   	int3
ffff8000002015a6: cc                   	int3
ffff8000002015a7: cc                   	int3
ffff8000002015a8: cc                   	int3
ffff8000002015a9: cc                   	int3
ffff8000002015aa: cc                   	int3
ffff8000002015ab: cc                   	int3
ffff8000002015ac: cc                   	int3
ffff8000002015ad: cc                   	int3
ffff8000002015ae: cc                   	int3
ffff8000002015af: cc                   	int3
ffff8000002015b0: cc                   	int3
ffff8000002015b1: cc                   	int3
ffff8000002015b2: cc                   	int3
ffff8000002015b3: cc                   	int3
ffff8000002015b4: cc                   	int3
ffff8000002015b5: cc                   	int3
ffff8000002015b6: cc                   	int3
ffff8000002015b7: cc                   	int3
ffff8000002015b8: cc                   	int3
ffff8000002015b9: cc                   	int3
ffff8000002015ba: cc                   	int3
ffff8000002015bb: cc                   	int3
ffff8000002015bc: cc                   	int3
ffff8000002015bd: cc                   	int3
ffff8000002015be: cc                   	int3
ffff8000002015bf: cc                   	int3
ffff8000002015c0: cc                   	int3
ffff8000002015c1: cc                   	int3
ffff8000002015c2: cc                   	int3
ffff8000002015c3: cc                   	int3
ffff8000002015c4: cc                   	int3
ffff8000002015c5: cc                   	int3
ffff8000002015c6: cc                   	int3
ffff8000002015c7: cc                   	int3
ffff8000002015c8: cc                   	int3
ffff8000002015c9: cc                   	int3
ffff8000002015ca: cc                   	int3
ffff8000002015cb: cc                   	int3
ffff8000002015cc: cc                   	int3
ffff8000002015cd: cc                   	int3
ffff8000002015ce: cc                   	int3
ffff8000002015cf: cc                   	int3
ffff8000002015d0: cc                   	int3
ffff8000002015d1: cc                   	int3
ffff8000002015d2: cc                   	int3
ffff8000002015d3: cc                   	int3
ffff8000002015d4: cc                   	int3
ffff8000002015d5: cc                   	int3
ffff8000002015d6: cc                   	int3
ffff8000002015d7: cc                   	int3
ffff8000002015d8: cc                   	int3
ffff8000002015d9: cc                   	int3
ffff8000002015da: cc                   	int3
ffff8000002015db: cc                   	int3
ffff8000002015dc: cc                   	int3
ffff8000002015dd: cc                   	int3
ffff8000002015de: cc                   	int3
ffff8000002015df: cc                   	int3
ffff8000002015e0: cc                   	int3
ffff8000002015e1: cc                   	int3
ffff8000002015e2: cc                   	int3
ffff8000002015e3: cc                   	int3
ffff8000002015e4: cc                   	int3
ffff8000002015e5: cc                   	int3
ffff8000002015e6: cc                   	int3
ffff8000002015e7: cc                   	int3
ffff8000002015e8: cc                   	int3
ffff8000002015e9: cc                   	int3
ffff8000002015ea: cc                   	int3
ffff8000002015eb: cc                   	int3
ffff8000002015ec: cc                   	int3
ffff8000002015ed: cc                   	int3
ffff8000002015ee: cc                   	int3
ffff8000002015ef: cc                   	int3
ffff8000002015f0: cc                   	int3
ffff8000002015f1: cc                   	int3
ffff8000002015f2: cc                   	int3
ffff8000002015f3: cc                   	int3
ffff8000002015f4: cc                   	int3
ffff8000002015f5: cc                   	int3
ffff8000002015f6: cc                   	int3
ffff8000002015f7: cc                   	int3
ffff8000002015f8: cc                   	int3
ffff8000002015f9: cc                   	int3
ffff8000002015fa: cc                   	int3
ffff8000002015fb: cc                   	int3
ffff8000002015fc: cc                   	int3
ffff8000002015fd: cc                   	int3
ffff8000002015fe: cc                   	int3
ffff8000002015ff: cc                   	int3
ffff800000201600: cc                   	int3
ffff800000201601: cc                   	int3
ffff800000201602: cc                   	int3
ffff800000201603: cc                   	int3
ffff800000201604: cc                   	int3
ffff800000201605: cc                   	int3
ffff800000201606: cc                   	int3
ffff800000201607: cc                   	int3
ffff800000201608: cc                   	int3
ffff800000201609: cc                   	int3
ffff80000020160a: cc                   	int3
ffff80000020160b: cc                   	int3
ffff80000020160c: cc                   	int3
ffff80000020160d: cc                   	int3
ffff80000020160e: cc                   	int3
ffff80000020160f: cc                   	int3
ffff800000201610: cc                   	int3
ffff800000201611: cc                   	int3
ffff800000201612: cc                   	int3
ffff800000201613: cc                   	int3
ffff800000201614: cc                   	int3
ffff800000201615: cc                   	int3
ffff800000201616: cc                   	int3
ffff800000201617: cc                   	int3
ffff800000201618: cc                   	int3
ffff800000201619: cc                   	int3
ffff80000020161a: cc                   	int3
ffff80000020161b: cc                   	int3
ffff80000020161c: cc                   	int3
ffff80000020161d: cc                   	int3
ffff80000020161e: cc                   	int3
ffff80000020161f: cc                   	int3
ffff800000201620: cc                   	int3
ffff800000201621: cc                   	int3
ffff800000201622: cc                   	int3
ffff800000201623: cc                   	int3
ffff800000201624: cc                   	int3
ffff800000201625: cc                   	int3
ffff800000201626: cc                   	int3
ffff800000201627: cc                   	int3
ffff800000201628: cc                   	int3
ffff800000201629: cc                   	int3
ffff80000020162a: cc                   	int3
ffff80000020162b: cc                   	int3
ffff80000020162c: cc                   	int3
ffff80000020162d: cc                   	int3
ffff80000020162e: cc                   	int3
ffff80000020162f: cc                   	int3
ffff800000201630: cc                   	int3
ffff800000201631: cc                   	int3
ffff800000201632: cc                   	int3
ffff800000201633: cc                   	int3
ffff800000201634: cc                   	int3
ffff800000201635: cc                   	int3
ffff800000201636: cc                   	int3
ffff800000201637: cc                   	int3
ffff800000201638: cc                   	int3
ffff800000201639: cc                   	int3
ffff80000020163a: cc                   	int3
ffff80000020163b: cc                   	int3
ffff80000020163c: cc                   	int3
ffff80000020163d: cc                   	int3
ffff80000020163e: cc                   	int3
ffff80000020163f: cc                   	int3
ffff800000201640: cc                   	int3
ffff800000201641: cc                   	int3
ffff800000201642: cc                   	int3
ffff800000201643: cc                   	int3
ffff800000201644: cc                   	int3
ffff800000201645: cc                   	int3
ffff800000201646: cc                   	int3
ffff800000201647: cc                   	int3
ffff800000201648: cc                   	int3
ffff800000201649: cc                   	int3
ffff80000020164a: cc                   	int3
ffff80000020164b: cc                   	int3
ffff80000020164c: cc                   	int3
ffff80000020164d: cc                   	int3
ffff80000020164e: cc                   	int3
ffff80000020164f: cc                   	int3
ffff800000201650: cc                   	int3
ffff800000201651: cc                   	int3
ffff800000201652: cc                   	int3
ffff800000201653: cc                   	int3
ffff800000201654: cc                   	int3
ffff800000201655: cc                   	int3
ffff800000201656: cc                   	int3
ffff800000201657: cc                   	int3
ffff800000201658: cc                   	int3
ffff800000201659: cc                   	int3
ffff80000020165a: cc                   	int3
ffff80000020165b: cc                   	int3
ffff80000020165c: cc                   	int3
ffff80000020165d: cc                   	int3
ffff80000020165e: cc                   	int3
ffff80000020165f: cc                   	int3
ffff800000201660: cc                   	int3
ffff800000201661: cc                   	int3
ffff800000201662: cc                   	int3
ffff800000201663: cc                   	int3
ffff800000201664: cc                   	int3
ffff800000201665: cc                   	int3
ffff800000201666: cc                   	int3
ffff800000201667: cc                   	int3
ffff800000201668: cc                   	int3
ffff800000201669: cc                   	int3
ffff80000020166a: cc                   	int3
ffff80000020166b: cc                   	int3
ffff80000020166c: cc                   	int3
ffff80000020166d: cc                   	int3
ffff80000020166e: cc                   	int3
ffff80000020166f: cc                   	int3
ffff800000201670: cc                   	int3
ffff800000201671: cc                   	int3
ffff800000201672: cc                   	int3
ffff800000201673: cc                   	int3
ffff800000201674: cc                   	int3
ffff800000201675: cc                   	int3
ffff800000201676: cc                   	int3
ffff800000201677: cc                   	int3
ffff800000201678: cc                   	int3
ffff800000201679: cc                   	int3
ffff80000020167a: cc                   	int3
ffff80000020167b: cc                   	int3
ffff80000020167c: cc                   	int3
ffff80000020167d: cc                   	int3
ffff80000020167e: cc                   	int3
ffff80000020167f: cc                   	int3
ffff800000201680: cc                   	int3
ffff800000201681: cc                   	int3
ffff800000201682: cc                   	int3
ffff800000201683: cc                   	int3
ffff800000201684: cc                   	int3
ffff800000201685: cc                   	int3
ffff800000201686: cc                   	int3
ffff800000201687: cc                   	int3
ffff800000201688: cc                   	int3
ffff800000201689: cc                   	int3
ffff80000020168a: cc                   	int3
ffff80000020168b: cc                   	int3
ffff80000020168c: cc                   	int3
ffff80000020168d: cc                   	int3
ffff80000020168e: cc                   	int3
ffff80000020168f: cc                   	int3
ffff800000201690: cc                   	int3
ffff800000201691: cc                   	int3
ffff800000201692: cc                   	int3
ffff800000201693: cc                   	int3
ffff800000201694: cc                   	int3
ffff800000201695: cc                   	int3
ffff800000201696: cc                   	int3
ffff800000201697: cc                   	int3
ffff800000201698: cc                   	int3
ffff800000201699: cc                   	int3
ffff80000020169a: cc                   	int3
ffff80000020169b: cc                   	int3
ffff80000020169c: cc                   	int3
ffff80000020169d: cc                   	int3
ffff80000020169e: cc                   	int3
ffff80000020169f: cc                   	int3
ffff8000002016a0: cc                   	int3
ffff8000002016a1: cc                   	int3
ffff8000002016a2: cc                   	int3
ffff8000002016a3: cc                   	int3
ffff8000002016a4: cc                   	int3
ffff8000002016a5: cc                   	int3
ffff8000002016a6: cc                   	int3
ffff8000002016a7: cc                   	int3
ffff8000002016a8: cc                   	int3
ffff8000002016a9: cc                   	int3
ffff8000002016aa: cc                   	int3
ffff8000002016ab: cc                   	int3
ffff8000002016ac: cc                   	int3
ffff8000002016ad: cc                   	int3
ffff8000002016ae: cc                   	int3
ffff8000002016af: cc                   	int3
ffff8000002016b0: cc                   	int3
ffff8000002016b1: cc                   	int3
ffff8000002016b2: cc                   	int3
ffff8000002016b3: cc                   	int3
ffff8000002016b4: cc                   	int3
ffff8000002016b5: cc                   	int3
ffff8000002016b6: cc                   	int3
ffff8000002016b7: cc                   	int3
ffff8000002016b8: cc                   	int3
ffff8000002016b9: cc                   	int3
ffff8000002016ba: cc                   	int3
ffff8000002016bb: cc                   	int3
ffff8000002016bc: cc                   	int3
ffff8000002016bd: cc                   	int3
ffff8000002016be: cc                   	int3
ffff8000002016bf: cc                   	int3
ffff8000002016c0: cc                   	int3
ffff8000002016c1: cc                   	int3
ffff8000002016c2: cc                   	int3
ffff8000002016c3: cc                   	int3
ffff8000002016c4: cc                   	int3
ffff8000002016c5: cc                   	int3
ffff8000002016c6: cc                   	int3
ffff8000002016c7: cc                   	int3
ffff8000002016c8: cc                   	int3
ffff8000002016c9: cc                   	int3
ffff8000002016ca: cc                   	int3
ffff8000002016cb: cc                   	int3
ffff8000002016cc: cc                   	int3
ffff8000002016cd: cc                   	int3
ffff8000002016ce: cc                   	int3
ffff8000002016cf: cc                   	int3
ffff8000002016d0: cc                   	int3
ffff8000002016d1: cc                   	int3
ffff8000002016d2: cc                   	int3
ffff8000002016d3: cc                   	int3
ffff8000002016d4: cc                   	int3
ffff8000002016d5: cc                   	int3
ffff8000002016d6: cc                   	int3
ffff8000002016d7: cc                   	int3
ffff8000002016d8: cc                   	int3
ffff8000002016d9: cc                   	int3
ffff8000002016da: cc                   	int3
ffff8000002016db: cc                   	int3
ffff8000002016dc: cc                   	int3
ffff8000002016dd: cc                   	int3
ffff8000002016de: cc                   	int3
ffff8000002016df: cc                   	int3
ffff8000002016e0: cc                   	int3
ffff8000002016e1: cc                   	int3
ffff8000002016e2: cc                   	int3
ffff8000002016e3: cc                   	int3
ffff8000002016e4: cc                   	int3
ffff8000002016e5: cc                   	int3
ffff8000002016e6: cc                   	int3
ffff8000002016e7: cc                   	int3
ffff8000002016e8: cc                   	int3
ffff8000002016e9: cc                   	int3
ffff8000002016ea: cc                   	int3
ffff8000002016eb: cc                   	int3
ffff8000002016ec: cc                   	int3
ffff8000002016ed: cc                   	int3
ffff8000002016ee: cc                   	int3
ffff8000002016ef: cc                   	int3
ffff8000002016f0: cc                   	int3
ffff8000002016f1: cc                   	int3
ffff8000002016f2: cc                   	int3
ffff8000002016f3: cc                   	int3
ffff8000002016f4: cc                   	int3
ffff8000002016f5: cc                   	int3
ffff8000002016f6: cc                   	int3
ffff8000002016f7: cc                   	int3
ffff8000002016f8: cc                   	int3
ffff8000002016f9: cc                   	int3
ffff8000002016fa: cc                   	int3
ffff8000002016fb: cc                   	int3
ffff8000002016fc: cc                   	int3
ffff8000002016fd: cc                   	int3
ffff8000002016fe: cc                   	int3
ffff8000002016ff: cc                   	int3
ffff800000201700: cc                   	int3
ffff800000201701: cc                   	int3
ffff800000201702: cc                   	int3
ffff800000201703: cc                   	int3
ffff800000201704: cc                   	int3
ffff800000201705: cc                   	int3
ffff800000201706: cc                   	int3
ffff800000201707: cc                   	int3
ffff800000201708: cc                   	int3
ffff800000201709: cc                   	int3
ffff80000020170a: cc                   	int3
ffff80000020170b: cc                   	int3
ffff80000020170c: cc                   	int3
ffff80000020170d: cc                   	int3
ffff80000020170e: cc                   	int3
ffff80000020170f: cc                   	int3
ffff800000201710: cc                   	int3
ffff800000201711: cc                   	int3
ffff800000201712: cc                   	int3
ffff800000201713: cc                   	int3
ffff800000201714: cc                   	int3
ffff800000201715: cc                   	int3
ffff800000201716: cc                   	int3
ffff800000201717: cc                   	int3
ffff800000201718: cc                   	int3
ffff800000201719: cc                   	int3
ffff80000020171a: cc                   	int3
ffff80000020171b: cc                   	int3
ffff80000020171c: cc                   	int3
ffff80000020171d: cc                   	int3
ffff80000020171e: cc                   	int3
ffff80000020171f: cc                   	int3
ffff800000201720: cc                   	int3
ffff800000201721: cc                   	int3
ffff800000201722: cc                   	int3
ffff800000201723: cc                   	int3
ffff800000201724: cc                   	int3
ffff800000201725: cc                   	int3
ffff800000201726: cc                   	int3
ffff800000201727: cc                   	int3
ffff800000201728: cc                   	int3
ffff800000201729: cc                   	int3
ffff80000020172a: cc                   	int3
ffff80000020172b: cc                   	int3
ffff80000020172c: cc                   	int3
ffff80000020172d: cc                   	int3
ffff80000020172e: cc                   	int3
ffff80000020172f: cc                   	int3
ffff800000201730: cc                   	int3
ffff800000201731: cc                   	int3
ffff800000201732: cc                   	int3
ffff800000201733: cc                   	int3
ffff800000201734: cc                   	int3
ffff800000201735: cc                   	int3
ffff800000201736: cc                   	int3
ffff800000201737: cc                   	int3
ffff800000201738: cc                   	int3
ffff800000201739: cc                   	int3
ffff80000020173a: cc                   	int3
ffff80000020173b: cc                   	int3
ffff80000020173c: cc                   	int3
ffff80000020173d: cc                   	int3
ffff80000020173e: cc                   	int3
ffff80000020173f: cc                   	int3
ffff800000201740: cc                   	int3
ffff800000201741: cc                   	int3
ffff800000201742: cc                   	int3
ffff800000201743: cc                   	int3
ffff800000201744: cc                   	int3
ffff800000201745: cc                   	int3
ffff800000201746: cc                   	int3
ffff800000201747: cc                   	int3
ffff800000201748: cc                   	int3
ffff800000201749: cc                   	int3
ffff80000020174a: cc                   	int3
ffff80000020174b: cc                   	int3
ffff80000020174c: cc                   	int3
ffff80000020174d: cc                   	int3
ffff80000020174e: cc                   	int3
ffff80000020174f: cc                   	int3
ffff800000201750: cc                   	int3
ffff800000201751: cc                   	int3
ffff800000201752: cc                   	int3
ffff800000201753: cc                   	int3
ffff800000201754: cc                   	int3
ffff800000201755: cc                   	int3
ffff800000201756: cc                   	int3
ffff800000201757: cc                   	int3
ffff800000201758: cc                   	int3
ffff800000201759: cc                   	int3
ffff80000020175a: cc                   	int3
ffff80000020175b: cc                   	int3
ffff80000020175c: cc                   	int3
ffff80000020175d: cc                   	int3
ffff80000020175e: cc                   	int3
ffff80000020175f: cc                   	int3
ffff800000201760: cc                   	int3
ffff800000201761: cc                   	int3
ffff800000201762: cc                   	int3
ffff800000201763: cc                   	int3
ffff800000201764: cc                   	int3
ffff800000201765: cc                   	int3
ffff800000201766: cc                   	int3
ffff800000201767: cc                   	int3
ffff800000201768: cc                   	int3
ffff800000201769: cc                   	int3
ffff80000020176a: cc                   	int3
ffff80000020176b: cc                   	int3
ffff80000020176c: cc                   	int3
ffff80000020176d: cc                   	int3
ffff80000020176e: cc                   	int3
ffff80000020176f: cc                   	int3
ffff800000201770: cc                   	int3
ffff800000201771: cc                   	int3
ffff800000201772: cc                   	int3
ffff800000201773: cc                   	int3
ffff800000201774: cc                   	int3
ffff800000201775: cc                   	int3
ffff800000201776: cc                   	int3
ffff800000201777: cc                   	int3
ffff800000201778: cc                   	int3
ffff800000201779: cc                   	int3
ffff80000020177a: cc                   	int3
ffff80000020177b: cc                   	int3
ffff80000020177c: cc                   	int3
ffff80000020177d: cc                   	int3
ffff80000020177e: cc                   	int3
ffff80000020177f: cc                   	int3
ffff800000201780: cc                   	int3
ffff800000201781: cc                   	int3
ffff800000201782: cc                   	int3
ffff800000201783: cc                   	int3
ffff800000201784: cc                   	int3
ffff800000201785: cc                   	int3
ffff800000201786: cc                   	int3
ffff800000201787: cc                   	int3
ffff800000201788: cc                   	int3
ffff800000201789: cc                   	int3
ffff80000020178a: cc                   	int3
ffff80000020178b: cc                   	int3
ffff80000020178c: cc                   	int3
ffff80000020178d: cc                   	int3
ffff80000020178e: cc                   	int3
ffff80000020178f: cc                   	int3
ffff800000201790: cc                   	int3
ffff800000201791: cc                   	int3
ffff800000201792: cc                   	int3
ffff800000201793: cc                   	int3
ffff800000201794: cc                   	int3
ffff800000201795: cc                   	int3
ffff800000201796: cc                   	int3
ffff800000201797: cc                   	int3
ffff800000201798: cc                   	int3
ffff800000201799: cc                   	int3
ffff80000020179a: cc                   	int3
ffff80000020179b: cc                   	int3
ffff80000020179c: cc                   	int3
ffff80000020179d: cc                   	int3
ffff80000020179e: cc                   	int3
ffff80000020179f: cc                   	int3
ffff8000002017a0: cc                   	int3
ffff8000002017a1: cc                   	int3
ffff8000002017a2: cc                   	int3
ffff8000002017a3: cc                   	int3
ffff8000002017a4: cc                   	int3
ffff8000002017a5: cc                   	int3
ffff8000002017a6: cc                   	int3
ffff8000002017a7: cc                   	int3
ffff8000002017a8: cc                   	int3
ffff8000002017a9: cc                   	int3
ffff8000002017aa: cc                   	int3
ffff8000002017ab: cc                   	int3
ffff8000002017ac: cc                   	int3
ffff8000002017ad: cc                   	int3
ffff8000002017ae: cc                   	int3
ffff8000002017af: cc                   	int3
ffff8000002017b0: cc                   	int3
ffff8000002017b1: cc                   	int3
ffff8000002017b2: cc                   	int3
ffff8000002017b3: cc                   	int3
ffff8000002017b4: cc                   	int3
ffff8000002017b5: cc                   	int3
ffff8000002017b6: cc                   	int3
ffff8000002017b7: cc                   	int3
ffff8000002017b8: cc                   	int3
ffff8000002017b9: cc                   	int3
ffff8000002017ba: cc                   	int3
ffff8000002017bb: cc                   	int3
ffff8000002017bc: cc                   	int3
ffff8000002017bd: cc                   	int3
ffff8000002017be: cc                   	int3
ffff8000002017bf: cc                   	int3
ffff8000002017c0: cc                   	int3
ffff8000002017c1: cc                   	int3
ffff8000002017c2: cc                   	int3
ffff8000002017c3: cc                   	int3
ffff8000002017c4: cc                   	int3
ffff8000002017c5: cc                   	int3
ffff8000002017c6: cc                   	int3
ffff8000002017c7: cc                   	int3
ffff8000002017c8: cc                   	int3
ffff8000002017c9: cc                   	int3
ffff8000002017ca: cc                   	int3
ffff8000002017cb: cc                   	int3
ffff8000002017cc: cc                   	int3
ffff8000002017cd: cc                   	int3
ffff8000002017ce: cc                   	int3
ffff8000002017cf: cc                   	int3
ffff8000002017d0: cc                   	int3
ffff8000002017d1: cc                   	int3
ffff8000002017d2: cc                   	int3
ffff8000002017d3: cc                   	int3
ffff8000002017d4: cc                   	int3
ffff8000002017d5: cc                   	int3
ffff8000002017d6: cc                   	int3
ffff8000002017d7: cc                   	int3
ffff8000002017d8: cc                   	int3
ffff8000002017d9: cc                   	int3
ffff8000002017da: cc                   	int3
ffff8000002017db: cc                   	int3
ffff8000002017dc: cc                   	int3
ffff8000002017dd: cc                   	int3
ffff8000002017de: cc                   	int3
ffff8000002017df: cc                   	int3
ffff8000002017e0: cc                   	int3
ffff8000002017e1: cc                   	int3
ffff8000002017e2: cc                   	int3
ffff8000002017e3: cc                   	int3
ffff8000002017e4: cc                   	int3
ffff8000002017e5: cc                   	int3
ffff8000002017e6: cc                   	int3
ffff8000002017e7: cc                   	int3
ffff8000002017e8: cc                   	int3
ffff8000002017e9: cc                   	int3
ffff8000002017ea: cc                   	int3
ffff8000002017eb: cc                   	int3
ffff8000002017ec: cc                   	int3
ffff8000002017ed: cc                   	int3
ffff8000002017ee: cc                   	int3
ffff8000002017ef: cc                   	int3
ffff8000002017f0: cc                   	int3
ffff8000002017f1: cc                   	int3
ffff8000002017f2: cc                   	int3
ffff8000002017f3: cc                   	int3
ffff8000002017f4: cc                   	int3
ffff8000002017f5: cc                   	int3
ffff8000002017f6: cc                   	int3
ffff8000002017f7: cc                   	int3
ffff8000002017f8: cc                   	int3
ffff8000002017f9: cc                   	int3
ffff8000002017fa: cc                   	int3
ffff8000002017fb: cc                   	int3
ffff8000002017fc: cc                   	int3
ffff8000002017fd: cc                   	int3
ffff8000002017fe: cc                   	int3
ffff8000002017ff: cc                   	int3
ffff800000201800: cc                   	int3
ffff800000201801: cc                   	int3
ffff800000201802: cc                   	int3
ffff800000201803: cc                   	int3
ffff800000201804: cc                   	int3
ffff800000201805: cc                   	int3
ffff800000201806: cc                   	int3
ffff800000201807: cc                   	int3
ffff800000201808: cc                   	int3
ffff800000201809: cc                   	int3
ffff80000020180a: cc                   	int3
ffff80000020180b: cc                   	int3
ffff80000020180c: cc                   	int3
ffff80000020180d: cc                   	int3
ffff80000020180e: cc                   	int3
ffff80000020180f: cc                   	int3
ffff800000201810: cc                   	int3
ffff800000201811: cc                   	int3
ffff800000201812: cc                   	int3
ffff800000201813: cc                   	int3
ffff800000201814: cc                   	int3
ffff800000201815: cc                   	int3
ffff800000201816: cc                   	int3
ffff800000201817: cc                   	int3
ffff800000201818: cc                   	int3
ffff800000201819: cc                   	int3
ffff80000020181a: cc                   	int3
ffff80000020181b: cc                   	int3
ffff80000020181c: cc                   	int3
ffff80000020181d: cc                   	int3
ffff80000020181e: cc                   	int3
ffff80000020181f: cc                   	int3
ffff800000201820: cc                   	int3
ffff800000201821: cc                   	int3
ffff800000201822: cc                   	int3
ffff800000201823: cc                   	int3
ffff800000201824: cc                   	int3
ffff800000201825: cc                   	int3
ffff800000201826: cc                   	int3
ffff800000201827: cc                   	int3
ffff800000201828: cc                   	int3
ffff800000201829: cc                   	int3
ffff80000020182a: cc                   	int3
ffff80000020182b: cc                   	int3
ffff80000020182c: cc                   	int3
ffff80000020182d: cc                   	int3
ffff80000020182e: cc                   	int3
ffff80000020182f: cc                   	int3
ffff800000201830: cc                   	int3
ffff800000201831: cc                   	int3
ffff800000201832: cc                   	int3
ffff800000201833: cc                   	int3
ffff800000201834: cc                   	int3
ffff800000201835: cc                   	int3
ffff800000201836: cc                   	int3
ffff800000201837: cc                   	int3
ffff800000201838: cc                   	int3
ffff800000201839: cc                   	int3
ffff80000020183a: cc                   	int3
ffff80000020183b: cc                   	int3
ffff80000020183c: cc                   	int3
ffff80000020183d: cc                   	int3
ffff80000020183e: cc                   	int3
ffff80000020183f: cc                   	int3
ffff800000201840: cc                   	int3
ffff800000201841: cc                   	int3
ffff800000201842: cc                   	int3
ffff800000201843: cc                   	int3
ffff800000201844: cc                   	int3
ffff800000201845: cc                   	int3
ffff800000201846: cc                   	int3
ffff800000201847: cc                   	int3
ffff800000201848: cc                   	int3
ffff800000201849: cc                   	int3
ffff80000020184a: cc                   	int3
ffff80000020184b: cc                   	int3
ffff80000020184c: cc                   	int3
ffff80000020184d: cc                   	int3
ffff80000020184e: cc                   	int3
ffff80000020184f: cc                   	int3
ffff800000201850: cc                   	int3
ffff800000201851: cc                   	int3
ffff800000201852: cc                   	int3
ffff800000201853: cc                   	int3
ffff800000201854: cc                   	int3
ffff800000201855: cc                   	int3
ffff800000201856: cc                   	int3
ffff800000201857: cc                   	int3
ffff800000201858: cc                   	int3
ffff800000201859: cc                   	int3
ffff80000020185a: cc                   	int3
ffff80000020185b: cc                   	int3
ffff80000020185c: cc                   	int3
ffff80000020185d: cc                   	int3
ffff80000020185e: cc                   	int3
ffff80000020185f: cc                   	int3
ffff800000201860: cc                   	int3
ffff800000201861: cc                   	int3
ffff800000201862: cc                   	int3
ffff800000201863: cc                   	int3
ffff800000201864: cc                   	int3
ffff800000201865: cc                   	int3
ffff800000201866: cc                   	int3
ffff800000201867: cc                   	int3
ffff800000201868: cc                   	int3
ffff800000201869: cc                   	int3
ffff80000020186a: cc                   	int3
ffff80000020186b: cc                   	int3
ffff80000020186c: cc                   	int3
ffff80000020186d: cc                   	int3
ffff80000020186e: cc                   	int3
ffff80000020186f: cc                   	int3
ffff800000201870: cc                   	int3
ffff800000201871: cc                   	int3
ffff800000201872: cc                   	int3
ffff800000201873: cc                   	int3
ffff800000201874: cc                   	int3
ffff800000201875: cc                   	int3
ffff800000201876: cc                   	int3
ffff800000201877: cc                   	int3
ffff800000201878: cc                   	int3
ffff800000201879: cc                   	int3
ffff80000020187a: cc                   	int3
ffff80000020187b: cc                   	int3
ffff80000020187c: cc                   	int3
ffff80000020187d: cc                   	int3
ffff80000020187e: cc                   	int3
ffff80000020187f: cc                   	int3
ffff800000201880: cc                   	int3
ffff800000201881: cc                   	int3
ffff800000201882: cc                   	int3
ffff800000201883: cc                   	int3
ffff800000201884: cc                   	int3
ffff800000201885: cc                   	int3
ffff800000201886: cc                   	int3
ffff800000201887: cc                   	int3
ffff800000201888: cc                   	int3
ffff800000201889: cc                   	int3
ffff80000020188a: cc                   	int3
ffff80000020188b: cc                   	int3
ffff80000020188c: cc                   	int3
ffff80000020188d: cc                   	int3
ffff80000020188e: cc                   	int3
ffff80000020188f: cc                   	int3
ffff800000201890: cc                   	int3
ffff800000201891: cc                   	int3
ffff800000201892: cc                   	int3
ffff800000201893: cc                   	int3
ffff800000201894: cc                   	int3
ffff800000201895: cc                   	int3
ffff800000201896: cc                   	int3
ffff800000201897: cc                   	int3
ffff800000201898: cc                   	int3
ffff800000201899: cc                   	int3
ffff80000020189a: cc                   	int3
ffff80000020189b: cc                   	int3
ffff80000020189c: cc                   	int3
ffff80000020189d: cc                   	int3
ffff80000020189e: cc                   	int3
ffff80000020189f: cc                   	int3
ffff8000002018a0: cc                   	int3
ffff8000002018a1: cc                   	int3
ffff8000002018a2: cc                   	int3
ffff8000002018a3: cc                   	int3
ffff8000002018a4: cc                   	int3
ffff8000002018a5: cc                   	int3
ffff8000002018a6: cc                   	int3
ffff8000002018a7: cc                   	int3
ffff8000002018a8: cc                   	int3
ffff8000002018a9: cc                   	int3
ffff8000002018aa: cc                   	int3
ffff8000002018ab: cc                   	int3
ffff8000002018ac: cc                   	int3
ffff8000002018ad: cc                   	int3
ffff8000002018ae: cc                   	int3
ffff8000002018af: cc                   	int3
ffff8000002018b0: cc                   	int3
ffff8000002018b1: cc                   	int3
ffff8000002018b2: cc                   	int3
ffff8000002018b3: cc                   	int3
ffff8000002018b4: cc                   	int3
ffff8000002018b5: cc                   	int3
ffff8000002018b6: cc                   	int3
ffff8000002018b7: cc                   	int3
ffff8000002018b8: cc                   	int3
ffff8000002018b9: cc                   	int3
ffff8000002018ba: cc                   	int3
ffff8000002018bb: cc                   	int3
ffff8000002018bc: cc                   	int3
ffff8000002018bd: cc                   	int3
ffff8000002018be: cc                   	int3
ffff8000002018bf: cc                   	int3
ffff8000002018c0: cc                   	int3
ffff8000002018c1: cc                   	int3
ffff8000002018c2: cc                   	int3
ffff8000002018c3: cc                   	int3
ffff8000002018c4: cc                   	int3
ffff8000002018c5: cc                   	int3
ffff8000002018c6: cc                   	int3
ffff8000002018c7: cc                   	int3
ffff8000002018c8: cc                   	int3
ffff8000002018c9: cc                   	int3
ffff8000002018ca: cc                   	int3
ffff8000002018cb: cc                   	int3
ffff8000002018cc: cc                   	int3
ffff8000002018cd: cc                   	int3
ffff8000002018ce: cc                   	int3
ffff8000002018cf: cc                   	int3
ffff8000002018d0: cc                   	int3
ffff8000002018d1: cc                   	int3
ffff8000002018d2: cc                   	int3
ffff8000002018d3: cc                   	int3
ffff8000002018d4: cc                   	int3
ffff8000002018d5: cc                   	int3
ffff8000002018d6: cc                   	int3
ffff8000002018d7: cc                   	int3
ffff8000002018d8: cc                   	int3
ffff8000002018d9: cc                   	int3
ffff8000002018da: cc                   	int3
ffff8000002018db: cc                   	int3
ffff8000002018dc: cc                   	int3
ffff8000002018dd: cc                   	int3
ffff8000002018de: cc                   	int3
ffff8000002018df: cc                   	int3
ffff8000002018e0: cc                   	int3
ffff8000002018e1: cc                   	int3
ffff8000002018e2: cc                   	int3
ffff8000002018e3: cc                   	int3
ffff8000002018e4: cc                   	int3
ffff8000002018e5: cc                   	int3
ffff8000002018e6: cc                   	int3
ffff8000002018e7: cc                   	int3
ffff8000002018e8: cc                   	int3
ffff8000002018e9: cc                   	int3
ffff8000002018ea: cc                   	int3
ffff8000002018eb: cc                   	int3
ffff8000002018ec: cc                   	int3
ffff8000002018ed: cc                   	int3
ffff8000002018ee: cc                   	int3
ffff8000002018ef: cc                   	int3
ffff8000002018f0: cc                   	int3
ffff8000002018f1: cc                   	int3
ffff8000002018f2: cc                   	int3
ffff8000002018f3: cc                   	int3
ffff8000002018f4: cc                   	int3
ffff8000002018f5: cc                   	int3
ffff8000002018f6: cc                   	int3
ffff8000002018f7: cc                   	int3
ffff8000002018f8: cc                   	int3
ffff8000002018f9: cc                   	int3
ffff8000002018fa: cc                   	int3
ffff8000002018fb: cc                   	int3
ffff8000002018fc: cc                   	int3
ffff8000002018fd: cc                   	int3
ffff8000002018fe: cc                   	int3
ffff8000002018ff: cc                   	int3
ffff800000201900: cc                   	int3
ffff800000201901: cc                   	int3
ffff800000201902: cc                   	int3
ffff800000201903: cc                   	int3
ffff800000201904: cc                   	int3
ffff800000201905: cc                   	int3
ffff800000201906: cc                   	int3
ffff800000201907: cc                   	int3
ffff800000201908: cc                   	int3
ffff800000201909: cc                   	int3
ffff80000020190a: cc                   	int3
ffff80000020190b: cc                   	int3
ffff80000020190c: cc                   	int3
ffff80000020190d: cc                   	int3
ffff80000020190e: cc                   	int3
ffff80000020190f: cc                   	int3
ffff800000201910: cc                   	int3
ffff800000201911: cc                   	int3
ffff800000201912: cc                   	int3
ffff800000201913: cc                   	int3
ffff800000201914: cc                   	int3
ffff800000201915: cc                   	int3
ffff800000201916: cc                   	int3
ffff800000201917: cc                   	int3
ffff800000201918: cc                   	int3
ffff800000201919: cc                   	int3
ffff80000020191a: cc                   	int3
ffff80000020191b: cc                   	int3
ffff80000020191c: cc                   	int3
ffff80000020191d: cc                   	int3
ffff80000020191e: cc                   	int3
ffff80000020191f: cc                   	int3
ffff800000201920: cc                   	int3
ffff800000201921: cc                   	int3
ffff800000201922: cc                   	int3
ffff800000201923: cc                   	int3
ffff800000201924: cc                   	int3
ffff800000201925: cc                   	int3
ffff800000201926: cc                   	int3
ffff800000201927: cc                   	int3
ffff800000201928: cc                   	int3
ffff800000201929: cc                   	int3
ffff80000020192a: cc                   	int3
ffff80000020192b: cc                   	int3
ffff80000020192c: cc                   	int3
ffff80000020192d: cc                   	int3
ffff80000020192e: cc                   	int3
ffff80000020192f: cc                   	int3
ffff800000201930: cc                   	int3
ffff800000201931: cc                   	int3
ffff800000201932: cc                   	int3
ffff800000201933: cc                   	int3
ffff800000201934: cc                   	int3
ffff800000201935: cc                   	int3
ffff800000201936: cc                   	int3
ffff800000201937: cc                   	int3
ffff800000201938: cc                   	int3
ffff800000201939: cc                   	int3
ffff80000020193a: cc                   	int3
ffff80000020193b: cc                   	int3
ffff80000020193c: cc                   	int3
ffff80000020193d: cc                   	int3
ffff80000020193e: cc                   	int3
ffff80000020193f: cc                   	int3
ffff800000201940: cc                   	int3
ffff800000201941: cc                   	int3
ffff800000201942: cc                   	int3
ffff800000201943: cc                   	int3
ffff800000201944: cc                   	int3
ffff800000201945: cc                   	int3
ffff800000201946: cc                   	int3
ffff800000201947: cc                   	int3
ffff800000201948: cc                   	int3
ffff800000201949: cc                   	int3
ffff80000020194a: cc                   	int3
ffff80000020194b: cc                   	int3
ffff80000020194c: cc                   	int3
ffff80000020194d: cc                   	int3
ffff80000020194e: cc                   	int3
ffff80000020194f: cc                   	int3
ffff800000201950: cc                   	int3
ffff800000201951: cc                   	int3
ffff800000201952: cc                   	int3
ffff800000201953: cc                   	int3
ffff800000201954: cc                   	int3
ffff800000201955: cc                   	int3
ffff800000201956: cc                   	int3
ffff800000201957: cc                   	int3
ffff800000201958: cc                   	int3
ffff800000201959: cc                   	int3
ffff80000020195a: cc                   	int3
ffff80000020195b: cc                   	int3
ffff80000020195c: cc                   	int3
ffff80000020195d: cc                   	int3
ffff80000020195e: cc                   	int3
ffff80000020195f: cc                   	int3
ffff800000201960: cc                   	int3
ffff800000201961: cc                   	int3
ffff800000201962: cc                   	int3
ffff800000201963: cc                   	int3
ffff800000201964: cc                   	int3
ffff800000201965: cc                   	int3
ffff800000201966: cc                   	int3
ffff800000201967: cc                   	int3
ffff800000201968: cc                   	int3
ffff800000201969: cc                   	int3
ffff80000020196a: cc                   	int3
ffff80000020196b: cc                   	int3
ffff80000020196c: cc                   	int3
ffff80000020196d: cc                   	int3
ffff80000020196e: cc                   	int3
ffff80000020196f: cc                   	int3
ffff800000201970: cc                   	int3
ffff800000201971: cc                   	int3
ffff800000201972: cc                   	int3
ffff800000201973: cc                   	int3
ffff800000201974: cc                   	int3
ffff800000201975: cc                   	int3
ffff800000201976: cc                   	int3
ffff800000201977: cc                   	int3
ffff800000201978: cc                   	int3
ffff800000201979: cc                   	int3
ffff80000020197a: cc                   	int3
ffff80000020197b: cc                   	int3
ffff80000020197c: cc                   	int3
ffff80000020197d: cc                   	int3
ffff80000020197e: cc                   	int3
ffff80000020197f: cc                   	int3
ffff800000201980: cc                   	int3
ffff800000201981: cc                   	int3
ffff800000201982: cc                   	int3
ffff800000201983: cc                   	int3
ffff800000201984: cc                   	int3
ffff800000201985: cc                   	int3
ffff800000201986: cc                   	int3
ffff800000201987: cc                   	int3
ffff800000201988: cc                   	int3
ffff800000201989: cc                   	int3
ffff80000020198a: cc                   	int3
ffff80000020198b: cc                   	int3
ffff80000020198c: cc                   	int3
ffff80000020198d: cc                   	int3
ffff80000020198e: cc                   	int3
ffff80000020198f: cc                   	int3
ffff800000201990: cc                   	int3
ffff800000201991: cc                   	int3
ffff800000201992: cc                   	int3
ffff800000201993: cc                   	int3
ffff800000201994: cc                   	int3
ffff800000201995: cc                   	int3
ffff800000201996: cc                   	int3
ffff800000201997: cc                   	int3
ffff800000201998: cc                   	int3
ffff800000201999: cc                   	int3
ffff80000020199a: cc                   	int3
ffff80000020199b: cc                   	int3
ffff80000020199c: cc                   	int3
ffff80000020199d: cc                   	int3
ffff80000020199e: cc                   	int3
ffff80000020199f: cc                   	int3
ffff8000002019a0: cc                   	int3
ffff8000002019a1: cc                   	int3
ffff8000002019a2: cc                   	int3
ffff8000002019a3: cc                   	int3
ffff8000002019a4: cc                   	int3
ffff8000002019a5: cc                   	int3
ffff8000002019a6: cc                   	int3
ffff8000002019a7: cc                   	int3
ffff8000002019a8: cc                   	int3
ffff8000002019a9: cc                   	int3
ffff8000002019aa: cc                   	int3
ffff8000002019ab: cc                   	int3
ffff8000002019ac: cc                   	int3
ffff8000002019ad: cc                   	int3
ffff8000002019ae: cc                   	int3
ffff8000002019af: cc                   	int3
ffff8000002019b0: cc                   	int3
ffff8000002019b1: cc                   	int3
ffff8000002019b2: cc                   	int3
ffff8000002019b3: cc                   	int3
ffff8000002019b4: cc                   	int3
ffff8000002019b5: cc                   	int3
ffff8000002019b6: cc                   	int3
ffff8000002019b7: cc                   	int3
ffff8000002019b8: cc                   	int3
ffff8000002019b9: cc                   	int3
ffff8000002019ba: cc                   	int3
ffff8000002019bb: cc                   	int3
ffff8000002019bc: cc                   	int3
ffff8000002019bd: cc                   	int3
ffff8000002019be: cc                   	int3
ffff8000002019bf: cc                   	int3
ffff8000002019c0: cc                   	int3
ffff8000002019c1: cc                   	int3
ffff8000002019c2: cc                   	int3
ffff8000002019c3: cc                   	int3
ffff8000002019c4: cc                   	int3
ffff8000002019c5: cc                   	int3
ffff8000002019c6: cc                   	int3
ffff8000002019c7: cc                   	int3
ffff8000002019c8: cc                   	int3
ffff8000002019c9: cc                   	int3
ffff8000002019ca: cc                   	int3
ffff8000002019cb: cc                   	int3
ffff8000002019cc: cc                   	int3
ffff8000002019cd: cc                   	int3
ffff8000002019ce: cc                   	int3
ffff8000002019cf: cc                   	int3
ffff8000002019d0: cc                   	int3
ffff8000002019d1: cc                   	int3
ffff8000002019d2: cc                   	int3
ffff8000002019d3: cc                   	int3
ffff8000002019d4: cc                   	int3
ffff8000002019d5: cc                   	int3
ffff8000002019d6: cc                   	int3
ffff8000002019d7: cc                   	int3
ffff8000002019d8: cc                   	int3
ffff8000002019d9: cc                   	int3
ffff8000002019da: cc                   	int3
ffff8000002019db: cc                   	int3
ffff8000002019dc: cc                   	int3
ffff8000002019dd: cc                   	int3
ffff8000002019de: cc                   	int3
ffff8000002019df: cc                   	int3
ffff8000002019e0: cc                   	int3
ffff8000002019e1: cc                   	int3
ffff8000002019e2: cc                   	int3
ffff8000002019e3: cc                   	int3
ffff8000002019e4: cc                   	int3
ffff8000002019e5: cc                   	int3
ffff8000002019e6: cc                   	int3
ffff8000002019e7: cc                   	int3
ffff8000002019e8: cc                   	int3
ffff8000002019e9: cc                   	int3
ffff8000002019ea: cc                   	int3
ffff8000002019eb: cc                   	int3
ffff8000002019ec: cc                   	int3
ffff8000002019ed: cc                   	int3
ffff8000002019ee: cc                   	int3
ffff8000002019ef: cc                   	int3
ffff8000002019f0: cc                   	int3
ffff8000002019f1: cc                   	int3
ffff8000002019f2: cc                   	int3
ffff8000002019f3: cc                   	int3
ffff8000002019f4: cc                   	int3
ffff8000002019f5: cc                   	int3
ffff8000002019f6: cc                   	int3
ffff8000002019f7: cc                   	int3
ffff8000002019f8: cc                   	int3
ffff8000002019f9: cc                   	int3
ffff8000002019fa: cc                   	int3
ffff8000002019fb: cc                   	int3
ffff8000002019fc: cc                   	int3
ffff8000002019fd: cc                   	int3
ffff8000002019fe: cc                   	int3
ffff8000002019ff: cc                   	int3
ffff800000201a00: cc                   	int3
ffff800000201a01: cc                   	int3
ffff800000201a02: cc                   	int3
ffff800000201a03: cc                   	int3
ffff800000201a04: cc                   	int3
ffff800000201a05: cc                   	int3
ffff800000201a06: cc                   	int3
ffff800000201a07: cc                   	int3
ffff800000201a08: cc                   	int3
ffff800000201a09: cc                   	int3
ffff800000201a0a: cc                   	int3
ffff800000201a0b: cc                   	int3
ffff800000201a0c: cc                   	int3
ffff800000201a0d: cc                   	int3
ffff800000201a0e: cc                   	int3
ffff800000201a0f: cc                   	int3
ffff800000201a10: cc                   	int3
ffff800000201a11: cc                   	int3
ffff800000201a12: cc                   	int3
ffff800000201a13: cc                   	int3
ffff800000201a14: cc                   	int3
ffff800000201a15: cc                   	int3
ffff800000201a16: cc                   	int3
ffff800000201a17: cc                   	int3
ffff800000201a18: cc                   	int3
ffff800000201a19: cc                   	int3
ffff800000201a1a: cc                   	int3
ffff800000201a1b: cc                   	int3
ffff800000201a1c: cc                   	int3
ffff800000201a1d: cc                   	int3
ffff800000201a1e: cc                   	int3
ffff800000201a1f: cc                   	int3
ffff800000201a20: cc                   	int3
ffff800000201a21: cc                   	int3
ffff800000201a22: cc                   	int3
ffff800000201a23: cc                   	int3
ffff800000201a24: cc                   	int3
ffff800000201a25: cc                   	int3
ffff800000201a26: cc                   	int3
ffff800000201a27: cc                   	int3
ffff800000201a28: cc                   	int3
ffff800000201a29: cc                   	int3
ffff800000201a2a: cc                   	int3
ffff800000201a2b: cc                   	int3
ffff800000201a2c: cc                   	int3
ffff800000201a2d: cc                   	int3
ffff800000201a2e: cc                   	int3
ffff800000201a2f: cc                   	int3
ffff800000201a30: cc                   	int3
ffff800000201a31: cc                   	int3
ffff800000201a32: cc                   	int3
ffff800000201a33: cc                   	int3
ffff800000201a34: cc                   	int3
ffff800000201a35: cc                   	int3
ffff800000201a36: cc                   	int3
ffff800000201a37: cc                   	int3
ffff800000201a38: cc                   	int3
ffff800000201a39: cc                   	int3
ffff800000201a3a: cc                   	int3
ffff800000201a3b: cc                   	int3
ffff800000201a3c: cc                   	int3
ffff800000201a3d: cc                   	int3
ffff800000201a3e: cc                   	int3
ffff800000201a3f: cc                   	int3
ffff800000201a40: cc                   	int3
ffff800000201a41: cc                   	int3
ffff800000201a42: cc                   	int3
ffff800000201a43: cc                   	int3
ffff800000201a44: cc                   	int3
ffff800000201a45: cc                   	int3
ffff800000201a46: cc                   	int3
ffff800000201a47: cc                   	int3
ffff800000201a48: cc                   	int3
ffff800000201a49: cc                   	int3
ffff800000201a4a: cc                   	int3
ffff800000201a4b: cc                   	int3
ffff800000201a4c: cc                   	int3
ffff800000201a4d: cc                   	int3
ffff800000201a4e: cc                   	int3
ffff800000201a4f: cc                   	int3
ffff800000201a50: cc                   	int3
ffff800000201a51: cc                   	int3
ffff800000201a52: cc                   	int3
ffff800000201a53: cc                   	int3
ffff800000201a54: cc                   	int3
ffff800000201a55: cc                   	int3
ffff800000201a56: cc                   	int3
ffff800000201a57: cc                   	int3
ffff800000201a58: cc                   	int3
ffff800000201a59: cc                   	int3
ffff800000201a5a: cc                   	int3
ffff800000201a5b: cc                   	int3
ffff800000201a5c: cc                   	int3
ffff800000201a5d: cc                   	int3
ffff800000201a5e: cc                   	int3
ffff800000201a5f: cc                   	int3
ffff800000201a60: cc                   	int3
ffff800000201a61: cc                   	int3
ffff800000201a62: cc                   	int3
ffff800000201a63: cc                   	int3
ffff800000201a64: cc                   	int3
ffff800000201a65: cc                   	int3
ffff800000201a66: cc                   	int3
ffff800000201a67: cc                   	int3
ffff800000201a68: cc                   	int3
ffff800000201a69: cc                   	int3
ffff800000201a6a: cc                   	int3
ffff800000201a6b: cc                   	int3
ffff800000201a6c: cc                   	int3
ffff800000201a6d: cc                   	int3
ffff800000201a6e: cc                   	int3
ffff800000201a6f: cc                   	int3
ffff800000201a70: cc                   	int3
ffff800000201a71: cc                   	int3
ffff800000201a72: cc                   	int3
ffff800000201a73: cc                   	int3
ffff800000201a74: cc                   	int3
ffff800000201a75: cc                   	int3
ffff800000201a76: cc                   	int3
ffff800000201a77: cc                   	int3
ffff800000201a78: cc                   	int3
ffff800000201a79: cc                   	int3
ffff800000201a7a: cc                   	int3
ffff800000201a7b: cc                   	int3
ffff800000201a7c: cc                   	int3
ffff800000201a7d: cc                   	int3
ffff800000201a7e: cc                   	int3
ffff800000201a7f: cc                   	int3
ffff800000201a80: cc                   	int3
ffff800000201a81: cc                   	int3
ffff800000201a82: cc                   	int3
ffff800000201a83: cc                   	int3
ffff800000201a84: cc                   	int3
ffff800000201a85: cc                   	int3
ffff800000201a86: cc                   	int3
ffff800000201a87: cc                   	int3
ffff800000201a88: cc                   	int3
ffff800000201a89: cc                   	int3
ffff800000201a8a: cc                   	int3
ffff800000201a8b: cc                   	int3
ffff800000201a8c: cc                   	int3
ffff800000201a8d: cc                   	int3
ffff800000201a8e: cc                   	int3
ffff800000201a8f: cc                   	int3
ffff800000201a90: cc                   	int3
ffff800000201a91: cc                   	int3
ffff800000201a92: cc                   	int3
ffff800000201a93: cc                   	int3
ffff800000201a94: cc                   	int3
ffff800000201a95: cc                   	int3
ffff800000201a96: cc                   	int3
ffff800000201a97: cc                   	int3
ffff800000201a98: cc                   	int3
ffff800000201a99: cc                   	int3
ffff800000201a9a: cc                   	int3
ffff800000201a9b: cc                   	int3
ffff800000201a9c: cc                   	int3
ffff800000201a9d: cc                   	int3
ffff800000201a9e: cc                   	int3
ffff800000201a9f: cc                   	int3
ffff800000201aa0: cc                   	int3
ffff800000201aa1: cc                   	int3
ffff800000201aa2: cc                   	int3
ffff800000201aa3: cc                   	int3
ffff800000201aa4: cc                   	int3
ffff800000201aa5: cc                   	int3
ffff800000201aa6: cc                   	int3
ffff800000201aa7: cc                   	int3
ffff800000201aa8: cc                   	int3
ffff800000201aa9: cc                   	int3
ffff800000201aaa: cc                   	int3
ffff800000201aab: cc                   	int3
ffff800000201aac: cc                   	int3
ffff800000201aad: cc                   	int3
ffff800000201aae: cc                   	int3
ffff800000201aaf: cc                   	int3
ffff800000201ab0: cc                   	int3
ffff800000201ab1: cc                   	int3
ffff800000201ab2: cc                   	int3
ffff800000201ab3: cc                   	int3
ffff800000201ab4: cc                   	int3
ffff800000201ab5: cc                   	int3
ffff800000201ab6: cc                   	int3
ffff800000201ab7: cc                   	int3
ffff800000201ab8: cc                   	int3
ffff800000201ab9: cc                   	int3
ffff800000201aba: cc                   	int3
ffff800000201abb: cc                   	int3
ffff800000201abc: cc                   	int3
ffff800000201abd: cc                   	int3
ffff800000201abe: cc                   	int3
ffff800000201abf: cc                   	int3
ffff800000201ac0: cc                   	int3
ffff800000201ac1: cc                   	int3
ffff800000201ac2: cc                   	int3
ffff800000201ac3: cc                   	int3
ffff800000201ac4: cc                   	int3
ffff800000201ac5: cc                   	int3
ffff800000201ac6: cc                   	int3
ffff800000201ac7: cc                   	int3
ffff800000201ac8: cc                   	int3
ffff800000201ac9: cc                   	int3
ffff800000201aca: cc                   	int3
ffff800000201acb: cc                   	int3
ffff800000201acc: cc                   	int3
ffff800000201acd: cc                   	int3
ffff800000201ace: cc                   	int3
ffff800000201acf: cc                   	int3
ffff800000201ad0: cc                   	int3
ffff800000201ad1: cc                   	int3
ffff800000201ad2: cc                   	int3
ffff800000201ad3: cc                   	int3
ffff800000201ad4: cc                   	int3
ffff800000201ad5: cc                   	int3
ffff800000201ad6: cc                   	int3
ffff800000201ad7: cc                   	int3
ffff800000201ad8: cc                   	int3
ffff800000201ad9: cc                   	int3
ffff800000201ada: cc                   	int3
ffff800000201adb: cc                   	int3
ffff800000201adc: cc                   	int3
ffff800000201add: cc                   	int3
ffff800000201ade: cc                   	int3
ffff800000201adf: cc                   	int3
ffff800000201ae0: cc                   	int3
ffff800000201ae1: cc                   	int3
ffff800000201ae2: cc                   	int3
ffff800000201ae3: cc                   	int3
ffff800000201ae4: cc                   	int3
ffff800000201ae5: cc                   	int3
ffff800000201ae6: cc                   	int3
ffff800000201ae7: cc                   	int3
ffff800000201ae8: cc                   	int3
ffff800000201ae9: cc                   	int3
ffff800000201aea: cc                   	int3
ffff800000201aeb: cc                   	int3
ffff800000201aec: cc                   	int3
ffff800000201aed: cc                   	int3
ffff800000201aee: cc                   	int3
ffff800000201aef: cc                   	int3
ffff800000201af0: cc                   	int3
ffff800000201af1: cc                   	int3
ffff800000201af2: cc                   	int3
ffff800000201af3: cc                   	int3
ffff800000201af4: cc                   	int3
ffff800000201af5: cc                   	int3
ffff800000201af6: cc                   	int3
ffff800000201af7: cc                   	int3
ffff800000201af8: cc                   	int3
ffff800000201af9: cc                   	int3
ffff800000201afa: cc                   	int3
ffff800000201afb: cc                   	int3
ffff800000201afc: cc                   	int3
ffff800000201afd: cc                   	int3
ffff800000201afe: cc                   	int3
ffff800000201aff: cc                   	int3
ffff800000201b00: cc                   	int3
ffff800000201b01: cc                   	int3
ffff800000201b02: cc                   	int3
ffff800000201b03: cc                   	int3
ffff800000201b04: cc                   	int3
ffff800000201b05: cc                   	int3
ffff800000201b06: cc                   	int3
ffff800000201b07: cc                   	int3
ffff800000201b08: cc                   	int3
ffff800000201b09: cc                   	int3
ffff800000201b0a: cc                   	int3
ffff800000201b0b: cc                   	int3
ffff800000201b0c: cc                   	int3
ffff800000201b0d: cc                   	int3
ffff800000201b0e: cc                   	int3
ffff800000201b0f: cc                   	int3
ffff800000201b10: cc                   	int3
ffff800000201b11: cc                   	int3
ffff800000201b12: cc                   	int3
ffff800000201b13: cc                   	int3
ffff800000201b14: cc                   	int3
ffff800000201b15: cc                   	int3
ffff800000201b16: cc                   	int3
ffff800000201b17: cc                   	int3
ffff800000201b18: cc                   	int3
ffff800000201b19: cc                   	int3
ffff800000201b1a: cc                   	int3
ffff800000201b1b: cc                   	int3
ffff800000201b1c: cc                   	int3
ffff800000201b1d: cc                   	int3
ffff800000201b1e: cc                   	int3
ffff800000201b1f: cc                   	int3
ffff800000201b20: cc                   	int3
ffff800000201b21: cc                   	int3
ffff800000201b22: cc                   	int3
ffff800000201b23: cc                   	int3
ffff800000201b24: cc                   	int3
ffff800000201b25: cc                   	int3
ffff800000201b26: cc                   	int3
ffff800000201b27: cc                   	int3
ffff800000201b28: cc                   	int3
ffff800000201b29: cc                   	int3
ffff800000201b2a: cc                   	int3
ffff800000201b2b: cc                   	int3
ffff800000201b2c: cc                   	int3
ffff800000201b2d: cc                   	int3
ffff800000201b2e: cc                   	int3
ffff800000201b2f: cc                   	int3
ffff800000201b30: cc                   	int3
ffff800000201b31: cc                   	int3
ffff800000201b32: cc                   	int3
ffff800000201b33: cc                   	int3
ffff800000201b34: cc                   	int3
ffff800000201b35: cc                   	int3
ffff800000201b36: cc                   	int3
ffff800000201b37: cc                   	int3
ffff800000201b38: cc                   	int3
ffff800000201b39: cc                   	int3
ffff800000201b3a: cc                   	int3
ffff800000201b3b: cc                   	int3
ffff800000201b3c: cc                   	int3
ffff800000201b3d: cc                   	int3
ffff800000201b3e: cc                   	int3
ffff800000201b3f: cc                   	int3
ffff800000201b40: cc                   	int3
ffff800000201b41: cc                   	int3
ffff800000201b42: cc                   	int3
ffff800000201b43: cc                   	int3
ffff800000201b44: cc                   	int3
ffff800000201b45: cc                   	int3
ffff800000201b46: cc                   	int3
ffff800000201b47: cc                   	int3
ffff800000201b48: cc                   	int3
ffff800000201b49: cc                   	int3
ffff800000201b4a: cc                   	int3
ffff800000201b4b: cc                   	int3
ffff800000201b4c: cc                   	int3
ffff800000201b4d: cc                   	int3
ffff800000201b4e: cc                   	int3
ffff800000201b4f: cc                   	int3
ffff800000201b50: cc                   	int3
ffff800000201b51: cc                   	int3
ffff800000201b52: cc                   	int3
ffff800000201b53: cc                   	int3
ffff800000201b54: cc                   	int3
ffff800000201b55: cc                   	int3
ffff800000201b56: cc                   	int3
ffff800000201b57: cc                   	int3
ffff800000201b58: cc                   	int3
ffff800000201b59: cc                   	int3
ffff800000201b5a: cc                   	int3
ffff800000201b5b: cc                   	int3
ffff800000201b5c: cc                   	int3
ffff800000201b5d: cc                   	int3
ffff800000201b5e: cc                   	int3
ffff800000201b5f: cc                   	int3
ffff800000201b60: cc                   	int3
ffff800000201b61: cc                   	int3
ffff800000201b62: cc                   	int3
ffff800000201b63: cc                   	int3
ffff800000201b64: cc                   	int3
ffff800000201b65: cc                   	int3
ffff800000201b66: cc                   	int3
ffff800000201b67: cc                   	int3
ffff800000201b68: cc                   	int3
ffff800000201b69: cc                   	int3
ffff800000201b6a: cc                   	int3
ffff800000201b6b: cc                   	int3
ffff800000201b6c: cc                   	int3
ffff800000201b6d: cc                   	int3
ffff800000201b6e: cc                   	int3
ffff800000201b6f: cc                   	int3
ffff800000201b70: cc                   	int3
ffff800000201b71: cc                   	int3
ffff800000201b72: cc                   	int3
ffff800000201b73: cc                   	int3
ffff800000201b74: cc                   	int3
ffff800000201b75: cc                   	int3
ffff800000201b76: cc                   	int3
ffff800000201b77: cc                   	int3
ffff800000201b78: cc                   	int3
ffff800000201b79: cc                   	int3
ffff800000201b7a: cc                   	int3
ffff800000201b7b: cc                   	int3
ffff800000201b7c: cc                   	int3
ffff800000201b7d: cc                   	int3
ffff800000201b7e: cc                   	int3
ffff800000201b7f: cc                   	int3
ffff800000201b80: cc                   	int3
ffff800000201b81: cc                   	int3
ffff800000201b82: cc                   	int3
ffff800000201b83: cc                   	int3
ffff800000201b84: cc                   	int3
ffff800000201b85: cc                   	int3
ffff800000201b86: cc                   	int3
ffff800000201b87: cc                   	int3
ffff800000201b88: cc                   	int3
ffff800000201b89: cc                   	int3
ffff800000201b8a: cc                   	int3
ffff800000201b8b: cc                   	int3
ffff800000201b8c: cc                   	int3
ffff800000201b8d: cc                   	int3
ffff800000201b8e: cc                   	int3
ffff800000201b8f: cc                   	int3
ffff800000201b90: cc                   	int3
ffff800000201b91: cc                   	int3
ffff800000201b92: cc                   	int3
ffff800000201b93: cc                   	int3
ffff800000201b94: cc                   	int3
ffff800000201b95: cc                   	int3
ffff800000201b96: cc                   	int3
ffff800000201b97: cc                   	int3
ffff800000201b98: cc                   	int3
ffff800000201b99: cc                   	int3
ffff800000201b9a: cc                   	int3
ffff800000201b9b: cc                   	int3
ffff800000201b9c: cc                   	int3
ffff800000201b9d: cc                   	int3
ffff800000201b9e: cc                   	int3
ffff800000201b9f: cc                   	int3
ffff800000201ba0: cc                   	int3
ffff800000201ba1: cc                   	int3
ffff800000201ba2: cc                   	int3
ffff800000201ba3: cc                   	int3
ffff800000201ba4: cc                   	int3
ffff800000201ba5: cc                   	int3
ffff800000201ba6: cc                   	int3
ffff800000201ba7: cc                   	int3
ffff800000201ba8: cc                   	int3
ffff800000201ba9: cc                   	int3
ffff800000201baa: cc                   	int3
ffff800000201bab: cc                   	int3
ffff800000201bac: cc                   	int3
ffff800000201bad: cc                   	int3
ffff800000201bae: cc                   	int3
ffff800000201baf: cc                   	int3
ffff800000201bb0: cc                   	int3
ffff800000201bb1: cc                   	int3
ffff800000201bb2: cc                   	int3
ffff800000201bb3: cc                   	int3
ffff800000201bb4: cc                   	int3
ffff800000201bb5: cc                   	int3
ffff800000201bb6: cc                   	int3
ffff800000201bb7: cc                   	int3
ffff800000201bb8: cc                   	int3
ffff800000201bb9: cc                   	int3
ffff800000201bba: cc                   	int3
ffff800000201bbb: cc                   	int3
ffff800000201bbc: cc                   	int3
ffff800000201bbd: cc                   	int3
ffff800000201bbe: cc                   	int3
ffff800000201bbf: cc                   	int3
ffff800000201bc0: cc                   	int3
ffff800000201bc1: cc                   	int3
ffff800000201bc2: cc                   	int3
ffff800000201bc3: cc                   	int3
ffff800000201bc4: cc                   	int3
ffff800000201bc5: cc                   	int3
ffff800000201bc6: cc                   	int3
ffff800000201bc7: cc                   	int3
ffff800000201bc8: cc                   	int3
ffff800000201bc9: cc                   	int3
ffff800000201bca: cc                   	int3
ffff800000201bcb: cc                   	int3
ffff800000201bcc: cc                   	int3
ffff800000201bcd: cc                   	int3
ffff800000201bce: cc                   	int3
ffff800000201bcf: cc                   	int3
ffff800000201bd0: cc                   	int3
ffff800000201bd1: cc                   	int3
ffff800000201bd2: cc                   	int3
ffff800000201bd3: cc                   	int3
ffff800000201bd4: cc                   	int3
ffff800000201bd5: cc                   	int3
ffff800000201bd6: cc                   	int3
ffff800000201bd7: cc                   	int3
ffff800000201bd8: cc                   	int3
ffff800000201bd9: cc                   	int3
ffff800000201bda: cc                   	int3
ffff800000201bdb: cc                   	int3
ffff800000201bdc: cc                   	int3
ffff800000201bdd: cc                   	int3
ffff800000201bde: cc                   	int3
ffff800000201bdf: cc                   	int3
ffff800000201be0: cc                   	int3
ffff800000201be1: cc                   	int3
ffff800000201be2: cc                   	int3
ffff800000201be3: cc                   	int3
ffff800000201be4: cc                   	int3
ffff800000201be5: cc                   	int3
ffff800000201be6: cc                   	int3
ffff800000201be7: cc                   	int3
ffff800000201be8: cc                   	int3
ffff800000201be9: cc                   	int3
ffff800000201bea: cc                   	int3
ffff800000201beb: cc                   	int3
ffff800000201bec: cc                   	int3
ffff800000201bed: cc                   	int3
ffff800000201bee: cc                   	int3
ffff800000201bef: cc                   	int3
ffff800000201bf0: cc                   	int3
ffff800000201bf1: cc                   	int3
ffff800000201bf2: cc                   	int3
ffff800000201bf3: cc                   	int3
ffff800000201bf4: cc                   	int3
ffff800000201bf5: cc                   	int3
ffff800000201bf6: cc                   	int3
ffff800000201bf7: cc                   	int3
ffff800000201bf8: cc                   	int3
ffff800000201bf9: cc                   	int3
ffff800000201bfa: cc                   	int3
ffff800000201bfb: cc                   	int3
ffff800000201bfc: cc                   	int3
ffff800000201bfd: cc                   	int3
ffff800000201bfe: cc                   	int3
ffff800000201bff: cc                   	int3
ffff800000201c00: cc                   	int3
ffff800000201c01: cc                   	int3
ffff800000201c02: cc                   	int3
ffff800000201c03: cc                   	int3
ffff800000201c04: cc                   	int3
ffff800000201c05: cc                   	int3
ffff800000201c06: cc                   	int3
ffff800000201c07: cc                   	int3
ffff800000201c08: cc                   	int3
ffff800000201c09: cc                   	int3
ffff800000201c0a: cc                   	int3
ffff800000201c0b: cc                   	int3
ffff800000201c0c: cc                   	int3
ffff800000201c0d: cc                   	int3
ffff800000201c0e: cc                   	int3
ffff800000201c0f: cc                   	int3
ffff800000201c10: cc                   	int3
ffff800000201c11: cc                   	int3
ffff800000201c12: cc                   	int3
ffff800000201c13: cc                   	int3
ffff800000201c14: cc                   	int3
ffff800000201c15: cc                   	int3
ffff800000201c16: cc                   	int3
ffff800000201c17: cc                   	int3
ffff800000201c18: cc                   	int3
ffff800000201c19: cc                   	int3
ffff800000201c1a: cc                   	int3
ffff800000201c1b: cc                   	int3
ffff800000201c1c: cc                   	int3
ffff800000201c1d: cc                   	int3
ffff800000201c1e: cc                   	int3
ffff800000201c1f: cc                   	int3
ffff800000201c20: cc                   	int3
ffff800000201c21: cc                   	int3
ffff800000201c22: cc                   	int3
ffff800000201c23: cc                   	int3
ffff800000201c24: cc                   	int3
ffff800000201c25: cc                   	int3
ffff800000201c26: cc                   	int3
ffff800000201c27: cc                   	int3
ffff800000201c28: cc                   	int3
ffff800000201c29: cc                   	int3
ffff800000201c2a: cc                   	int3
ffff800000201c2b: cc                   	int3
ffff800000201c2c: cc                   	int3
ffff800000201c2d: cc                   	int3
ffff800000201c2e: cc                   	int3
ffff800000201c2f: cc                   	int3
ffff800000201c30: cc                   	int3
ffff800000201c31: cc                   	int3
ffff800000201c32: cc                   	int3
ffff800000201c33: cc                   	int3
ffff800000201c34: cc                   	int3
ffff800000201c35: cc                   	int3
ffff800000201c36: cc                   	int3
ffff800000201c37: cc                   	int3
ffff800000201c38: cc                   	int3
ffff800000201c39: cc                   	int3
ffff800000201c3a: cc                   	int3
ffff800000201c3b: cc                   	int3
ffff800000201c3c: cc                   	int3
ffff800000201c3d: cc                   	int3
ffff800000201c3e: cc                   	int3
ffff800000201c3f: cc                   	int3
ffff800000201c40: cc                   	int3
ffff800000201c41: cc                   	int3
ffff800000201c42: cc                   	int3
ffff800000201c43: cc                   	int3
ffff800000201c44: cc                   	int3
ffff800000201c45: cc                   	int3
ffff800000201c46: cc                   	int3
ffff800000201c47: cc                   	int3
ffff800000201c48: cc                   	int3
ffff800000201c49: cc                   	int3
ffff800000201c4a: cc                   	int3
ffff800000201c4b: cc                   	int3
ffff800000201c4c: cc                   	int3
ffff800000201c4d: cc                   	int3
ffff800000201c4e: cc                   	int3
ffff800000201c4f: cc                   	int3
ffff800000201c50: cc                   	int3
ffff800000201c51: cc                   	int3
ffff800000201c52: cc                   	int3
ffff800000201c53: cc                   	int3
ffff800000201c54: cc                   	int3
ffff800000201c55: cc                   	int3
ffff800000201c56: cc                   	int3
ffff800000201c57: cc                   	int3
ffff800000201c58: cc                   	int3
ffff800000201c59: cc                   	int3
ffff800000201c5a: cc                   	int3
ffff800000201c5b: cc                   	int3
ffff800000201c5c: cc                   	int3
ffff800000201c5d: cc                   	int3
ffff800000201c5e: cc                   	int3
ffff800000201c5f: cc                   	int3
ffff800000201c60: cc                   	int3
ffff800000201c61: cc                   	int3
ffff800000201c62: cc                   	int3
ffff800000201c63: cc                   	int3
ffff800000201c64: cc                   	int3
ffff800000201c65: cc                   	int3
ffff800000201c66: cc                   	int3
ffff800000201c67: cc                   	int3
ffff800000201c68: cc                   	int3
ffff800000201c69: cc                   	int3
ffff800000201c6a: cc                   	int3
ffff800000201c6b: cc                   	int3
ffff800000201c6c: cc                   	int3
ffff800000201c6d: cc                   	int3
ffff800000201c6e: cc                   	int3
ffff800000201c6f: cc                   	int3
ffff800000201c70: cc                   	int3
ffff800000201c71: cc                   	int3
ffff800000201c72: cc                   	int3
ffff800000201c73: cc                   	int3
ffff800000201c74: cc                   	int3
ffff800000201c75: cc                   	int3
ffff800000201c76: cc                   	int3
ffff800000201c77: cc                   	int3
ffff800000201c78: cc                   	int3
ffff800000201c79: cc                   	int3
ffff800000201c7a: cc                   	int3
ffff800000201c7b: cc                   	int3
ffff800000201c7c: cc                   	int3
ffff800000201c7d: cc                   	int3
ffff800000201c7e: cc                   	int3
ffff800000201c7f: cc                   	int3
ffff800000201c80: cc                   	int3
ffff800000201c81: cc                   	int3
ffff800000201c82: cc                   	int3
ffff800000201c83: cc                   	int3
ffff800000201c84: cc                   	int3
ffff800000201c85: cc                   	int3
ffff800000201c86: cc                   	int3
ffff800000201c87: cc                   	int3
ffff800000201c88: cc                   	int3
ffff800000201c89: cc                   	int3
ffff800000201c8a: cc                   	int3
ffff800000201c8b: cc                   	int3
ffff800000201c8c: cc                   	int3
ffff800000201c8d: cc                   	int3
ffff800000201c8e: cc                   	int3
ffff800000201c8f: cc                   	int3
ffff800000201c90: cc                   	int3
ffff800000201c91: cc                   	int3
ffff800000201c92: cc                   	int3
ffff800000201c93: cc                   	int3
ffff800000201c94: cc                   	int3
ffff800000201c95: cc                   	int3
ffff800000201c96: cc                   	int3
ffff800000201c97: cc                   	int3
ffff800000201c98: cc                   	int3
ffff800000201c99: cc                   	int3
ffff800000201c9a: cc                   	int3
ffff800000201c9b: cc                   	int3
ffff800000201c9c: cc                   	int3
ffff800000201c9d: cc                   	int3
ffff800000201c9e: cc                   	int3
ffff800000201c9f: cc                   	int3
ffff800000201ca0: cc                   	int3
ffff800000201ca1: cc                   	int3
ffff800000201ca2: cc                   	int3
ffff800000201ca3: cc                   	int3
ffff800000201ca4: cc                   	int3
ffff800000201ca5: cc                   	int3
ffff800000201ca6: cc                   	int3
ffff800000201ca7: cc                   	int3
ffff800000201ca8: cc                   	int3
ffff800000201ca9: cc                   	int3
ffff800000201caa: cc                   	int3
ffff800000201cab: cc                   	int3
ffff800000201cac: cc                   	int3
ffff800000201cad: cc                   	int3
ffff800000201cae: cc                   	int3
ffff800000201caf: cc                   	int3
ffff800000201cb0: cc                   	int3
ffff800000201cb1: cc                   	int3
ffff800000201cb2: cc                   	int3
ffff800000201cb3: cc                   	int3
ffff800000201cb4: cc                   	int3
ffff800000201cb5: cc                   	int3
ffff800000201cb6: cc                   	int3
ffff800000201cb7: cc                   	int3
ffff800000201cb8: cc                   	int3
ffff800000201cb9: cc                   	int3
ffff800000201cba: cc                   	int3
ffff800000201cbb: cc                   	int3
ffff800000201cbc: cc                   	int3
ffff800000201cbd: cc                   	int3
ffff800000201cbe: cc                   	int3
ffff800000201cbf: cc                   	int3
ffff800000201cc0: cc                   	int3
ffff800000201cc1: cc                   	int3
ffff800000201cc2: cc                   	int3
ffff800000201cc3: cc                   	int3
ffff800000201cc4: cc                   	int3
ffff800000201cc5: cc                   	int3
ffff800000201cc6: cc                   	int3
ffff800000201cc7: cc                   	int3
ffff800000201cc8: cc                   	int3
ffff800000201cc9: cc                   	int3
ffff800000201cca: cc                   	int3
ffff800000201ccb: cc                   	int3
ffff800000201ccc: cc                   	int3
ffff800000201ccd: cc                   	int3
ffff800000201cce: cc                   	int3
ffff800000201ccf: cc                   	int3
ffff800000201cd0: cc                   	int3
ffff800000201cd1: cc                   	int3
ffff800000201cd2: cc                   	int3
ffff800000201cd3: cc                   	int3
ffff800000201cd4: cc                   	int3
ffff800000201cd5: cc                   	int3
ffff800000201cd6: cc                   	int3
ffff800000201cd7: cc                   	int3
ffff800000201cd8: cc                   	int3
ffff800000201cd9: cc                   	int3
ffff800000201cda: cc                   	int3
ffff800000201cdb: cc                   	int3
ffff800000201cdc: cc                   	int3
ffff800000201cdd: cc                   	int3
ffff800000201cde: cc                   	int3
ffff800000201cdf: cc                   	int3
ffff800000201ce0: cc                   	int3
ffff800000201ce1: cc                   	int3
ffff800000201ce2: cc                   	int3
ffff800000201ce3: cc                   	int3
ffff800000201ce4: cc                   	int3
ffff800000201ce5: cc                   	int3
ffff800000201ce6: cc                   	int3
ffff800000201ce7: cc                   	int3
ffff800000201ce8: cc                   	int3
ffff800000201ce9: cc                   	int3
ffff800000201cea: cc                   	int3
ffff800000201ceb: cc                   	int3
ffff800000201cec: cc                   	int3
ffff800000201ced: cc                   	int3
ffff800000201cee: cc                   	int3
ffff800000201cef: cc                   	int3
ffff800000201cf0: cc                   	int3
ffff800000201cf1: cc                   	int3
ffff800000201cf2: cc                   	int3
ffff800000201cf3: cc                   	int3
ffff800000201cf4: cc                   	int3
ffff800000201cf5: cc                   	int3
ffff800000201cf6: cc                   	int3
ffff800000201cf7: cc                   	int3
ffff800000201cf8: cc                   	int3
ffff800000201cf9: cc                   	int3
ffff800000201cfa: cc                   	int3
ffff800000201cfb: cc                   	int3
ffff800000201cfc: cc                   	int3
ffff800000201cfd: cc                   	int3
ffff800000201cfe: cc                   	int3
ffff800000201cff: cc                   	int3
ffff800000201d00: cc                   	int3
ffff800000201d01: cc                   	int3
ffff800000201d02: cc                   	int3
ffff800000201d03: cc                   	int3
ffff800000201d04: cc                   	int3
ffff800000201d05: cc                   	int3
ffff800000201d06: cc                   	int3
ffff800000201d07: cc                   	int3
ffff800000201d08: cc                   	int3
ffff800000201d09: cc                   	int3
ffff800000201d0a: cc                   	int3
ffff800000201d0b: cc                   	int3
ffff800000201d0c: cc                   	int3
ffff800000201d0d: cc                   	int3
ffff800000201d0e: cc                   	int3
ffff800000201d0f: cc                   	int3
ffff800000201d10: cc                   	int3
ffff800000201d11: cc                   	int3
ffff800000201d12: cc                   	int3
ffff800000201d13: cc                   	int3
ffff800000201d14: cc                   	int3
ffff800000201d15: cc                   	int3
ffff800000201d16: cc                   	int3
ffff800000201d17: cc                   	int3
ffff800000201d18: cc                   	int3
ffff800000201d19: cc                   	int3
ffff800000201d1a: cc                   	int3
ffff800000201d1b: cc                   	int3
ffff800000201d1c: cc                   	int3
ffff800000201d1d: cc                   	int3
ffff800000201d1e: cc                   	int3
ffff800000201d1f: cc                   	int3
ffff800000201d20: cc                   	int3
ffff800000201d21: cc                   	int3
ffff800000201d22: cc                   	int3
ffff800000201d23: cc                   	int3
ffff800000201d24: cc                   	int3
ffff800000201d25: cc                   	int3
ffff800000201d26: cc                   	int3
ffff800000201d27: cc                   	int3
ffff800000201d28: cc                   	int3
ffff800000201d29: cc                   	int3
ffff800000201d2a: cc                   	int3
ffff800000201d2b: cc                   	int3
ffff800000201d2c: cc                   	int3
ffff800000201d2d: cc                   	int3
ffff800000201d2e: cc                   	int3
ffff800000201d2f: cc                   	int3
ffff800000201d30: cc                   	int3
ffff800000201d31: cc                   	int3
ffff800000201d32: cc                   	int3
ffff800000201d33: cc                   	int3
ffff800000201d34: cc                   	int3
ffff800000201d35: cc                   	int3
ffff800000201d36: cc                   	int3
ffff800000201d37: cc                   	int3
ffff800000201d38: cc                   	int3
ffff800000201d39: cc                   	int3
ffff800000201d3a: cc                   	int3
ffff800000201d3b: cc                   	int3
ffff800000201d3c: cc                   	int3
ffff800000201d3d: cc                   	int3
ffff800000201d3e: cc                   	int3
ffff800000201d3f: cc                   	int3
ffff800000201d40: cc                   	int3
ffff800000201d41: cc                   	int3
ffff800000201d42: cc                   	int3
ffff800000201d43: cc                   	int3
ffff800000201d44: cc                   	int3
ffff800000201d45: cc                   	int3
ffff800000201d46: cc                   	int3
ffff800000201d47: cc                   	int3
ffff800000201d48: cc                   	int3
ffff800000201d49: cc                   	int3
ffff800000201d4a: cc                   	int3
ffff800000201d4b: cc                   	int3
ffff800000201d4c: cc                   	int3
ffff800000201d4d: cc                   	int3
ffff800000201d4e: cc                   	int3
ffff800000201d4f: cc                   	int3
ffff800000201d50: cc                   	int3
ffff800000201d51: cc                   	int3
ffff800000201d52: cc                   	int3
ffff800000201d53: cc                   	int3
ffff800000201d54: cc                   	int3
ffff800000201d55: cc                   	int3
ffff800000201d56: cc                   	int3
ffff800000201d57: cc                   	int3
ffff800000201d58: cc                   	int3
ffff800000201d59: cc                   	int3
ffff800000201d5a: cc                   	int3
ffff800000201d5b: cc                   	int3
ffff800000201d5c: cc                   	int3
ffff800000201d5d: cc                   	int3
ffff800000201d5e: cc                   	int3
ffff800000201d5f: cc                   	int3
ffff800000201d60: cc                   	int3
ffff800000201d61: cc                   	int3
ffff800000201d62: cc                   	int3
ffff800000201d63: cc                   	int3
ffff800000201d64: cc                   	int3
ffff800000201d65: cc                   	int3
ffff800000201d66: cc                   	int3
ffff800000201d67: cc                   	int3
ffff800000201d68: cc                   	int3
ffff800000201d69: cc                   	int3
ffff800000201d6a: cc                   	int3
ffff800000201d6b: cc                   	int3
ffff800000201d6c: cc                   	int3
ffff800000201d6d: cc                   	int3
ffff800000201d6e: cc                   	int3
ffff800000201d6f: cc                   	int3
ffff800000201d70: cc                   	int3
ffff800000201d71: cc                   	int3
ffff800000201d72: cc                   	int3
ffff800000201d73: cc                   	int3
ffff800000201d74: cc                   	int3
ffff800000201d75: cc                   	int3
ffff800000201d76: cc                   	int3
ffff800000201d77: cc                   	int3
ffff800000201d78: cc                   	int3
ffff800000201d79: cc                   	int3
ffff800000201d7a: cc                   	int3
ffff800000201d7b: cc                   	int3
ffff800000201d7c: cc                   	int3
ffff800000201d7d: cc                   	int3
ffff800000201d7e: cc                   	int3
ffff800000201d7f: cc                   	int3
ffff800000201d80: cc                   	int3
ffff800000201d81: cc                   	int3
ffff800000201d82: cc                   	int3
ffff800000201d83: cc                   	int3
ffff800000201d84: cc                   	int3
ffff800000201d85: cc                   	int3
ffff800000201d86: cc                   	int3
ffff800000201d87: cc                   	int3
ffff800000201d88: cc                   	int3
ffff800000201d89: cc                   	int3
ffff800000201d8a: cc                   	int3
ffff800000201d8b: cc                   	int3
ffff800000201d8c: cc                   	int3
ffff800000201d8d: cc                   	int3
ffff800000201d8e: cc                   	int3
ffff800000201d8f: cc                   	int3
ffff800000201d90: cc                   	int3
ffff800000201d91: cc                   	int3
ffff800000201d92: cc                   	int3
ffff800000201d93: cc                   	int3
ffff800000201d94: cc                   	int3
ffff800000201d95: cc                   	int3
ffff800000201d96: cc                   	int3
ffff800000201d97: cc                   	int3
ffff800000201d98: cc                   	int3
ffff800000201d99: cc                   	int3
ffff800000201d9a: cc                   	int3
ffff800000201d9b: cc                   	int3
ffff800000201d9c: cc                   	int3
ffff800000201d9d: cc                   	int3
ffff800000201d9e: cc                   	int3
ffff800000201d9f: cc                   	int3
ffff800000201da0: cc                   	int3
ffff800000201da1: cc                   	int3
ffff800000201da2: cc                   	int3
ffff800000201da3: cc                   	int3
ffff800000201da4: cc                   	int3
ffff800000201da5: cc                   	int3
ffff800000201da6: cc                   	int3
ffff800000201da7: cc                   	int3
ffff800000201da8: cc                   	int3
ffff800000201da9: cc                   	int3
ffff800000201daa: cc                   	int3
ffff800000201dab: cc                   	int3
ffff800000201dac: cc                   	int3
ffff800000201dad: cc                   	int3
ffff800000201dae: cc                   	int3
ffff800000201daf: cc                   	int3
ffff800000201db0: cc                   	int3
ffff800000201db1: cc                   	int3
ffff800000201db2: cc                   	int3
ffff800000201db3: cc                   	int3
ffff800000201db4: cc                   	int3
ffff800000201db5: cc                   	int3
ffff800000201db6: cc                   	int3
ffff800000201db7: cc                   	int3
ffff800000201db8: cc                   	int3
ffff800000201db9: cc                   	int3
ffff800000201dba: cc                   	int3
ffff800000201dbb: cc                   	int3
ffff800000201dbc: cc                   	int3
ffff800000201dbd: cc                   	int3
ffff800000201dbe: cc                   	int3
ffff800000201dbf: cc                   	int3
ffff800000201dc0: cc                   	int3
ffff800000201dc1: cc                   	int3
ffff800000201dc2: cc                   	int3
ffff800000201dc3: cc                   	int3
ffff800000201dc4: cc                   	int3
ffff800000201dc5: cc                   	int3
ffff800000201dc6: cc                   	int3
ffff800000201dc7: cc                   	int3
ffff800000201dc8: cc                   	int3
ffff800000201dc9: cc                   	int3
ffff800000201dca: cc                   	int3
ffff800000201dcb: cc                   	int3
ffff800000201dcc: cc                   	int3
ffff800000201dcd: cc                   	int3
ffff800000201dce: cc                   	int3
ffff800000201dcf: cc                   	int3
ffff800000201dd0: cc                   	int3
ffff800000201dd1: cc                   	int3
ffff800000201dd2: cc                   	int3
ffff800000201dd3: cc                   	int3
ffff800000201dd4: cc                   	int3
ffff800000201dd5: cc                   	int3
ffff800000201dd6: cc                   	int3
ffff800000201dd7: cc                   	int3
ffff800000201dd8: cc                   	int3
ffff800000201dd9: cc                   	int3
ffff800000201dda: cc                   	int3
ffff800000201ddb: cc                   	int3
ffff800000201ddc: cc                   	int3
ffff800000201ddd: cc                   	int3
ffff800000201dde: cc                   	int3
ffff800000201ddf: cc                   	int3
ffff800000201de0: cc                   	int3
ffff800000201de1: cc                   	int3
ffff800000201de2: cc                   	int3
ffff800000201de3: cc                   	int3
ffff800000201de4: cc                   	int3
ffff800000201de5: cc                   	int3
ffff800000201de6: cc                   	int3
ffff800000201de7: cc                   	int3
ffff800000201de8: cc                   	int3
ffff800000201de9: cc                   	int3
ffff800000201dea: cc                   	int3
ffff800000201deb: cc                   	int3
ffff800000201dec: cc                   	int3
ffff800000201ded: cc                   	int3
ffff800000201dee: cc                   	int3
ffff800000201def: cc                   	int3
ffff800000201df0: cc                   	int3
ffff800000201df1: cc                   	int3
ffff800000201df2: cc                   	int3
ffff800000201df3: cc                   	int3
ffff800000201df4: cc                   	int3
ffff800000201df5: cc                   	int3
ffff800000201df6: cc                   	int3
ffff800000201df7: cc                   	int3
ffff800000201df8: cc                   	int3
ffff800000201df9: cc                   	int3
ffff800000201dfa: cc                   	int3
ffff800000201dfb: cc                   	int3
ffff800000201dfc: cc                   	int3
ffff800000201dfd: cc                   	int3
ffff800000201dfe: cc                   	int3
ffff800000201dff: cc                   	int3
ffff800000201e00: cc                   	int3
ffff800000201e01: cc                   	int3
ffff800000201e02: cc                   	int3
ffff800000201e03: cc                   	int3
ffff800000201e04: cc                   	int3
ffff800000201e05: cc                   	int3
ffff800000201e06: cc                   	int3
ffff800000201e07: cc                   	int3
ffff800000201e08: cc                   	int3
ffff800000201e09: cc                   	int3
ffff800000201e0a: cc                   	int3
ffff800000201e0b: cc                   	int3
ffff800000201e0c: cc                   	int3
ffff800000201e0d: cc                   	int3
ffff800000201e0e: cc                   	int3
ffff800000201e0f: cc                   	int3
ffff800000201e10: cc                   	int3
ffff800000201e11: cc                   	int3
ffff800000201e12: cc                   	int3
ffff800000201e13: cc                   	int3
ffff800000201e14: cc                   	int3
ffff800000201e15: cc                   	int3
ffff800000201e16: cc                   	int3
ffff800000201e17: cc                   	int3
ffff800000201e18: cc                   	int3
ffff800000201e19: cc                   	int3
ffff800000201e1a: cc                   	int3
ffff800000201e1b: cc                   	int3
ffff800000201e1c: cc                   	int3
ffff800000201e1d: cc                   	int3
ffff800000201e1e: cc                   	int3
ffff800000201e1f: cc                   	int3
ffff800000201e20: cc                   	int3
ffff800000201e21: cc                   	int3
ffff800000201e22: cc                   	int3
ffff800000201e23: cc                   	int3
ffff800000201e24: cc                   	int3
ffff800000201e25: cc                   	int3
ffff800000201e26: cc                   	int3
ffff800000201e27: cc                   	int3
ffff800000201e28: cc                   	int3
ffff800000201e29: cc                   	int3
ffff800000201e2a: cc                   	int3
ffff800000201e2b: cc                   	int3
ffff800000201e2c: cc                   	int3
ffff800000201e2d: cc                   	int3
ffff800000201e2e: cc                   	int3
ffff800000201e2f: cc                   	int3
ffff800000201e30: cc                   	int3
ffff800000201e31: cc                   	int3
ffff800000201e32: cc                   	int3
ffff800000201e33: cc                   	int3
ffff800000201e34: cc                   	int3
ffff800000201e35: cc                   	int3
ffff800000201e36: cc                   	int3
ffff800000201e37: cc                   	int3
ffff800000201e38: cc                   	int3
ffff800000201e39: cc                   	int3
ffff800000201e3a: cc                   	int3
ffff800000201e3b: cc                   	int3
ffff800000201e3c: cc                   	int3
ffff800000201e3d: cc                   	int3
ffff800000201e3e: cc                   	int3
ffff800000201e3f: cc                   	int3
ffff800000201e40: cc                   	int3
ffff800000201e41: cc                   	int3
ffff800000201e42: cc                   	int3
ffff800000201e43: cc                   	int3
ffff800000201e44: cc                   	int3
ffff800000201e45: cc                   	int3
ffff800000201e46: cc                   	int3
ffff800000201e47: cc                   	int3
ffff800000201e48: cc                   	int3
ffff800000201e49: cc                   	int3
ffff800000201e4a: cc                   	int3
ffff800000201e4b: cc                   	int3
ffff800000201e4c: cc                   	int3
ffff800000201e4d: cc                   	int3
ffff800000201e4e: cc                   	int3
ffff800000201e4f: cc                   	int3
ffff800000201e50: cc                   	int3
ffff800000201e51: cc                   	int3
ffff800000201e52: cc                   	int3
ffff800000201e53: cc                   	int3
ffff800000201e54: cc                   	int3
ffff800000201e55: cc                   	int3
ffff800000201e56: cc                   	int3
ffff800000201e57: cc                   	int3
ffff800000201e58: cc                   	int3
ffff800000201e59: cc                   	int3
ffff800000201e5a: cc                   	int3
ffff800000201e5b: cc                   	int3
ffff800000201e5c: cc                   	int3
ffff800000201e5d: cc                   	int3
ffff800000201e5e: cc                   	int3
ffff800000201e5f: cc                   	int3
ffff800000201e60: cc                   	int3
ffff800000201e61: cc                   	int3
ffff800000201e62: cc                   	int3
ffff800000201e63: cc                   	int3
ffff800000201e64: cc                   	int3
ffff800000201e65: cc                   	int3
ffff800000201e66: cc                   	int3
ffff800000201e67: cc                   	int3
ffff800000201e68: cc                   	int3
ffff800000201e69: cc                   	int3
ffff800000201e6a: cc                   	int3
ffff800000201e6b: cc                   	int3
ffff800000201e6c: cc                   	int3
ffff800000201e6d: cc                   	int3
ffff800000201e6e: cc                   	int3
ffff800000201e6f: cc                   	int3
ffff800000201e70: cc                   	int3
ffff800000201e71: cc                   	int3
ffff800000201e72: cc                   	int3
ffff800000201e73: cc                   	int3
ffff800000201e74: cc                   	int3
ffff800000201e75: cc                   	int3
ffff800000201e76: cc                   	int3
ffff800000201e77: cc                   	int3
ffff800000201e78: cc                   	int3
ffff800000201e79: cc                   	int3
ffff800000201e7a: cc                   	int3
ffff800000201e7b: cc                   	int3
ffff800000201e7c: cc                   	int3
ffff800000201e7d: cc                   	int3
ffff800000201e7e: cc                   	int3
ffff800000201e7f: cc                   	int3
ffff800000201e80: cc                   	int3
ffff800000201e81: cc                   	int3
ffff800000201e82: cc                   	int3
ffff800000201e83: cc                   	int3
ffff800000201e84: cc                   	int3
ffff800000201e85: cc                   	int3
ffff800000201e86: cc                   	int3
ffff800000201e87: cc                   	int3
ffff800000201e88: cc                   	int3
ffff800000201e89: cc                   	int3
ffff800000201e8a: cc                   	int3
ffff800000201e8b: cc                   	int3
ffff800000201e8c: cc                   	int3
ffff800000201e8d: cc                   	int3
ffff800000201e8e: cc                   	int3
ffff800000201e8f: cc                   	int3
ffff800000201e90: cc                   	int3
ffff800000201e91: cc                   	int3
ffff800000201e92: cc                   	int3
ffff800000201e93: cc                   	int3
ffff800000201e94: cc                   	int3
ffff800000201e95: cc                   	int3
ffff800000201e96: cc                   	int3
ffff800000201e97: cc                   	int3
ffff800000201e98: cc                   	int3
ffff800000201e99: cc                   	int3
ffff800000201e9a: cc                   	int3
ffff800000201e9b: cc                   	int3
ffff800000201e9c: cc                   	int3
ffff800000201e9d: cc                   	int3
ffff800000201e9e: cc                   	int3
ffff800000201e9f: cc                   	int3
ffff800000201ea0: cc                   	int3
ffff800000201ea1: cc                   	int3
ffff800000201ea2: cc                   	int3
ffff800000201ea3: cc                   	int3
ffff800000201ea4: cc                   	int3
ffff800000201ea5: cc                   	int3
ffff800000201ea6: cc                   	int3
ffff800000201ea7: cc                   	int3
ffff800000201ea8: cc                   	int3
ffff800000201ea9: cc                   	int3
ffff800000201eaa: cc                   	int3
ffff800000201eab: cc                   	int3
ffff800000201eac: cc                   	int3
ffff800000201ead: cc                   	int3
ffff800000201eae: cc                   	int3
ffff800000201eaf: cc                   	int3
ffff800000201eb0: cc                   	int3
ffff800000201eb1: cc                   	int3
ffff800000201eb2: cc                   	int3
ffff800000201eb3: cc                   	int3
ffff800000201eb4: cc                   	int3
ffff800000201eb5: cc                   	int3
ffff800000201eb6: cc                   	int3
ffff800000201eb7: cc                   	int3
ffff800000201eb8: cc                   	int3
ffff800000201eb9: cc                   	int3
ffff800000201eba: cc                   	int3
ffff800000201ebb: cc                   	int3
ffff800000201ebc: cc                   	int3
ffff800000201ebd: cc                   	int3
ffff800000201ebe: cc                   	int3
ffff800000201ebf: cc                   	int3
ffff800000201ec0: cc                   	int3
ffff800000201ec1: cc                   	int3
ffff800000201ec2: cc                   	int3
ffff800000201ec3: cc                   	int3
ffff800000201ec4: cc                   	int3
ffff800000201ec5: cc                   	int3
ffff800000201ec6: cc                   	int3
ffff800000201ec7: cc                   	int3
ffff800000201ec8: cc                   	int3
ffff800000201ec9: cc                   	int3
ffff800000201eca: cc                   	int3
ffff800000201ecb: cc                   	int3
ffff800000201ecc: cc                   	int3
ffff800000201ecd: cc                   	int3
ffff800000201ece: cc                   	int3
ffff800000201ecf: cc                   	int3
ffff800000201ed0: cc                   	int3
ffff800000201ed1: cc                   	int3
ffff800000201ed2: cc                   	int3
ffff800000201ed3: cc                   	int3
ffff800000201ed4: cc                   	int3
ffff800000201ed5: cc                   	int3
ffff800000201ed6: cc                   	int3
ffff800000201ed7: cc                   	int3
ffff800000201ed8: cc                   	int3
ffff800000201ed9: cc                   	int3
ffff800000201eda: cc                   	int3
ffff800000201edb: cc                   	int3
ffff800000201edc: cc                   	int3
ffff800000201edd: cc                   	int3
ffff800000201ede: cc                   	int3
ffff800000201edf: cc                   	int3
ffff800000201ee0: cc                   	int3
ffff800000201ee1: cc                   	int3
ffff800000201ee2: cc                   	int3
ffff800000201ee3: cc                   	int3
ffff800000201ee4: cc                   	int3
ffff800000201ee5: cc                   	int3
ffff800000201ee6: cc                   	int3
ffff800000201ee7: cc                   	int3
ffff800000201ee8: cc                   	int3
ffff800000201ee9: cc                   	int3
ffff800000201eea: cc                   	int3
ffff800000201eeb: cc                   	int3
ffff800000201eec: cc                   	int3
ffff800000201eed: cc                   	int3
ffff800000201eee: cc                   	int3
ffff800000201eef: cc                   	int3
ffff800000201ef0: cc                   	int3
ffff800000201ef1: cc                   	int3
ffff800000201ef2: cc                   	int3
ffff800000201ef3: cc                   	int3
ffff800000201ef4: cc                   	int3
ffff800000201ef5: cc                   	int3
ffff800000201ef6: cc                   	int3
ffff800000201ef7: cc                   	int3
ffff800000201ef8: cc                   	int3
ffff800000201ef9: cc                   	int3
ffff800000201efa: cc                   	int3
ffff800000201efb: cc                   	int3
ffff800000201efc: cc                   	int3
ffff800000201efd: cc                   	int3
ffff800000201efe: cc                   	int3
ffff800000201eff: cc                   	int3
ffff800000201f00: cc                   	int3
ffff800000201f01: cc                   	int3
ffff800000201f02: cc                   	int3
ffff800000201f03: cc                   	int3
ffff800000201f04: cc                   	int3
ffff800000201f05: cc                   	int3
ffff800000201f06: cc                   	int3
ffff800000201f07: cc                   	int3
ffff800000201f08: cc                   	int3
ffff800000201f09: cc                   	int3
ffff800000201f0a: cc                   	int3
ffff800000201f0b: cc                   	int3
ffff800000201f0c: cc                   	int3
ffff800000201f0d: cc                   	int3
ffff800000201f0e: cc                   	int3
ffff800000201f0f: cc                   	int3
ffff800000201f10: cc                   	int3
ffff800000201f11: cc                   	int3
ffff800000201f12: cc                   	int3
ffff800000201f13: cc                   	int3
ffff800000201f14: cc                   	int3
ffff800000201f15: cc                   	int3
ffff800000201f16: cc                   	int3
ffff800000201f17: cc                   	int3
ffff800000201f18: cc                   	int3
ffff800000201f19: cc                   	int3
ffff800000201f1a: cc                   	int3
ffff800000201f1b: cc                   	int3
ffff800000201f1c: cc                   	int3
ffff800000201f1d: cc                   	int3
ffff800000201f1e: cc                   	int3
ffff800000201f1f: cc                   	int3
ffff800000201f20: cc                   	int3
ffff800000201f21: cc                   	int3
ffff800000201f22: cc                   	int3
ffff800000201f23: cc                   	int3
ffff800000201f24: cc                   	int3
ffff800000201f25: cc                   	int3
ffff800000201f26: cc                   	int3
ffff800000201f27: cc                   	int3
ffff800000201f28: cc                   	int3
ffff800000201f29: cc                   	int3
ffff800000201f2a: cc                   	int3
ffff800000201f2b: cc                   	int3
ffff800000201f2c: cc                   	int3
ffff800000201f2d: cc                   	int3
ffff800000201f2e: cc                   	int3
ffff800000201f2f: cc                   	int3
ffff800000201f30: cc                   	int3
ffff800000201f31: cc                   	int3
ffff800000201f32: cc                   	int3
ffff800000201f33: cc                   	int3
ffff800000201f34: cc                   	int3
ffff800000201f35: cc                   	int3
ffff800000201f36: cc                   	int3
ffff800000201f37: cc                   	int3
ffff800000201f38: cc                   	int3
ffff800000201f39: cc                   	int3
ffff800000201f3a: cc                   	int3
ffff800000201f3b: cc                   	int3
ffff800000201f3c: cc                   	int3
ffff800000201f3d: cc                   	int3
ffff800000201f3e: cc                   	int3
ffff800000201f3f: cc                   	int3
ffff800000201f40: cc                   	int3
ffff800000201f41: cc                   	int3
ffff800000201f42: cc                   	int3
ffff800000201f43: cc                   	int3
ffff800000201f44: cc                   	int3
ffff800000201f45: cc                   	int3
ffff800000201f46: cc                   	int3
ffff800000201f47: cc                   	int3
ffff800000201f48: cc                   	int3
ffff800000201f49: cc                   	int3
ffff800000201f4a: cc                   	int3
ffff800000201f4b: cc                   	int3
ffff800000201f4c: cc                   	int3
ffff800000201f4d: cc                   	int3
ffff800000201f4e: cc                   	int3
ffff800000201f4f: cc                   	int3
ffff800000201f50: cc                   	int3
ffff800000201f51: cc                   	int3
ffff800000201f52: cc                   	int3
ffff800000201f53: cc                   	int3
ffff800000201f54: cc                   	int3
ffff800000201f55: cc                   	int3
ffff800000201f56: cc                   	int3
ffff800000201f57: cc                   	int3
ffff800000201f58: cc                   	int3
ffff800000201f59: cc                   	int3
ffff800000201f5a: cc                   	int3
ffff800000201f5b: cc                   	int3
ffff800000201f5c: cc                   	int3
ffff800000201f5d: cc                   	int3
ffff800000201f5e: cc                   	int3
ffff800000201f5f: cc                   	int3
ffff800000201f60: cc                   	int3
ffff800000201f61: cc                   	int3
ffff800000201f62: cc                   	int3
ffff800000201f63: cc                   	int3
ffff800000201f64: cc                   	int3
ffff800000201f65: cc                   	int3
ffff800000201f66: cc                   	int3
ffff800000201f67: cc                   	int3
ffff800000201f68: cc                   	int3
ffff800000201f69: cc                   	int3
ffff800000201f6a: cc                   	int3
ffff800000201f6b: cc                   	int3
ffff800000201f6c: cc                   	int3
ffff800000201f6d: cc                   	int3
ffff800000201f6e: cc                   	int3
ffff800000201f6f: cc                   	int3
ffff800000201f70: cc                   	int3
ffff800000201f71: cc                   	int3
ffff800000201f72: cc                   	int3
ffff800000201f73: cc                   	int3
ffff800000201f74: cc                   	int3
ffff800000201f75: cc                   	int3
ffff800000201f76: cc                   	int3
ffff800000201f77: cc                   	int3
ffff800000201f78: cc                   	int3
ffff800000201f79: cc                   	int3
ffff800000201f7a: cc                   	int3
ffff800000201f7b: cc                   	int3
ffff800000201f7c: cc                   	int3
ffff800000201f7d: cc                   	int3
ffff800000201f7e: cc                   	int3
ffff800000201f7f: cc                   	int3
ffff800000201f80: cc                   	int3
ffff800000201f81: cc                   	int3
ffff800000201f82: cc                   	int3
ffff800000201f83: cc                   	int3
ffff800000201f84: cc                   	int3
ffff800000201f85: cc                   	int3
ffff800000201f86: cc                   	int3
ffff800000201f87: cc                   	int3
ffff800000201f88: cc                   	int3
ffff800000201f89: cc                   	int3
ffff800000201f8a: cc                   	int3
ffff800000201f8b: cc                   	int3
ffff800000201f8c: cc                   	int3
ffff800000201f8d: cc                   	int3
ffff800000201f8e: cc                   	int3
ffff800000201f8f: cc                   	int3
ffff800000201f90: cc                   	int3
ffff800000201f91: cc                   	int3
ffff800000201f92: cc                   	int3
ffff800000201f93: cc                   	int3
ffff800000201f94: cc                   	int3
ffff800000201f95: cc                   	int3
ffff800000201f96: cc                   	int3
ffff800000201f97: cc                   	int3
ffff800000201f98: cc                   	int3
ffff800000201f99: cc                   	int3
ffff800000201f9a: cc                   	int3
ffff800000201f9b: cc                   	int3
ffff800000201f9c: cc                   	int3
ffff800000201f9d: cc                   	int3
ffff800000201f9e: cc                   	int3
ffff800000201f9f: cc                   	int3
ffff800000201fa0: cc                   	int3
ffff800000201fa1: cc                   	int3
ffff800000201fa2: cc                   	int3
ffff800000201fa3: cc                   	int3
ffff800000201fa4: cc                   	int3
ffff800000201fa5: cc                   	int3
ffff800000201fa6: cc                   	int3
ffff800000201fa7: cc                   	int3
ffff800000201fa8: cc                   	int3
ffff800000201fa9: cc                   	int3
ffff800000201faa: cc                   	int3
ffff800000201fab: cc                   	int3
ffff800000201fac: cc                   	int3
ffff800000201fad: cc                   	int3
ffff800000201fae: cc                   	int3
ffff800000201faf: cc                   	int3
ffff800000201fb0: cc                   	int3
ffff800000201fb1: cc                   	int3
ffff800000201fb2: cc                   	int3
ffff800000201fb3: cc                   	int3
ffff800000201fb4: cc                   	int3
ffff800000201fb5: cc                   	int3
ffff800000201fb6: cc                   	int3
ffff800000201fb7: cc                   	int3
ffff800000201fb8: cc                   	int3
ffff800000201fb9: cc                   	int3
ffff800000201fba: cc                   	int3
ffff800000201fbb: cc                   	int3
ffff800000201fbc: cc                   	int3
ffff800000201fbd: cc                   	int3
ffff800000201fbe: cc                   	int3
ffff800000201fbf: cc                   	int3
ffff800000201fc0: cc                   	int3
ffff800000201fc1: cc                   	int3
ffff800000201fc2: cc                   	int3
ffff800000201fc3: cc                   	int3
ffff800000201fc4: cc                   	int3
ffff800000201fc5: cc                   	int3
ffff800000201fc6: cc                   	int3
ffff800000201fc7: cc                   	int3
ffff800000201fc8: cc                   	int3
ffff800000201fc9: cc                   	int3
ffff800000201fca: cc                   	int3
ffff800000201fcb: cc                   	int3
ffff800000201fcc: cc                   	int3
ffff800000201fcd: cc                   	int3
ffff800000201fce: cc                   	int3
ffff800000201fcf: cc                   	int3
ffff800000201fd0: cc                   	int3
ffff800000201fd1: cc                   	int3
ffff800000201fd2: cc                   	int3
ffff800000201fd3: cc                   	int3
ffff800000201fd4: cc                   	int3
ffff800000201fd5: cc                   	int3
ffff800000201fd6: cc                   	int3
ffff800000201fd7: cc                   	int3
ffff800000201fd8: cc                   	int3
ffff800000201fd9: cc                   	int3
ffff800000201fda: cc                   	int3
ffff800000201fdb: cc                   	int3
ffff800000201fdc: cc                   	int3
ffff800000201fdd: cc                   	int3
ffff800000201fde: cc                   	int3
ffff800000201fdf: cc                   	int3
ffff800000201fe0: cc                   	int3
ffff800000201fe1: cc                   	int3
ffff800000201fe2: cc                   	int3
ffff800000201fe3: cc                   	int3
ffff800000201fe4: cc                   	int3
ffff800000201fe5: cc                   	int3
ffff800000201fe6: cc                   	int3
ffff800000201fe7: cc                   	int3
ffff800000201fe8: cc                   	int3
ffff800000201fe9: cc                   	int3
ffff800000201fea: cc                   	int3
ffff800000201feb: cc                   	int3
ffff800000201fec: cc                   	int3
ffff800000201fed: cc                   	int3
ffff800000201fee: cc                   	int3
ffff800000201fef: cc                   	int3
ffff800000201ff0: cc                   	int3
ffff800000201ff1: cc                   	int3
ffff800000201ff2: cc                   	int3
ffff800000201ff3: cc                   	int3
ffff800000201ff4: cc                   	int3
ffff800000201ff5: cc                   	int3
ffff800000201ff6: cc                   	int3
ffff800000201ff7: cc                   	int3
ffff800000201ff8: cc                   	int3
ffff800000201ff9: cc                   	int3
ffff800000201ffa: cc                   	int3
ffff800000201ffb: cc                   	int3
ffff800000201ffc: cc                   	int3
ffff800000201ffd: cc                   	int3
ffff800000201ffe: cc                   	int3
ffff800000201fff: cc                   	int3
