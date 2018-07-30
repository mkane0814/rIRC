extern crate rIRC;
extern crate futures;
extern crate irc;


use irc::client::prelude::*;


fn main() {
    let config = Config {
        nickname: Some("irc_client_test".to_owned()),
        server: Some("irc.mozilla.org".to_owned()),
        channels: Some(vec!["#rust-spam".to_owned()]),
        ..Config::default()
    };

    let mut reactor = IrcReactor::new().unwrap();
    let client = reactor.prepare_client_and_connect(&config).unwrap();
    client.identify().unwrap();


    reactor.register_client_with_handler(client.clone(), |_, message| {
        print!("{}", message);

        Ok(())
    });

    reactor.run().unwrap();
}
