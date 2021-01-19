# alien-say

alien-say is a communicating alien, it's not quite up to standards, but it tries its best to speak the language of the humans.

alien-say is a cli written in rust.

## Installation

In order to install alien-say you will need have rust installed. Once you have rust installed you can install it with rust's package manager, cargo:

~~~
cargo install alien-say
~~~

If you haven't changed your install directory, you can find it under 

``` 
.cargo/bin/
```



## Usage

To let the alien say something, you simply type alien-say and it will say some random stuff!

~~~ 
$ alien-say

                     +-------------+
                     | ⋷⋺⋻⋸⋺⋳⋶⋪⋳⋶ |
                     +-------------+
                            .-.
             .-""`""-.    |(@ @)
          _/`oOoOoOoOo`\_ \ \-/
         '.-=-=-=-=-=-=-.' \/ \
     jgs   `-=.=-.-=.=-'    \ /\
              ^  ^  ^       _H_ \
~~~

If you want the alien to say some pre-defined words you can do this by following the alien-say command by a string of words!

~~~
$ alien-say "Hello, Earthling."

                     +-------------------+
                     | Hello, Earthling. |
                     +-------------------+
                            .-.
             .-""`""-.    |(@ @)
          _/`oOoOoOoOo`\_ \ \-/
         '.-=-=-=-=-=-=-.' \/ \
     jgs   `-=.=-.-=.=-'    \ /\
              ^  ^  ^       _H_ \
~~~

