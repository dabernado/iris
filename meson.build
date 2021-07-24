project('iris', 'c',
  version : '0.1',
  default_options : ['warning_level=3'])

capnp = dependency('c-capnproto')
dthreads = dependency('libdthread')

exe = executable('iris', src,
  install : true,
  dependencies : [capnp, dthreads])

test('basic', exe)
