use git2::{
    build::{CheckoutBuilder, RepoBuilder},
    Cred,
};
use git2::{FetchOptions, Progress, RemoteCallbacks};
use iced::{button, Align, Button, Column, Element, Sandbox, Settings, Text};
use std::path::Path;

pub fn main() -> iced::Result {
    //GitUI::run(Settings::default())
    //let arg_url = "https://github.com/TeXitoi/structopt.git";
    let arg_url = "https://github.com/Zizico2/bora-coruche.git";
    let path = "/home/bernardo/testssstt/bora-c";
    let mut cb = RemoteCallbacks::new();
    cb.credentials(|_url, username_from_url, _allowed_types| {
        Cred::userpass_plaintext("berna.agua@gmail.com", "Berna@agua1999")
    });
    let mut fo = FetchOptions::new();
    fo.remote_callbacks(cb);
    let repo = match RepoBuilder::new()
        .fetch_options(fo)
        .clone(&arg_url, Path::new(&path))
    {
        Ok(repo) => repo,
        Err(e) => {
            panic!("{}", e.message());
        }
    };
    Ok(())
}

#[derive(Default)]
struct GitUI {
    active_repo: Option<Box<Path>>,
    git_clone_button: button::State,
    git_add_all_button: button::State,
    git_commit_button: button::State,
    git_push_button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    GitClonePressed,
    GitAddAllPressed,
    GitCommitPressed,
    GitPushPressed,
}

impl Sandbox for GitUI {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        /*
        match message {

        }
        */
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            //.push(Text::new(self.active_repo))
            .push(
                Button::new(&mut self.git_clone_button, Text::new("Clone"))
                    .on_press(Message::GitClonePressed),
            )
            .push(
                Button::new(&mut self.git_add_all_button, Text::new("Add All"))
                    .on_press(Message::GitAddAllPressed),
            )
            .push(
                Button::new(&mut self.git_commit_button, Text::new("Commit"))
                    .on_press(Message::GitCommitPressed),
            )
            .push(
                Button::new(&mut self.git_push_button, Text::new("Push"))
                    .on_press(Message::GitPushPressed),
            )
            .into()
    }
}
