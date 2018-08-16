trait Assoc {
    type G;
    type H;

    fn kill(&self, _: &Self::G, _: &Self::G) -> Bool;
    fn you(&self, _: &Self::G) -> Vec<H>;
}


struct Temp;
struct GImp;
struct HImp;

impl Assoc for Temp {
    type G = GImp;
    type H = HImp;

    fn kill(&self, _: & <Self as Assoc>::G, _: & <Self as Assoc>::G) -> _ {
        unimplemented!()
    }

    fn you(&self, _: & <Self as Assoc>::G) -> _ {
        unimplemented!()
    }
}

fn distance<G: Assoc>(graph: &G, start: &G::N, end: &G::H) {  }

fn create() {
    let bla = Temp;
    let obj = Box::new(bla) as Box<Assoc<G=GImp, H=HImp>>;
}

struct Foo<T: ?Sized = Self> {
    f: T,
}

impl Add<i32> for Foo {
    type Output = f64;

    fn add(self, rhs: i32) -> f64 {
        // add an i32 to a Point and get an f64
    }
}

macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}