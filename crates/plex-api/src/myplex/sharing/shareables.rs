use crate::{Library, MyPlex, Server};

pub enum ShareableServer<'a> {
    MachineIdentifier(&'a str),
    Server(&'a Server),
}

impl<'a> ShareableServer<'a> {
    pub fn id(&self) -> &str {
        match self {
            Self::MachineIdentifier(id) => id,
            Self::Server(srv) => srv.machine_identifier(),
        }
    }
}

pub enum ShareableLibrary<'a> {
    Library(&'a Library),
    LibraryId(&'a str),
}

impl<'a> ShareableLibrary<'a> {
    pub fn id(&self) -> &str {
        match self {
            Self::Library(library) => library.id(),
            Self::LibraryId(id) => id,
        }
    }
}

pub enum User<'a> {
    Account(&'a MyPlex),
    UsernameOrEmail(&'a str),
}

impl<'a> User<'a> {
    pub fn id(&self) -> &str {
        match self {
            Self::Account(MyPlex {
                account: Some(account),
                ..
            }) => &account.email,
            Self::Account(_) => "",
            Self::UsernameOrEmail(u) => u,
        }
    }
}
