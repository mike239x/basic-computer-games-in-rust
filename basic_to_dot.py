#!/usr/bin/env python3

# cat code.bas | ./basic_to_dot.py | dot -Tpng -o code.png && feh code.png

lines = []
flow = []
loop_heads = {}
sub_heads = []
last_label = None
last_flow_label = None

while True:
    try:
        line = input().strip()
    except EOFError:
        break
    i = line.find(' ')
    label = line[:i]
    line = line[i+1:]
    new_label = label
    new_flow_label = None
    if line.startswith('GOTO'):
        to = line[5:]
        flow.append((label, to, None))
        new_label = None
    elif line.startswith('IF'):
        i = line.find('THEN')
        to = line[i+5:]
        flow.append((label, to, 'then'))
        new_flow_label = 'else'
        line = line[:i-1]
    elif line.startswith('FOR'):
        x = line
        var = x[x.find(' ')+1:x.find('=')]
        if var in loop_heads:
            assert False, 'repeating var name in for loops'
        loop_heads[var] = label
    elif line.startswith('NEXT'):
        x = line
        var = x[x.find(' ')+1:]
        if var not in loop_heads:
            assert False, f'for loop for {var} variable not found'
        flow.append((label, loop_heads[var], None))
        new_label = loop_heads[var]
        new_flow_label = 'end'
        del loop_heads[var]
    #  elif line.startswith('GOSUB'):
    #      pass
    elif line.startswith('RETURN'):
        new_label = None
    else:
        pass
    if last_label != None:
        flow.append((last_label, label, last_flow_label))
    last_label = new_label
    last_flow_label = new_flow_label
    lines.append((label, line))

print('digraph {')
for lab,line in lines:
    l = line.replace('"', "'")
    print(f'    {lab}[label="{l}"];')
for fro,to,lab in flow:
    if lab:
        print(f'    {fro} -> {to}[label="{lab}"];')
    else:
        print(f'    {fro} -> {to};')
print('}')

