// @generated by hh_manual from manual/hack/11-built-in-types/36-enum-class-label.md
// @codegen-command : buck run fbcode//hphp/hack/src/hh_manual:hh_manual extract fbcode/hphp/hack/manual/hack/
newtype MemberOf<-TEnumClass, +TType> as TType = TType;
newtype Label<-TEnumClass, TType> = mixed;

class A {}
class B extends A {}
enum class G: A {
  A X = new A();
  B Y = new B();
}
