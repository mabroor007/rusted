#[allow(dead_code)]
#[derive(Debug)]
enum Scope {
    Default,
    X8,
    X6,
    X4,
    X3,
    X2,
    RedDot,
    Holographic,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Mag {
    Default,
    Quickdraw,
    Extended,
    ExtendedPlusQuickdraw,
}

// you can also attatch methods to
//   enums
impl Mag {
    pub fn log(&self) {
        println!("{:?}", self);
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum GunKind {
    AR,
    SNIPER,
    PISTOL,
    SHOTGUN,
    SMG,
}

// this is called type alias its for making
//   the name of enum short for easier referencing
//   they are both interchangeable in usage
type GunType = GunKind;

// you can define methods on alias and it will work for
//   both alias and enum
impl GunType {
    pub fn log(&self) {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
struct Gun {
    name: String,
    scope: Scope,
    mag: Mag,
    kind: GunKind,
    loaded: bool,
}

impl Gun {
    pub fn new(name: String, scope: Scope, mag: Mag, kind: GunType) -> Gun {
        Gun {
            name,
            scope,
            mag,
            kind,
            loaded: false,
        }
    }
    // not sure about this though
    pub fn reload(&mut self) -> &Gun {
        (*self).loaded = true;
        return self;
    }
}

pub fn run() {
    let mut akm = Gun::new(
        "AKM".to_string(),
        Scope::RedDot,
        Mag::ExtendedPlusQuickdraw,
        GunKind::AR,
    );

    akm.reload();

    println!("Best gun in pubg is {:?}", akm);

    // method on enum example
    let quick_draw = Mag::Quickdraw;
    quick_draw.log();

    // method on alias working on enums as well
    let sniper = GunType::SNIPER;
    let ar = GunKind::AR;

    // notice log method was implemented on GunType
    //   but it works for GunKind as well
    sniper.log();
    ar.log();
}
