pub mod helpers {
    use std::fmt::Display;

    fn apply_escape_sequence<T: Display>(content: T, converting_integer: i32) -> String {
        const ESCAPE_FRONT: &str = "\x1b[";
        const ESCAPE_SEQUENCE_END: &str = "m";
        const ESCAPE_LAST: &str = "0";
        let original_content = content.to_string();

        return String::from(
            String::from(
                String::from(
                    String::from(
                        (String::from(
                            String::from(ESCAPE_FRONT)
                                + converting_integer.to_string().as_str(),
                        )
                        .clone()
                            + ESCAPE_SEQUENCE_END)
                            .as_str(),
                    )
                    .clone()
                        + original_content.as_str(),
                )
                .clone()
                    + ESCAPE_FRONT,
            )
            .clone()
                + ESCAPE_LAST,
        )
        .clone()
            + ESCAPE_SEQUENCE_END;
    }


    pub fn bold<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 1);
    }
    pub fn italic<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 3);
    }
    pub fn underline<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 4);
    }
    pub fn faint<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 2);
    }
    pub fn strike<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 9);
    }
    //foreground colors
    pub fn black_front<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 30);
    }
    pub fn red_front<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 31);
    }
    pub fn green_front<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 32);
    }
    pub fn yellow_front<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 33);
    }
    pub fn blue_front<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 34);
    }
    pub fn magenta_front<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 35);
    }
    pub fn cyan_front<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 36);
    }
    pub fn white_front<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 37);
    }
    //background colors
    pub fn black_back<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 40);
    }
    pub fn red_back<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 41);
    }
    pub fn green_back<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 42);
    }
    pub fn yellow_back<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 43);
    }
    pub fn blue_back<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 44);
    }
    pub fn magenta_back<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 45);
    }
    pub fn cyan_back<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 46);
    }
    pub fn white_back<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 47);
    }

    //foreground colors
    pub fn bright_black_front<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 90);
    }
    pub fn bright_red_front<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 91);
    }
    pub fn bright_green_front<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 92);
    }
    pub fn bright_yellow_front<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 93);
    }
    pub fn bright_blue_front<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 94);
    }
    pub fn bright_magenta_front<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 95);
    }
    pub fn bright_cyan_front<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 96);
    }
    pub fn bright_white_front<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 97);
    }
    //background colors
    pub fn bright_black_back<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 100);
    }
    pub fn bright_red_back<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 101);
    }
    pub fn bright_green_back<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 102);
    }
    pub fn bright_yellow_back<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 103);
    }
    pub fn bright_blue_back<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 104);
    }
    pub fn bright_magenta_back<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 105);
    }
    pub fn bright_cyan_back<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 106);
    }
    pub fn bright_white_back<T: Display>(content: T) -> String {
        return apply_escape_sequence(content, 107);
    }


}