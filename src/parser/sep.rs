
named!(
    pub space(&str) -> &str,
    is_a_s!(" \t")
);

named!(
    pub newline(&str) -> &str,
    recognize!(tuple!(
        opt!(space),
        fold_many1!(
            tuple!(
                alt_complete!(tag!("\r\n") | tag!("\n")), 
                opt!(space)
            ),
            (), |_, _| ()
        )
    ))
);
