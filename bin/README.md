# test


## test1

__mov r32 imm32__

```
#---- asm ---#

00000000  B829000000        mov eax,0x29
00000005  BB20000000        mov ebx,0x20
0000000A  B9FF000000        mov ecx,0xff
0000000F  E9EC83FFFF        jmp 0xffff8400

#----- result -----#

EAX = 0x00000029
ECX = 0x000000ff
EDX = 0x00000000
EBX = 0x00000020
ESP = 0x00007c00
EBP = 0x00000000
ESI = 0x00000000
EDI = 0x00000000
EIP = 0x00000000
```


## test2

__ModR/M__ ( `mov, add, sub, inc,`)
```
#---- asm ----#

00000000  83EC10            sub esp,byte +0x10
00000003  89E5              mov ebp,esp
00000005  B802000000        mov eax,0x2
0000000A  C7450405000000    mov dword [ebp+0x4],0x5
00000011  014504            add [ebp+0x4],eax
00000014  8B7504            mov esi,[ebp+0x4]
00000017  FF4504            inc dword [ebp+0x4]
0000001A  8B7D04            mov edi,[ebp+0x4]
0000001D  E9DE83FFFF        jmp 0xffff8400

#----- asm -----#

EAX = 0x00000002
ECX = 0x00000000
EDX = 0x00000000
EBX = 0x00000000
ESP = 0x00007bf0
EBP = 0x00007bf0
ESI = 0x00000007
EDI = 0x00000008
EIP = 0x00000000
```

## test3
__call_func__ 
```
#---- asm----#

00000000  B8F1000000        mov eax,0xf1
00000005  BB29000000        mov ebx,0x29
0000000A  E805000000        call 0x14
0000000F  E9EC83FFFF        jmp 0xffff8400
00000014  89C1              mov ecx,eax
00000016  01D9              add ecx,ebx
00000018  C3                ret

#----- result -----#

EAX = 0x000000f1
ECX = 0x0000011a
EDX = 0x00000000
EBX = 0x00000029
ESP = 0x00007c00
EBP = 0x00000000
ESI = 0x00000000
EDI = 0x00000000
EIP = 0x00000000

```

__call func with arg__

```

#---- C ----#

int add(int a, int b)
{
    int c = 10;
    return a + b + c;
}

int main(void)
{
    return add(2, 5);
}

#---- asm ----#

$ ndisasm -b 32 <file name>

00000000  55                push ebp
00000001  89E5              mov ebp,esp
00000003  83EC10            sub esp,byte +0x10
00000006  C745FC0A000000    mov dword [ebp-0x4],0xa
0000000D  8B5508            mov edx,[ebp+0x8]
00000010  8B450C            mov eax,[ebp+0xc]
00000013  01C2              add edx,eax         ; 0x07
00000015  8B45FC            mov eax,[ebp-0x4]
00000018  01D0              add eax,edx         ; 0x11
0000001A  C9                leave
0000001B  C3                ret
0000001C  55                push ebp
0000001D  89E5              mov ebp,esp
0000001F  6A05              push byte +0x5
00000021  6A02              push byte +0x2
00000023  E8D8FFFFFF        call 0x0
00000028  83C408            add esp,byte +0x8
0000002B  C9                leave
0000002C  C3                ret

#----- result -----#

EAX = 0x00000011
ECX = 0x00000000
EDX = 0x00000007
EBX = 0x00000000
ESP = 0x00007c00
EBP = 0x00000000
ESI = 0x00000000
EDI = 0x00000000
EIP = 0x00000000

```