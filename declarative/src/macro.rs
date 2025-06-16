///
/// Declarative Macro providing Scala like For Comprehension
///

#[macro_export]
macro_rules! foreach {
    ($a:ident <- ($e:expr) yield $result:expr) => {{
        $e.map(move |$a| $result)
    }};
    ($a:ident <- ($e:expr) if ($cond:expr) yield $result:expr) => {{
        $e.filter(move |&$a| $cond)
          .map(move |$a| $result)
    }};
    ($a:ident <- ($e:expr) if ($cond:expr) $($r:tt)+) => {{
        $e.filter(move |&$a| $cond)
          .map(move |$a| foreach!($($r)+)).flatten()
    }};
    ($a:ident <- ($e:expr) $($r:tt)+) => {{
        $e.map(move |$a| foreach!($($r)+)).flatten()
    }};
}
