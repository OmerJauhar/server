use std::process::Child;

use cursive::Cursive;
use cursive::backends::crossterm::crossterm::style::Color;
use cursive::theme ; 
use cursive::theme::BaseColor;
use cursive::view::{Nameable, Resizable, Finder};
use crate::BaseColor::Green;
// use cursive::views::{Dialog, TextView};
use cursive::theme::{BorderStyle, Palette};
use cursive::traits::With;
use cursive::views::{Dialog, EditView, LinearLayout, TextView};
use cursive::menu;
use cursive::event;
use cursive::align;
fn main() {
    let mut siv = cursive::default();
    siv.set_theme(cursive::theme::Theme {
        shadow: true,
        borders: BorderStyle::Outset,
        palette: Palette::retro().with(|palette| {
            use cursive::theme::BaseColor::*;
            {
                // First, override some colors from the base palette.
                use cursive::theme::PaletteColor::*;
                palette[Background] = Black.dark();
                palette[View] = Black.dark();
                palette[Primary] = White.dark();
                palette[TitlePrimary] = Blue.light();
                palette[Secondary] = Blue.light();
                palette[Highlight] = Blue.dark();
            }
            {
                // Then override some styles.
                use cursive::theme::Effect::*;
                use cursive::theme::PaletteStyle::*;
                use cursive::theme::Style;
                palette[Highlight] = Style::from(Blue.light()).combine(Bold);
            }
        }),
    });
    siv.menubar()
    .add_subtree(
        "File",
        menu::Tree::new()
            .leaf("New", |s| s.add_layer(Dialog::info("New file!")))
            .subtree(
                "Recent",
                menu::Tree::new().with(|tree| {
                    for i in 1..100 {
                        tree.add_leaf(format!("Item {}", i), |_| ())
                    }
                }),
            )
            .delimiter()
            .with(|tree| {
                for i in 1..10 {
                    tree.add_leaf(format!("Option {}", i), |_| ());
                }
            })
            .delimiter()
            .leaf("Quit", |s| s.quit()),
    )
    .add_subtree(
        "Help",
        menu::Tree::new()
            .subtree(
                "Help",
                menu::Tree::new()
                    .leaf("General", |s| {
                        s.add_layer(Dialog::info("Help message!"))
                    })
                    .leaf("Online", |s| {
                        s.add_layer(Dialog::info("Online help?"))
                    }),
            )
            .leaf("About", |s| {
                s.add_layer(Dialog::info("Contact OJ \n at p218055@pwr.nu.edu.pk"))
            }),
    );

siv.add_global_callback(event::Key::Esc, |s| s.select_menubar());
    //global callbacl for server 
    siv.add_global_callback('q', |s| s.quit());
    

    //login screen
    let login_layer = cursive::views::Dialog::text("text").title("Enter your Email")
    .padding_lrtb(1, 1, 1, 0)
    .content(EditView::new().with_name("name").fixed_width(20))
    .button("Exit", |s| s.quit())
    .button("Login", show_next);
    
    let options = cursive::views::Dialog::default()
    .padding_lrtb(0,0,0,0)
    .button("Help", |s|{
        // s.add_layer(cursive::views::HideableView(TextView::new("content")));
        s.add_fullscreen_layer(cursive::views::Dialog::info("Contact OJ \n at p218055@pwr.nu.edu.pk").button("Esc", |s| {
            s.pop_layer();
            // s.add_layer(LinearLayout);
        }))
    })
    .button("Exit", |v| v.quit());
    let linear_layout = cursive::views::LinearLayout::vertical()
    .child(cursive::views::Dialog::text("OJ's Server").padding_lrtb(6,0,0,0))
    .child(login_layer);
    // .child(options);
    siv.add_layer(linear_layout);
    siv.run();
}

fn show_next(s: &mut Cursive) {
    s.pop_layer();
    let password_Screen = cursive::views::Dialog::new()
    .title("Enter your Password")
    .padding_lrtb(1, 1, 1, 0)
    .content(EditView::new().with_name("name").fixed_width(20))
    .button("Exit", |s| s.quit())
    .button("Login", show_next);
    let linear_layout = cursive::views::LinearLayout::vertical()
    .child(cursive::views::Dialog::text("OJ's Server").padding_lrtb(6,0,0,0))
        .child(password_Screen);
    s.add_layer(linear_layout);
    // s.add_layer(Dialog::text("meow")
    //     .title("Question 1")
    //     .button("Yes!", |s| show_answer(s, "I knew it! Well done!"))
    //     .button("No!", |s| show_answer(s, "I knew you couldn't be trusted!"))
    //     .button("Uh?", |s| s.add_layer(Dialog::info("Try again!"))));
}
// fn show_main(s: &mut Cursive) {
//     s.pop_layer();
//     let main_Screen = cursive::views::Dialog::new{};
//     // s.add_layer(Dialog::text("meow")
//     //     .title("Question 1")
//     //     .button("Yes!", |s| show_answer(s, "I knew it! Well done!"))
//     //     .button("No!", |s| show_answer(s, "I knew you couldn't be trusted!"))
//     //     .button("Uh?", |s| s.add_layer(Dialog::info("Try again!"))));
// }

fn show_answer(s: &mut Cursive, msg: &str) {
    s.pop_layer();
    s.add_layer(Dialog::text(msg)
        .title("Results")
        .button("Finish", |s| s.quit()));
}
