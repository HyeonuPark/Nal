
macro_rules! setup_mapto {
    ($mapto:ident, $orig:expr, $env:expr) => (
        macro_rules! $mapto {
            ($pattern:pat => $target:expr) => (
                $orig.clone().map(|orig| match **orig {
                    $pattern => $target,
                    _ => unreachable!(),
                }).eval($env)
            );
        }
    );
    ($mapto:ident [], $orig:expr, $env:expr) => (
        macro_rules! $mapto {
            ($pattern:pat => $target:expr) => ({
                let target_vec = $orig.clone().map(|orig| match **orig {
                    $pattern => $target,
                    _ => unreachable!(),
                });

                let vec_len = target_vec.len();
                let env = $env;

                (0..vec_len)
                    .map(|idx| {
                        target_vec.clone()
                            .map(|vec| &vec[idx])
                            .eval(env)
                    })
                    .collect::<Result<Vec<_>>>()
            });
        }
    );
}
