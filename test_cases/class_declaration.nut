{

}


class Foo {	
  constructor(a1, a2) {
    this.a1 = a1;
    this.a2 = a2;
  }
  a1 = 0;
  a2 = 0;
}
class Bar extends Foo {	
  constructor() {
    base.constructor(0, 0);
  }
  function foo() {
    return this.a1 + this.a2;
  }
  function bar() {
    return this.a1 + this.a2;
  }
}
