local c = ::foo.bar.baz;
local d = foo::bar.baz;



local g = foo.bar().baz;

local l = foo.bar().baz.qux().quux.corge();

local m = this.getStuff(1).getMoreStuff(2);

local n = blabla.stuff.xx.here.xx;

local o = (a() + 5) * 2 / foo.bar() - c.b.d.e;

local p = foo.bar()++ >= c.b.d.e;
