<?hh

type UNSAFE_TYPE_HH_FIXME_<T> = T;

/* HH_FIXME[4101] Please add the expected type parameters */
type UNSAFE_TYPE_HH_FIXME = UNSAFE_TYPE_HH_FIXME_;

abstract final class HH_FIXME {
  const type MISSING_TYPE = UNSAFE_TYPE_HH_FIXME;
}

class C {
  public async function fa(int $x): Awaitable<mixed> {
    return 1;
  }

  public async function fb(int $x): Awaitable<dynamic> {
    return "string";
  }

  public async function fc(int $x): Awaitable<UNSAFE_TYPE_HH_FIXME> {
    return 1.0;
  }

  public async function fd(int $x): Awaitable<HH_FIXME::MISSING_TYPE>  {
    return 1;
  }
}
