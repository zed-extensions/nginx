(block
  "{" @start
  "}" @end)

(lua_block
  "{" @start
  "}" @end)

(simple_directive
  name: (directive) @start
  (#set! scope.byLine)) @end.after
