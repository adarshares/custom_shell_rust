pub mod helper;
use helper::helpers;


pub trait TextStyler {
    //styles
    fn bold(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::bold(self);
    }
    fn italic(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::italic(self);
    }
    fn underline(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::underline(self);
    }
    fn faint(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::faint(self);
    }
    fn strike(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::strike(self);
    }
    //foreground colors
    fn black_front(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::black_front(self);
    }
    fn red_front(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::red_front(self);
    }
    fn green_front(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::green_front(self);
    }
    fn yellow_front(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::yellow_front(self);
    }
    fn blue_front(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::blue_front(self);
    }
    fn magenta_front(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::magenta_front(self);
    }
    fn cyan_front(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::cyan_front(self);
    }
    fn white_front(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::white_front(self);
    }

    //background colors
    fn black_back(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::black_back(self);
    }
    fn red_back(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::red_back(self);
    }
    fn green_back(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::green_back(self);
    }
    fn yellow_back(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::yellow_back(self);
    }
    fn blue_back(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::blue_back(self);
    }
    fn magenta_back(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::magenta_back(self);
    }
    fn cyan_back(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::cyan_back(self);
    }
    fn white_back(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::white_back(self);
    }

    //bright colors
    //foreground colors
    fn bright_black_front(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::bright_black_front(self);
    }
    fn bright_red_front(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::bright_red_front(self);
    }
    fn bright_green_front(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::bright_green_front(self);
    }
    fn bright_yellow_front(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::bright_yellow_front(self);
    }
    fn bright_blue_front(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::bright_blue_front(self);
    }
    fn bright_magenta_front(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::bright_magenta_front(self);
    }
    fn bright_cyan_front(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::bright_cyan_front(self);
    }
    fn bright_white_front(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::bright_white_front(self);
    }

    //background colors
    fn bright_black_back(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::bright_black_back(self);
    }
    fn bright_red_back(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::bright_red_back(self);
    }
    fn bright_green_back(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::bright_green_back(self);
    }
    fn bright_yellow_back(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::bright_yellow_back(self);
    }
    fn bright_blue_back(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::bright_blue_back(self);
    }
    fn bright_magenta_back(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::bright_magenta_back(self);
    }
    fn bright_cyan_back(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::bright_cyan_back(self);
    }
    fn bright_white_back(&self) -> String
    where
        Self: std::fmt::Display,
    {
        return helpers::bright_white_back(self);
    }
}

impl TextStyler for str{}
impl TextStyler for String{}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}","random string".red_front().underline().strike());
    }
}
