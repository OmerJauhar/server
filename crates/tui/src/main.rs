use cursive::{Cursive,menu,event}; 
use cursive::view::{Nameable, Resizable, Margins};
use cursive::theme::{BorderStyle, Palette};
use cursive::traits::With;
use cursive::views::{Dialog, EditView};
fn main() {
    let mut siv = cursive::default();
    siv.set_theme(cursive::theme::Theme {
        shadow: true,
        borders: BorderStyle::Outset,
        palette: Palette::retro().with(|palette| {
            use cursive::theme::BaseColor::*;
            {
                use cursive::theme::PaletteColor::*;
                palette[Background] = Black.dark();
                palette[View] = Black.dark();
                palette[Primary] = White.dark();
                palette[TitlePrimary] = Blue.light();
                palette[Secondary] = Blue.light();
                palette[Highlight] = Blue.dark();
            }
            {
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
    let on_enter = |s: &mut Cursive| {
        let mut _email = String::from("");
        let mut _password = String::from("");
        if let Some(content) = s.call_on_name("name", |v: &mut EditView| {
            v.get_content()
        }) {
            if content.is_empty()
            {
                s.add_layer(Dialog::text(format!(" Enter an Email"))
            .title("Error")
            .button("Back", |s| {s.pop_layer();})
            .padding(Margins::lrtb(1, 2, 0,0)));
            }
            else  {
                _email = content.to_string(); 
                s.pop_layer();
                password_screen(s);
            }  
        }
        else if let Some(content1) = s.call_on_name("password", |v: &mut EditView| {
            v.get_content()
        }) {
            if content1.is_empty()
            {
                
                s.add_layer(Dialog::text(format!(" Enter password"))
            .title("Error")
            .button("Back", |s| {s.pop_layer();})
            .padding(Margins::lrtb(1, 2, 0,0)));
            }
            else  {
                _password = content1.to_string(); 
                s.pop_layer();
                s.quit();
                println!("The value of the email and password are {} {}",_email,_password);
            }  
        }

    };
    siv.add_global_callback(cursive::event::Key::Enter, on_enter);
    let login_layer = cursive::views::Dialog::text("text").title("Enter your Email")
    .padding_lrtb(1, 1, 1, 0)
    .content(EditView::new().with_name("name").fixed_width(20))
    .button("Exit", |s| s.quit())
    .button("Login", password_screen);
    let linear_layout = cursive::views::LinearLayout::vertical()
    .child(cursive::views::Dialog::text("OJ's Server").padding_lrtb(6,0,0,0))
    .child(login_layer);
    // .child(options);
    siv.add_layer(linear_layout);
    siv.run();
}

fn email_screen(s: &mut Cursive)
{
    let login_layer = cursive::views::Dialog::text("text").title("Enter your Email")
    .padding_lrtb(1, 1, 1, 0)
    .content(EditView::new().with_name("name").fixed_width(20))
    .button("Exit", |s| s.quit())
    .button("Login", password_screen);
    let linear_layout = cursive::views::LinearLayout::vertical()
    .child(cursive::views::Dialog::text("OJ's Server").padding_lrtb(6,0,0,0))
    .child(login_layer);
    // .child(options);
    s.pop_layer();
    s.add_layer(linear_layout);
}

fn password_screen(s: &mut Cursive) {
    s.pop_layer();
    // PASSWORD_FLAG = trsue ; 
    let password_screen = cursive::views::Dialog::new()
    .title("Enter your Password")
    .padding_lrtb(1, 1, 1, 0)
    .content(EditView::new().with_name("password").fixed_width(20))
    .button("Back", email_screen)
    .button("Login", password_screen);
    let linear_layout = cursive::views::LinearLayout::vertical()
    .child(cursive::views::Dialog::text("OJ's Server").padding_lrtb(6,0,0,0))
        .child(password_screen);
    s.add_layer(linear_layout);
}