function foo(arg1,arg2,arg3)
{
  ::print(arg1 + arg2 + arg3);

  return arg1 + arg2 + arg3;
}


this.invoke(function (arg1,arg2,arg3)
{
  ::print(arg1 + arg2 + arg3);
})
