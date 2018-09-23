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

I first tried nom, being one of the more popular libraries that lots of people
seemed to rave about. I hit quite a few issues, and decided to take a step back
and look at pest, another popular library. Quite quickly I ran into some other
issues. I relised that this was not going to be as simple as I first thourght
and decided to learn a selection of different libraries out there to compare
and contrast them. I went back to nom, taking a subset of the full language and
started to learn nom in more details, push through the problems and get a
complete solution to my partital problem. Then repeate this process for pest,
and a selection of other popular or intresting parser libraries.

As part of this I am looking at erganomics over preformance, there are other
resources out there that benchmark the various libraries (and all seem to come
to different conslusions). But I want to know about the developers side, how
well are they documented? How nice are they to use? What are the best practices
of using them? How well do they handle errors? In this series I will be
attempting to answer these questions as well as gain a better understanding of
parsing in general and create some minimal - but complete - examples for each
of the libraries (as I found a lot of partial example and very few that tie
everything togeather that were not hugly complex).

## Nom

Getting started with the examples was very easy, it does a good job of
explaining how the macro parsers work and how you can chain them up to build
more complex parsers. One thing I really like about nom is being able to parse
into any datastructure. This allows you to build up your own AST which makes
use of rusts powerful type system and saftey checks.

But once you move beyond the simple examples into creating your own parsers
things get a little bit more hary. The refence documentation is no were near as
complete or detailed as the getting started documentation with some macros
almost completely lacking any good description of what they actually do or how
to use them.

Mostly all the required details are there - but you have to learn to
interperate them which takes a bit of time. You should defently take some time
to read and digest the documentation and what all the parsers are supose to do
before getting started. But once you have things are not too bad.

There is a nice long list of projects using nom - which is mostly useless for
refence. A lot of what is listed uses older version (quite often v1) and the
same goes for most thrid part blog posts and guides about it. As such, always
check which version of nom some resource is using before you invest too much
time in reading it. I wasted quite a few hours reading out dated guides that
only confused me when they didn't work as written. At this time of writing this
included almost all thrid party documentation on the matter - though the
internal docs are consistant and upto date.

Version 4 switch from its own error type to the rusts Result type and encodes
various different situations into its error type. Nom is a streaming parser -
that means it must deal with partial input (when htere is not enough data to
complete a parser). It acutally handles this very well and returns an
`Err(Incomplete(uszie))` in this situation, you simply then obtain more data
from your source into your buffers and send it back to the parser.

But the rest of error management is complex, confusing and hard to get your
head around - let along figuring out how to use it. There are basicly no
examples on how to handle errors or report them to the user. There is a whole
documentation page on error managment - that does not do a great job at
explaining things, it took me a few reads and quite a bit of trial and error to
figure things out. I think I will leave the details of this to a future post as
it would take too long to describe everything about them here but in short most
of the time I spent on learning nom was how to deal with its errors and parse
them into some semi sane message for the end user.

Whitespace //todo

My last major consern is stability, it is already on version 4 which had some
large breaking changes from the previous version and this version still feels
incomplete - notabily around the error managment side. I suspect that at
somepoint more breaking changes will happen though I do have confidence that
this will result in a new major version. I don't fell the libaray is really at
1.0 quaility yet - though it is widly used for a number of tasks I could not
find many examples of proper error handling that also used arecent version of
nom.

## Pest

- Very good documentation and examples of how to write a grammer
- Grammer is very easy to read and write
-

* The book is imcomplete and missing some crutal examples of parsing the AST
* Lacks any documentation about how to parse their AST into a useable result
* Lacks any complete examples or shows best practices outside of writting grammers
* You need to parse the AST - it is too abstract (accross any program ever) which makes it almost useless for intermadary use. This results in a lot of potential places to panic in your code or some very verbose error handling (which should never happen if there is not a bug).

## Lalrpop

## Combine
