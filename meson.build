project('iris', 'c',
  version : '0.1',
  default_options : ['warning_level=3'])

capnp = dependency('c-capnproto')
threads = dependency('threads')

src = ['src/iris.c',
       'src/machine.c',
       'src/machine.h',
       'src/functions.c',
       'src/functions.h',
       'src/opcodes.h'
       ]
exe = executable('iris', src,
  install : true,
  dependencies : [capnp, threads])

test('basic', exe)
