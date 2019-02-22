# Info

My goal (not sure, if I'll would be able to acheive it) is to create an API, similar to [python-plexapi](https://github.com/pkkid/python-plexapi). Actually, to be honest, my real goal is to rewrite my [plexiglas](https://github.com/andrey-yantsen/plexiglas) project into Rust from Python :)

Any help is welcome.

As the starting point I'd like to have an easy way to bootstrap test env (especially in travis). You can check out what I've done for [python-plexapi](https://github.com/pkkid/python-plexapi/blob/master/tools/plex-bootstraptest.py) in terms of the env: there is a script which creates a new Plex Server instance in docker and populates the library with some stub media, and assigns this shiny new server to MyPlex account, if required.

# TODO

* [ ] CLI command to bootstrap new Plex server
* [ ] \+ assigning it to myplex
* [X] MyPlex access
* [ ] MobileSync
    * [ ] Read
    * [ ] Download
