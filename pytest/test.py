import csx64

def main():
    prog_name = 'demo.asm'
    prog = '''
global main

segment text
main:
    mov edi, 5
    mov esi, 4
    add edi, esi

    mov eax, edi
    ret
'''
    obj = csx64.assemble(prog_name, prog)
    objs = csx64.stdlib()
    objs.append((prog_name, obj))
    exe = csx64.link(objs, ('start', 'main'))
    emu = csx64.Emulator()
    emu.init(exe)
    emu.ots = True
    stdin, stdout, stderr = emu.setup_stdio()
    _, state = emu.execute_cycles()
    assert state == 'Terminated' and emu.get_state() == 'Terminated' and emu.get_return_value() == 9

    print(emu.rax, emu.raxi, emu.raxf, emu.flags, emu.cc_b)
    emu.rax = 12
    emu.iopl = 3
    print(emu.rax, emu.raxi, emu.raxf, emu.flags, emu.cc_be)
    emu.raxi = -14
    emu.cf = True
    emu.of = True
    print(emu.rax, emu.raxi, emu.raxf, emu.flags, emu.cc_g)
    emu.raxf = 12.43
    print(emu.rax, emu.raxi, emu.raxf, emu.flags, emu.cc_ge)

if __name__ == '__main__':
    main()