
macro_rules! setup {
    ($name:ident, $orig:expr, $env:expr) => (
        macro_rules! $name {
            ($pattern:pat => $target:expr) => (
                $orig.clone().map(|orig| match **orig {
                    $pattern => $target,
                    _ => unreachable!(),
                }).eval($env)
            );
        }
    );

    ($name:ident[$Collect:ty], $orig:expr, $env:expr, $($deref:tt)*) => (
        macro_rules! $name {
            ($pattern:pat => $target:expr) => ({
                let env = $env;
                let len = $orig.clone().map(|orig| match $($deref)* *orig {
                        $pattern => $target,
                        _ => unreachable!(),
                    }).len();

                (0..len)
                    .map(|idx| {
                        $orig.clone()
                            .map(|orig| match $($deref)* *orig {
                                $pattern => $target.get(idx).unwrap(),
                                _ => unreachable!(),
                            })
                            .eval(env)
                    })
                    .collect::<Result<$Collect>>()
            });
        }
    );

    ($name:ident[], $orig:expr, $env:expr, $($deref:tt)*) => (
        setup!($name[Vec<_>], $orig, $env, $($deref)*);
    );
}
