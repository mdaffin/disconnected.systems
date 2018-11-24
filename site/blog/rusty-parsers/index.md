---
date: '2018-09-22T00:00:00Z'
description: A look at various parsers in rust
slug: rusty-parsers
tags:
- rust
- programming
- parsers
---

# Rusty Parsers

Recently I started a project where I needed to parse some text in the form of a
hardware discription language (a way of describing how logic gates are
connected on a chip). Nothing very complex as far as parsers are conserned,
quite short input and a quite simple structure - quite liekly something that I
could do by hand but I wanted to take this as an oppotinity to learn some of
rusts parsing libraries.

Note that this is an opinionated journy though trialing various parsers - it is
likely I overlooked something or miss understood how something is meant to work
but I tried my best in following the documentation and looking up examples for
things.

## First dive into Nom

After doing to research on the various rust parser libaries around nom stuck
out as one of the more popular choices around (lots of people were raving about
it on reddit and in blog posts). So that is where I started, it has some nice
basic tutorials describes to pits parsers work quite well and was reasonably
easy to get started with the demos. After a brief play around I decided this
was at least good enough for my needs and started to implement my required
parser in it.

Unfortinutally I hit a few stumbling blocks with my first approach and quickly
discovered that while it has some very nice documentation around creating
parsers and how they work and some good examples of using them the refernce
documentation was lacking in some key areas. Not terminally bad, but it caused
more friction than I was expecting. This was all componded when I tried to look
for examples about how others implemented parsers in nom and found quite a few
blog posts out there - great, some real world use cases.

But it turns out that nom version 4 is quite new and changed in some
fundermental ways that make almost all the posts I found largely useless. For
example quite a few mention the `chain` macro which while it did what I wanted
to do but alas, it is not longer supported (FYI it was seemingly replaced by
the more powerful `do_parse` macro).

It is also worth noting that nom is a streaming parser and is very good at
handling incomplete streams of data. Unfortinutally my problem is not a
streaming problem and so requires a few tweaks, essentially you need to use
`CompletedStr` instead of `&str` for your input type - which poisions all of
your parsers requiring you to change the input type of them all to
`CompletedStr` but while anoying and makes writing generic parsers over
different input tpyes harder for most applications I don't expect it to be a
big problem. It also looked like this was a trait based system that you could
implement your own container with your own custom logic to handle more advanced
use cases.

At this point I discovered a big mistake that had made - I was completely
ignoring error managment, I mean it has a whole document on it so I figured it
was not too hard to add on after I had the basicses down. I was wrong - error
managment in nom is one of its biggest weaknesses and effectivle leaves it
entirely up to you to deal with. I could not find any good examples of how to
properly deal with errors and turn them into something useful to the end user
and no examples or in the wild uses of nom seemed to be dealing with the errors
at all.

This was a big sitcking point for me - I love rusts error reporting and would
like to replicated this as best I can, or at least give some useful error
messages about where the user went wrong even if at this stage I would not be
able to match rusts amazing error reporting. Nom did not seem to help with this
but also seemed it might give you enough to deal with it on your own.

It was at this point that I wondered if nom was all it was cracked up to be, I
mean yeah it is a good fast parser that is easy to use - if you don't care
about error reporting, which I started to suspect not everyone was (or were
willing to invest a lot of time into it). So I decided to try out another
parser library first - pest seemed to be another common one so I went with
that.

## A quick look at pest

Pests documentaion looked great, a nice amount of detail describing everything
I could want to know about how to write the parsing grammer and the examples
looked like it included almost no code at all. Great I thought, but how does it
handle error? This being a major problem with nom but was answered quickly on
their main site - and they look very nice, modeled in a similar way to rusts
but without the helpful hints as to how to fix your mistakes - that is fine I
would not expect any parser library to beable to do that. There is one problem,
the main book is incomplete, missing many of the major examples and very much
is a work in progress. Thats fine, it looks like enough to work with and what
is included looks very detailed.

So I gave it a try, and was very quickly able to reimplement my parser in it in
basically no time at all - great this is exactly what I want. One big
difference with nom (apart from the error handling being taken care of for you)
is it parses into its own datastructure - an abstract syntax tree of sorts?
That is fine I though, I know a lot of parsers do this, it is just a general
version of the manual one I constructed for nom right?

This is where the documentation broke down. There is almost nothing describing
how to parse this datastructure into something actually useful. There are two
examples (yey) but they are using two completely different methods, one uses
something called _precedence climbing_ (which I assume means something if you
know about parsers in general) and the other is looping over the tokens and
manually translating them into useful values.

It felt like I would need to write a parser for the AST to get it into a useful
state, coupled with lots of `unwraps` and `unreachable` macros in the example I
really stared to sour towards pest. I really don't like the idea of this, nom
felt much more elegant being able to parse into rust datastructures that I had
defined and built with knowledge of how the program can work. This felt like it
would be error prone and without any good examples about how you should parse
its ABT I began to question this choice. Luckly I had only invested a few hours
to get to this point unlike the few days I had invested in nom.

## A fair comparason

But you cannot give up on things as soon as you encounter a problem and I
suspect that a lot of the main issues I encountered are simple due to lack of
documentaiton and good examples for best practices. I bet the libraries are not
as bad as I original thourght if they where properly documented, but I also
cannot waste too much time looking at each in lots of detail and nor do I want
o settle for one if it turns out to have a subpar handling of some aspect that
I require.

For this I needed to give each one a fair chance on a subset of my actual
problem (so that I don't need to invest more time then required to get a good
feel for each library). I set my self a challange, revisit each libarary (and
possibly a few others) and implement a complete parser for a subset of the HDL
language I wanted to parse (a single reasonably intresting line) and write a
repl and test suite to test and compair them.

## Nom

- learnt about how to handle whitespace
- error managment struggle
  - custom types
  - how errors are proergated
  - difference between error and failure
  - the problems of failure

## Pest

- Very good documentation and examples of how to write a grammer
- Grammer is very easy to read and write
-

* The book is imcomplete and missing some crutal examples of parsing the AST
* Lacks any documentation about how to parse their AST into a useable result
* Lacks any complete examples or shows best practices outside of writting grammers
* You need to parse the AST - it is too abstract (accross any program ever) which makes it almost useless for intermadary use. This results in a lot of potential places to panic in your code or some very verbose error handling (which should never happen if there is not a bug).

## Lalrpop

Wow, why did I not look at this one sooner. My first impressions are excellent,
but I have fallen down this trap before. This is the first parser library that
I am seeing for the first time (both pest and nom I had my previous attempts
before starting this project). Its documentation looks very impressive out the
box, the included book is detailed and looks complete (unlike both nom and
pest). And the first few paragraphs answer a bunch of conserns that I had with
pest namly this part:

> Note that this parse tree is not a data structure but more a visualization of
> the parse. I mean, you _can_ build up a parse tree as a data structure, but
> typically you don't want to: it is more detailed than you need.

This is why I didn't like pest, it was parsing the tree into its own
datastructure that you then had to work with. So already I am liking this one
more and will very likely drop pest from the list of viable parsers. Nom is
still in the races as it is a combinator and streaming parser which might make
it more sutable for some tasks. But I am getting ahead of myself, lets actually
try using it first before we decide.

But it has one feature that makes it very hard to express the perticular
grammer that I require - the way it tokenises the input. One _nice_ feature of
the language I am trying to parse it that is was designed to be parsable by one
perticular parsing library - some java thing that I don't know much about and
one sideeffect of this is that there are two identifiers, one for the chip name
and one for the wire name. Chip names can accept any alphanumeric sequence that
start with a alpha character and wire names follow the same pattern, but can
also accept underscores. A minor detail that I was able to express in both nom
and pest.

The first step of Lalrpop, however, is to parse the input into a stream of
tokens. It matches looks for the longest matching token for the current
position and then move on to the next (consuming whitespace between any
tokens). It does not take any context into consideration at this point which
means it relies on the ability to distinguish between any two token without any
context.

Now, it does have an entire page of documentation dedicated to this
issue, which is nice but its examples where about how to deal with simpler
problem case. But given how similar the patterns I needed were I could not
figure out a way to express that in the grammer. It would be easy to solve in
this case with some post processing, but I wanted to avoid that as it was a
very hacky solutions to the problem.

After working this problem for the better part of a day I basically gave up,
removed the constraint on the naming difference between the Chip and Wire name
for this example, sadly wrote off lalrpop as not sutible for this project
(despite it being nicer overall) and called it a day.

Then next day while I was looking at the next parser it hit me, a viable
solution. The ChipName is a strict subset of WireName so the WireName can be
expressed in the grammer as an alternitive between the ChipName regex and the
WireName regex and combined with the precidence ordering of regexs that lalrpop
supports (and that I could not get to work on their own) I was able to express
the full language specs in the grammer. Albeit at the one downside of a
confusing error message in the case of an invalid token where a WireName should
be: `Expected one of ChipName or WireName`. But that is likely to be fixible
with custom formatting of the error messages, which look to be a lot less
complicated than doing the same thing in nom.

I think this speaks to how impotant resting is to problem solving.

## Combine
