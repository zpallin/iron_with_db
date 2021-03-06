
[![Build Status](https://travis-ci.org/zpallin/iron_with_db.svg?branch=master)](https://travis-ci.org/zpallin/iron_with_db)

iron with db
============

purpose
-------

Iron with Db is an example, and launchpoint, for web application developers who want to write in rust but find some of the idioms of the language hard to understand. I am one of those developers, so hopefully others can follow in my footsteps to find easier times ahead.

If you are reading this, you probably already understand why I am trying to write a webapp in rust. But just in case...

Rust is a high performance, memory-safe language. I want to be able to write applications in this language (instead of alternatives like C++). Also, the language uses paradigms and syntax that is more common in newer, trendier, high level languages. Both of these aspects make the use of Rust for web development (among other things) very attractive to developers like me.


development
-----------
This project has just begin. Do not expect much.

dockerfile
----------
If you want to launch this in docker, you know for the sake of testing code, I've provided an example dockerfile.

To run it, make sure you install the latest version of docker (1.12.1 at time of writing) and do the following:

1. `docker build . -t iwdb`
2. `docker run -d -p 3000:3000 --name iwdb iwdb`

Then, just visit localhost:3000 in your browser and you'll see the "basic" example running.

related projects
----------------

* [iron\_with\_mongo](https://github.com/zpallin/iron_with_mongo) - A plugin for iron\_with\_db that has sets you up for using mongodb.
* [ironframework](https://github.com/iron/iron) - The core framework that is the basis for this one.
