# test


## test1

__mov r32 imm32__

```
#---- source ----#

BITS 32
    org 0x7c00
    mov eax, 41
    jmp 0

#----- result -----#

EAX = 0x00000029 ; 41
ECX = 0x00000000
EDX = 0x00000000
EBX = 0x00000000
ESP = 0x00007c00
EBP = 0x00000000
ESI = 0x00000000
EDI = 0x00000000
EIP = 0x00000000
```


## test2

__ModR/M__ ( `mov, add, sub, inc,`)
```
#---- source ----#

BITS 32
    org 0x7c00
    sub esp, 16
    mov ebp, esp
    mov eax, 2
    mov dword [ebp+4], 5
    add dword [ebp+4], eax
    mov esi, [ebp+4]
    inc dword [ebp+4]
    mov edi, [ebp+4]
    jmp 0

#----- result -----#

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